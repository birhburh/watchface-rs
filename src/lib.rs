use std::{
    collections::{hash_map::Entry, HashMap},
    mem::size_of,
};

use winnow::{
    binary::{le_f32, le_u32, u8},
    stream::{Located, Location, Stream as _},
    token, PResult, Parser,
};

pub type Stream<'i> = Located<&'i [u8]>;

#[derive(Debug, PartialEq)]
struct Image {
    pixels: Vec<u8>,
    width: u32,
    height: u32,
    bits_per_pixel: u32,
    pixel_format: u32,
}

#[derive(Debug, PartialEq, Default)]
struct ImageReference {
    x: u32,
    y: u32,
    image_index: u32,
}

#[derive(Debug, PartialEq, Default)]
struct ImageRange {
    x: u32,
    y: u32,
    image_index: u32,
    image_count: u32,
}

#[derive(Debug, PartialEq, Default)]
struct Background {
    image: ImageReference,
}

#[derive(Debug, PartialEq, Default)]
struct Time {
    hours: Option<ImageRange>,
    minutes: Option<ImageRange>,
    seconds: Option<ImageRange>,
}

type Params = HashMap<u8, Vec<Param>>;

#[derive(Debug, PartialEq, Default)]
struct MiBandParams {
    background: Option<Background>,
    time: Option<Time>,
}

#[derive(Debug, PartialEq)]
enum Param {
    Bytes(Vec<u8>),
    Float(f32),
    Child(Params),
}

#[derive(Debug, PartialEq)]
enum WatchfaceBinFileParams {
    MiBand(MiBandParams),
}

#[derive(Debug, PartialEq)]
struct WatchfaceBinFile {
    parameters: WatchfaceBinFileParams,
    // images: Vec<Image>,
}

fn variable_width_value_parser<'s>(i: &mut Stream<'s>) -> PResult<(u64, usize)> {
    let mut value = 0u64;
    let bytes = token::take_till(0..10, |b| b & 0x80 != 0x80).parse_next(i)?;
    let last = u8.parse_next(i)?;
    let mut i = 0;
    for b in [bytes, &[last]].concat() {
        value |= (b as u64 & 0x7f) << (i * 7);
        i += 1;
    }

    Ok((value, i))
}

fn param_parser<'s>(i: &mut Stream<'s>) -> PResult<(u8, Param)> {
    // Read parameters info
    let (field_descriptor, _) = variable_width_value_parser(i)?;

    let key = (field_descriptor >> 3) as u8;
    let has_child = field_descriptor & 0x02 == 0x02;

    // From the second byte on is the value
    let value;
    let is_float = field_descriptor & 0x05 == 0x05;
    if is_float {
        value = Param::Float(le_f32.parse_next(i)?);
    } else {
        // variable width value
        let (temp_field_value, value_size) = variable_width_value_parser(i)?;
        let mut field_value = temp_field_value.to_le_bytes()[0..value_size].to_vec();
        field_value.reverse();

        if has_child {
            // When node has Child, field value is size of Child

            let child_size = bytes_to_usize(&field_value);
            if child_size <= 0 {
                panic!("Child size of 0 or less");
            }
            // Recursive call to read Child data
            let child = params_parser(i, child_size)?;
            value = Param::Child(child);
        } else {
            value = Param::Bytes(field_value);
        }
    }
    Ok((key, value))
}

fn params_parser<'s>(i: &mut Stream<'s>, max_size: usize) -> PResult<Params> {
    let mut prev = i.location();
    let mut bytes_left = max_size;
    let mut params = Params::from(HashMap::new());
    while bytes_left > 0 {
        match param_parser.parse_next(i) {
            Ok(o) => {
                let (key, val) = o;
                match params.entry(key) {
                    Entry::Occupied(mut occupied) => {
                        occupied.get_mut().push(val);
                    }
                    Entry::Vacant(vacant) => {
                        vacant.insert(vec![val]);
                    }
                }
                bytes_left -= i.location() - prev;
                prev = i.location();
            }
            Err(e) => return Err(e),
        }
    }
    Ok(params)
}

fn bytes_to_usize(bytes: &Vec<u8>) -> usize {
    let zeros = (0..(size_of::<usize>() - bytes.len()))
        .map(|_| 0u8)
        .collect::<Vec<_>>();
    let bytes = [bytes.clone(), zeros].concat();
    let bytes = bytes[0..size_of::<usize>()].try_into().unwrap();
    usize::from_le_bytes(bytes)
}

fn bytes_param_to_usize(param: &Param) -> usize {
    if let Param::Bytes(bytes) = param {
        bytes_to_usize(bytes)
    } else {
        unreachable!();
    }
}

fn bin_parser<'s>(mut i: Located<&[u8]>) -> PResult<WatchfaceBinFile> {
    let _signature = token::take(4usize).parse_next(&mut i)?;
    let _header = token::take(75usize).parse_next(&mut i)?;
    let _buffer_size = le_u32.parse_next(&mut i)?;
    let info_size = le_u32.parse_next(&mut i)?;
    dbg!(info_size);
    let parameter_info = params_parser(&mut i, info_size as usize)?;

    // First parameter info contains parameters size and images count
    use Param::*;

    let first_parameter = match &parameter_info.get(&1).unwrap()[0] {
        Child(child) => child,
        _ => panic!("First param should be child param"),
    };

    let parameters_size = bytes_param_to_usize(&first_parameter.get(&1).unwrap()[0]);

    let images_count = match &first_parameter.get(&2).unwrap()[0] {
        Bytes(bytes) => {
            let zeros = (0..(size_of::<usize>() - bytes.len()))
                .map(|_| 0u8)
                .collect::<Vec<_>>();
            let bytes = [bytes.clone(), zeros].concat();
            let bytes = bytes[0..size_of::<usize>()].try_into().unwrap();
            usize::from_le_bytes(bytes)
        }
        _ => panic!("First param is other params size, it should be int"),
    };

    let mut miband_params = MiBandParams {
        ..Default::default()
    };

    let params_start = i.checkpoint();

    for (key, value) in parameter_info.iter() {
        if *key == 1 {
            continue;
        }

        i.reset(&params_start);

        let subvalue = match &value.get(0).unwrap() {
            Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        let offset = bytes_param_to_usize(&subvalue.get(&1).unwrap()[0]);
        let size = bytes_param_to_usize(&subvalue.get(&2).unwrap()[0]);

        i.next_slice(offset);
        let parameter = params_parser(&mut i, size)?;

        match key {
            2 => {
                let mut background = Background {
                    ..Default::default()
                };

                for (key, value) in parameter.into_iter() {
                    match key {
                        1 => {
                            let mut image_ref = ImageReference {
                                ..Default::default()
                            };

                            let subvalue = match &value.get(0).unwrap() {
                                Child(child) => child,
                                _ => panic!("First param should be child param"),
                            };

                            for (key, value) in subvalue.into_iter() {
                                match key {
                                    1 => {
                                        image_ref.x =
                                            bytes_param_to_usize(&value.get(0).unwrap()) as u32;
                                    }
                                    2 => {
                                        image_ref.y =
                                            bytes_param_to_usize(&value.get(0).unwrap()) as u32;
                                    }
                                    3 => {
                                        image_ref.image_index =
                                            bytes_param_to_usize(&value.get(0).unwrap()) as u32;
                                    }
                                    _ => (),
                                }
                            }

                            background.image = image_ref;
                        }
                        _ => (),
                    }
                }

                miband_params.background = Some(background);
            }
            3 => {
                let mut time = Time {
                    ..Default::default()
                };

                for (key, value) in parameter.into_iter() {
                    dbg!((key, &value));
                    match key {
                        2 => {
                            let subvalue = match &value.get(0).unwrap() {
                                Child(child) => child,
                                _ => panic!("First param should be child param"),
                            };

                            for (key, value) in subvalue.into_iter() {
                                match key {
                                    1 => {
                                        dbg!(value);
                                        let subvalue = match &value.get(0).unwrap() {
                                            Child(child) => child,
                                            _ => panic!("First param should be child param"),
                                        };

                                        for (key, value) in subvalue.into_iter() {
                                            match key {
                                                2 => {
                                                    let mut image_range = ImageRange {
                                                        ..Default::default()
                                                    };

                                                    for (key, value) in subvalue.into_iter() {
                                                        dbg!((key, &value));
                                                        match key {
                                                            1 => {
                                                                image_range.x = bytes_param_to_usize(
                                                                    &value.get(0).unwrap(),
                                                                )
                                                                    as u32;
                                                            }
                                                            2 => {
                                                                image_range.y = bytes_param_to_usize(
                                                                    &value.get(0).unwrap(),
                                                                )
                                                                    as u32;
                                                            }
                                                            3 => {
                                                                image_range.image_index =
                                                                    bytes_param_to_usize(
                                                                        &value.get(0).unwrap(),
                                                                    )
                                                                        as u32;
                                                            }
                                                            4 => {
                                                                image_range.image_count =
                                                                    bytes_param_to_usize(
                                                                        &value.get(0).unwrap(),
                                                                    )
                                                                        as u32;
                                                            }
                                                            _ => (),
                                                        }
                                                    }

                                                    time.minutes = Some(image_range);
                                                }
                                                _ => (),
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    }
                }

                miband_params.time = Some(time);
            }
            _ => (),
        }
    }
    Ok(WatchfaceBinFile {
        parameters: WatchfaceBinFileParams::MiBand(miband_params),
    })
}

fn parse_watch_face_bin(bytes: &mut &[u8]) -> PResult<WatchfaceBinFile> {
    let res = bin_parser(Located::new(bytes));
    dbg!(&res);
    dbg!(bin_parser(Located::new(&b"\x01"[..])));
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_bin() {
        let bytes: Vec<u8> = vec![
            0x55, 0x49, 0x48, 0x48, // Signature
            // header
            0x01, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0xb5, 0xe5, 0x3d, 0x00, 0x3d, 0x00,
            0x30, 0x27, 0x00, 0x00, 0xab, 0x86, 0x09, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, // header end
            0x0C, 0x00, 0x00, 0x00, // Size of biggest param
            0x12, 0x00, 0x00, 0x00, // Size of params info: 18
            // First param info
            0x0a, 0x04, 0x08, 0x14, 0x10, 0x01, // size of params: 20, imagesCount: 1
            0x12, 0x04, 0x08, 0x00, 0x10, 0x08, // Background param info, offset 0, size 8
            0x1a, 0x04, 0x08, 0x08, 0x10, 0x0C, // Time param info, offset 8, size 12
            // Background param: x: 1, y: 2, imgid: 0
            0x0a, 0x06, 0x08, 0x01, 0x10, 0x02, 0x18, 0x00,
            // Time param: { Minutes: { Tens: { x: 16, y: 32, imgid: 0, imgcnt: 2 } } }
            0x12, 0x0A, 0x0A, 0x08, 0x08, 0x10, 0x10, 0x20, 0x18, 0x00, 0x20,
            0x02, // Time param
            0x00, 0x00, 0x00, 0x00, // Offset of 1st image: 0
            // Image
            0x42, 0x4D, 0x10, 0x00, 0x02, 0x00, 0x01, 0x00, 0x08, 0x00, 0x20, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x21, 0x31, 0x41, 0x12, 0x22, 0x32, 0x42,
        ];

        let result = parse_watch_face_bin(&mut &bytes[..]).unwrap();
        dbg!(&result);
        assert_eq!(
            result,
            WatchfaceBinFile {
                parameters: WatchfaceBinFileParams::MiBand(MiBandParams {
                    background: Some(Background {
                        image: ImageReference {
                            x: 1,
                            y: 2,
                            image_index: 0,
                        }
                    }),
                    time: Some(Time {
                        minutes: Some(ImageRange {
                            x: 16,
                            y: 32,
                            image_index: 0,
                            image_count: 2
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                // images: vec![Image {
                //     pixels: vec![
                //         0x11, // 1st pixel
                //         0x21,
                //         0x31,
                //         0xFF - 0x41, // 1st pixel end
                //         0x12,        // 2nd pixel
                //         0x22,
                //         0x32,
                //         0xFF - 0x42, // 2nd pixel end
                //     ],
                //     width: 2,
                //     height: 1,
                //     bits_per_pixel: 32,
                //     pixel_format: 0x10,
                // }]
            }
        );
    }

    #[test]
    fn parse_keys_and_values() {
        let bytes: Vec<u8> = vec![0x08, 0x04, 0x10, 0x6B];

        let result = params_parser(&mut Located::new(&bytes), bytes.len());
        assert!(result.is_ok());
        if let Ok(result) = result {
            assert_eq!(
                result,
                Params::from(HashMap::from([
                    (1, vec![Param::Bytes(vec![0x04])]),
                    (2, vec![Param::Bytes(vec![0x6B])]),
                ]))
            )
        }
    }

    #[test]
    fn parse_nested_structure() {
        let bytes: Vec<u8> = vec![0x0A, 0x05, 0x08, 0xBC, 0x04, 0x10, 0x6B];

        let result = params_parser(&mut Located::new(&bytes), bytes.len());
        assert!(result.is_ok());
        if let Ok(result) = result {
            assert_eq!(
                result,
                Params::from(HashMap::from([(
                    1,
                    vec![Param::Child(Params::from(HashMap::from([
                        (1, vec![Param::Bytes(vec![0x02, 0x3C])]),
                        (2, vec![Param::Bytes(vec![0x6B])]),
                    ])))]
                ),]))
            )
        }
    }

    #[test]
    fn parse_lists() {
        let bytes: Vec<u8> = vec![0x08, 0x04, 0x08, 0x7F, 0x10, 0x6B];

        let result = params_parser(&mut Located::new(&bytes), bytes.len());
        assert!(result.is_ok());
        if let Ok(result) = result {
            assert_eq!(
                result,
                Params::from(HashMap::from([
                    (1, vec![Param::Bytes(vec![0x04]), Param::Bytes(vec![0x7F])]),
                    (2, vec![Param::Bytes(vec![0x6B])]),
                ]))
            )
        }
    }

    #[test]
    fn parse_multi_byte_id() {
        let bytes: Vec<u8> = vec![0x80, 0x02, 0x04];

        let result = params_parser(&mut Located::new(&bytes), bytes.len());
        assert!(result.is_ok());
        if let Ok(result) = result {
            assert_eq!(
                result,
                Params::from(HashMap::from([(32, vec![Param::Bytes(vec![0x04])]),]))
            )
        }
    }

    #[test]
    fn parse_float_values() {
        let bytes: Vec<u8> = vec![
            0x0A, 0x0A, 0x0D, 0x00, 0x00, 0xA0, 0x3F, 0x3D, 0x00, 0x00, 0xB4, 0x43,
        ];

        let result = params_parser(&mut Located::new(&bytes), bytes.len());
        assert!(result.is_ok());
        if let Ok(result) = result {
            assert_eq!(
                result,
                Params::from(HashMap::from([(
                    1,
                    vec![Param::Child(Params::from(HashMap::from([
                        (1, vec![Param::Float(1.25)]),
                        (7, vec![Param::Float(360.0)])
                    ])),)]
                ),]))
            )
        }
    }
}
