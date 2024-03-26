#![allow(dead_code, unused_imports, unused_variables)] // TODO: Remove when moving from tests to real code

use {
    std::{
        collections::{hash_map::Entry, HashMap},
        default,
        fmt::{Debug, Display},
        mem::size_of,
    },
    winnow::{
        binary::{be_u16, le_f32, le_u16, le_u32, u8},
        stream::{Located, Location, Stream as _},
        token, PResult, Parser,
    },
};

pub type Stream<'i> = Located<&'i [u8]>;

#[derive(Debug, PartialEq, Default)]
struct Image {
    pixels: Vec<u16>,
    width: u16,
    height: u16,
    bits_per_pixel: u16,
    pixel_format: u16,
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
struct TimeNumbers {
    tens: Option<ImageRange>,
    ones: Option<ImageRange>,
}

#[derive(Debug, PartialEq, Default)]
struct Time {
    hours: Option<TimeNumbers>,
    minutes: Option<TimeNumbers>,
    seconds: Option<TimeNumbers>,
}

type Params = HashMap<u8, Vec<Param>>;

#[derive(Debug, PartialEq)]
enum Param {
    Bytes(Vec<u8>),
    Float(f32),
    Child(Params),
}

trait WatchfaceParams {
    fn new() -> Self;
    fn append(&mut self, key: u8, parameters: Params);
}

#[derive(Debug, PartialEq, Default)]
struct MiBandParams {
    background: Option<Background>,
    time: Option<Time>,
}

fn parse_image_ref(param: &Param) -> ImageReference {
    let mut image_ref = ImageReference {
        ..Default::default()
    };

    let subvalue = match param {
        Param::Child(child) => child,
        _ => panic!("First param should be child param"),
    };

    for (key, value) in subvalue.into_iter() {
        match key {
            1 => {
                image_ref.x = bytes_param_to_usize(value.get(0).unwrap()) as u32;
            }
            2 => {
                image_ref.y = bytes_param_to_usize(value.get(0).unwrap()) as u32;
            }
            3 => {
                image_ref.image_index = bytes_param_to_usize(value.get(0).unwrap()) as u32;
            }
            _ => (),
        }
    }
    image_ref
}

fn parse_image_range(param: &Param) -> ImageRange {
    let mut image_range = ImageRange {
        ..Default::default()
    };

    let subvalue = match param {
        Param::Child(child) => child,
        _ => panic!("First param should be child param"),
    };

    for (key, value) in subvalue.into_iter() {
        match key {
            1 => {
                image_range.x = bytes_param_to_usize(value.get(0).unwrap()) as u32;
            }
            2 => {
                image_range.y = bytes_param_to_usize(value.get(0).unwrap()) as u32;
            }
            3 => {
                image_range.image_index = bytes_param_to_usize(value.get(0).unwrap()) as u32;
            }
            4 => {
                image_range.image_count = bytes_param_to_usize(value.get(0).unwrap()) as u32;
            }
            _ => (),
        }
    }
    image_range
}

impl MiBandParams {
    fn parse_background(params: Params) -> Background {
        let mut background = Background {
            ..Default::default()
        };

        for (key, value) in params.into_iter() {
            match key {
                1 => {
                    background.image = parse_image_ref(value.get(0).unwrap());
                }
                _ => (),
            }
        }
        background
    }

    fn parse_time(params: Params) -> Time {
        let mut time = Time {
            ..Default::default()
        };

        for (key, value) in params.into_iter() {
            match key {
                2 => {
                    time.minutes = Self::parse_time_numbers(value.get(0).unwrap());
                }
                _ => (),
            }
        }
        time
    }

    fn parse_time_numbers(param: &Param) -> Option<TimeNumbers> {
        let mut numbers = TimeNumbers {
            ..Default::default()
        };
        let subvalue = match param {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        for (key, value) in subvalue.into_iter() {
            match key {
                1 => {
                    numbers.tens = Some(parse_image_range(value.get(0).unwrap()));
                }
                2 => {
                    numbers.ones = Some(parse_image_range(value.get(0).unwrap()));
                }
                _ => (),
            }
        }
        Some(numbers)
    }
}

impl WatchfaceParams for MiBandParams {
    fn new() -> Self {
        MiBandParams {
            ..Default::default()
        }
    }

    fn append(&mut self, key: u8, params: Params) {
        match key {
            2 => {
                self.background = Some(Self::parse_background(params));
            }
            3 => {
                self.time = Some(Self::parse_time(params));
            }
            _ => (),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Watchface<T: WatchfaceParams> {
    parameters: T,
    images: Vec<Image>,
}

fn variable_width_value_parser(i: &mut Stream) -> PResult<(i64, usize)> {
    let mut value = 0i64;
    let bytes = token::take_till(0..10, |b| b & 0x80 != 0x80).parse_next(i)?;
    let last = u8.parse_next(i)?;
    let mut i = 0;
    for b in [bytes, &[last]].concat() {
        value |= (b as i64 & 0x7f) << (i * 7);
        i += 1;
    }

    Ok((value, i))
}

fn write_variable_width_value(value: i64) -> Vec<u8> {
	let mut result = vec![];
	let mut value_big_int = value as u64;

	for i in 0..10 {
		// Read lower 7 bits of value
		let byte = (value_big_int & 0x7F) as u8;

		value_big_int = value_big_int >> 7;

		// If no data left after this byte, we can stop
		if value_big_int == 0 {
			result.push(byte);
			break
		}
		// Add byte with flag meaning the data continues in the next byte
		result.push(byte | 0x80)
	}

	result
}

fn param_parser(i: &mut Stream) -> PResult<(u8, Param)> {
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
        let field_value = temp_field_value.to_le_bytes()[0..value_size].to_vec();

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

fn image_parse(i: &mut Stream) -> PResult<Image> {
    let signature = le_u16.parse_next(i)?;
    if signature != 0x4D42 {
        panic!("Invalid image signature: {}", signature);
    }

    // read header
    let pixel_format = le_u16.parse_next(i)?;

    if pixel_format == 0x65 {
        todo!();
        // return parseCompressedImage(dataBuffer)
    } else if pixel_format == 0xFFFF {
        todo!();
        // return parse32BitImage(dataBuffer)
    }

    let width = le_u16.parse_next(i)?;
    let height = le_u16.parse_next(i)?;
    let row_size = le_u16.parse_next(i)?;
    let bits_per_pixel = le_u16.parse_next(i)?;
    let palette_colors_count = le_u16.parse_next(i)?;
    let transparent_palette_color = le_u16.parse_next(i)?;

    if !([16, 24, 32].contains(&bits_per_pixel)
        && palette_colors_count == 0
        && [0x08, 0x13, 0x1B, 0x1C, 0x10, 0x09].contains(&pixel_format))
        && !([1, 2, 4, 8].contains(&bits_per_pixel)
            && palette_colors_count > 0
            && pixel_format == 0x64)
    {
        panic!("Unsuported pixel format/color depth/Palette (should add support) {pixel_format} {bits_per_pixel} {palette_colors_count}")
    }

    if ((bits_per_pixel * width) as f32 / 8.).ceil() as u16 != row_size {
        panic!("Row size is not as expected (Padding ?)")
    }

    let palette_size = 0;
    // let palette = [];

    if palette_colors_count > 0 {
        // Read palette
        todo!();
        // for (let i = 0; i < palette_colors_count; i++) {
        // 	const color = {
        // 		red: dataView.getUint8(HEADER_SIZE + i * 4),
        // 		green: dataView.getUint8(HEADER_SIZE + i * 4 + 1),
        // 		blue: dataView.getUint8(HEADER_SIZE + i * 4 + 2),
        // 		alpha: i === transparentPaletteColor - 1 ? 0xFF : 0x00
        // 	}
        // 	palette.push(color)
        // }

        // 	palette_size = paletteColorsCount * 4
    }

    // Read pixel data
    let mut pixels = vec![0; (4 * width * height).into()];
    for y in 0..height {
        for x in 0..width {
            // read pixel color info
            let red;
            let green;
            let blue;
            let mut alpha = 0x00;
            if palette_colors_count != 0 {
                todo!();
            // let colorId
            // if (bits_per_pixel < 8) {
            // 	const pixelsPerByte = 8 / bits_per_pixel
            // 	const byte = dataView.getUint8(HEADER_SIZE + paletteSize + (y * row_size) + Math.floor(x / pixelsPerByte))
            // 	const bitMask = (1 << bits_per_pixel) - 1
            // 	const bitPosition = 8 - ((x % pixelsPerByte) + 1) * bits_per_pixel
            // 	colorId = (byte >> bitPosition) & bitMask;
            // } else {
            // 	colorId = dataView.getUint8(HEADER_SIZE + paletteSize + (y * row_size) + x)
            // }
            // const color = palette[colorId]
            // red = color.red
            // green = color.green
            // blue = color.blue
            // alpha = color.alpha
            } else {
                let byte_per_pixel = bits_per_pixel / 8;

                if byte_per_pixel == 4 {
                    red = u8.parse_next(i)? as u16;
                    green = u8.parse_next(i)? as u16;
                    blue = u8.parse_next(i)? as u16;
                    alpha = u8.parse_next(i)? as u16;
                } else {
                    let rgba;
                    if byte_per_pixel == 3 {
                        // 24 bits is 16 bit color data (big endian) with 8 bit alpha
                        alpha = u8.parse_next(i)? as u16;
                        rgba = be_u16.parse_next(i)?;
                    } else {
                        // for the 16 bit images, the value is little endian
                        rgba = le_u16.parse_next(i)?;
                    }
                    if pixel_format == 0x13 {
                        // color is 16 bit (4:4:4:4) abgr
                        alpha = (rgba & 0xF000) >> 8;
                        blue = (rgba & 0x0F00) >> 4;
                        green = rgba & 0x00F0;
                        red = (rgba & 0x000F) << 4;
                    } else if pixel_format == 0x1C || pixel_format == 0x09 {
                        // color is 16bit (5:6:5) rgb
                        red = (rgba & 0xF800) >> 8;
                        green = (rgba & 0x07E0) >> 3;
                        blue = (rgba & 0x001F) << 3;
                    } else {
                        // color is 16bit (5:6:5) bgr
                        blue = (rgba & 0xF800) >> 8;
                        green = (rgba & 0x07E0) >> 3;
                        red = (rgba & 0x001F) << 3;
                    }
                }
            }

            let pixel_position = ((y * width + x) * 4) as usize;
            pixels[pixel_position] = red;
            pixels[pixel_position + 1] = green;
            pixels[pixel_position + 2] = blue;
            // Alpha is inverted, 0xFF is transparent
            pixels[pixel_position + 3] = 0xFF - alpha;
        }
    }

    Ok(Image {
        width,
        height,
        bits_per_pixel,
        pixel_format,
        pixels,
        ..Default::default()
    })
}

fn params_parser(i: &mut Stream, max_size: usize) -> PResult<Params> {
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

fn bytes_to_usize(bytes: &[u8]) -> usize {
    let zeros = (0..(size_of::<usize>() - bytes.len()))
        .map(|_| 0u8)
        .collect::<Vec<_>>();
    let bytes = [bytes, &zeros[..]].concat();
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

fn bin_parser<T: WatchfaceParams>(mut i: Located<&[u8]>) -> PResult<Watchface<T>> {
    let _signature = token::take(4usize).parse_next(&mut i)?;
    let _header = token::take(75usize).parse_next(&mut i)?;
    let _buffer_size = le_u32.parse_next(&mut i)?;
    let info_size = le_u32.parse_next(&mut i)?;
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

    let mut parameters = T::new();

    let params_start = i.checkpoint();

    for (key, value) in parameter_info.iter() {
        if *key == 1 {
            continue;
        }

        i.reset(&params_start);

        let subvalue = match value.get(0).unwrap() {
            Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        let offset = bytes_param_to_usize(&subvalue.get(&1).unwrap()[0]);
        let size = bytes_param_to_usize(&subvalue.get(&2).unwrap()[0]);

        i.next_slice(offset);
        let params = params_parser(&mut i, size)?;

        parameters.append(*key, params);
    }

    i.reset(&params_start);
    i.next_slice(parameters_size);

    let images_info_size = 4 * images_count;
    let images_info = token::take(images_info_size).parse_next(&mut i)?;

    let images_start = i.checkpoint();

    // Load each image
    let mut images = vec![];
    for offset_index in 0..images_count {
        let image_offset = bytes_to_usize(&images_info[offset_index * 4..offset_index * 4 + 4]);
        i.reset(&images_start);
        i.next_slice(image_offset);
        let image = image_parse(&mut i)?;
        images.push(image);
    }

    Ok(Watchface { parameters, images })
}

fn parse_watch_face_bin<T: WatchfaceParams>(bytes: &mut &[u8]) -> PResult<Watchface<T>> {
    let res = bin_parser::<T>(Located::new(bytes));
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
            0x0a, 0x04, 0x08, 0x20, 0x10, 0x01, // size of params: 32, imagesCount: 1
            0x12, 0x04, 0x08, 0x00, 0x10, 0x09, // Background param info, offset 0, size 9
            0x1a, 0x04, 0x08, 0x09, 0x10, 0x17, // Time param info, offset 9, size 23
            // Background param: x: 1, y: 258, imgid: 0
            0x0a, 0x07, 0x08, 0x01, 0x10, 0x82, 0x02, 0x18, 0x00, // Background param
            0x12, 0x15, // Minutes: size: 21
            0x0A, 0x08, // Tens: size: 8
            // x: 16, y: 32, imgid: 0, imgcnt: 2
            0x08, 0x10, 0x10, 0x20, 0x18, 0x00, 0x20, 0x02, // Tens
            0x12, 0x09, // Ones: size: 9
            // x: 731, y: 12, imgid: 1, imgcnt: 7
            0x08, 0xDB, 0x05, 0x10, 0x0C, 0x18, 0x01, 0x20, 0x07, // Ones
            0x00, 0x00, 0x00, 0x00, // Offset of 1st image: 0
            // Image
            0x42, 0x4D, 0x10, 0x00, 0x02, 0x00, 0x01, 0x00, 0x08, 0x00, 0x20, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x21, 0x31, 0x41, 0x12, 0x22, 0x32, 0x42,
        ];

        let result = parse_watch_face_bin::<MiBandParams>(&mut &bytes[..]).unwrap();
        assert_eq!(
            result,
            Watchface {
                parameters: MiBandParams {
                    background: Some(Background {
                        image: ImageReference {
                            x: 1,
                            y: 258,
                            image_index: 0,
                        }
                    }),
                    time: Some(Time {
                        minutes: Some(TimeNumbers {
                            tens: Some(ImageRange {
                                x: 16,
                                y: 32,
                                image_index: 0,
                                image_count: 2
                            }),
                            ones: Some(ImageRange {
                                x: 731,
                                y: 12,
                                image_index: 1,
                                image_count: 7
                            })
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                images: vec![Image {
                    pixels: vec![
                        0x11, // 1st pixel
                        0x21,
                        0x31,
                        0xFF - 0x41, // 1st pixel end
                        0x12,        // 2nd pixel
                        0x22,
                        0x32,
                        0xFF - 0x42, // 2nd pixel end
                    ],
                    width: 2,
                    height: 1,
                    bits_per_pixel: 32,
                    pixel_format: 0x10,
                }]
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
                        (1, vec![Param::Bytes(vec![0x3C, 0x02])]),
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

    #[test]
    fn read_single_byte_value() {
        let bytes: Vec<u8> = vec![0x73];

        let result = variable_width_value_parser(&mut Located::new(&bytes));
        assert!(result.is_ok());
        if let Ok((value, value_size)) = result {
            assert_eq!(
                (value, value_size),
                (0x73, 1),
            )
        }
    }

    #[test]
    fn read_multi_byte_value() {
        let bytes: Vec<u8> = vec![0xF3, 0x42];

        let result = variable_width_value_parser(&mut Located::new(&bytes));
        assert!(result.is_ok());
        if let Ok((value, value_size)) = result {
            assert_eq!(
                (value, value_size),
                (0x2173, 2),
            )
        }
    }

    #[test]
    fn read_negative_values() {
        let bytes: Vec<u8> = vec![0xF3, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];

        let result = variable_width_value_parser(&mut Located::new(&bytes));
        assert!(result.is_ok());
        if let Ok((value, value_size)) = result {
            assert_eq!(
                (value, value_size),
                (-13, 10),
            )
        }
    }

    #[test]
    fn read_32bit_negative_values() {
        let bytes: Vec<u8> = vec![0xF3, 0xFF, 0xFF, 0xFF, 0x0F];

        let result = variable_width_value_parser(&mut Located::new(&bytes));
        assert!(result.is_ok());
        if let Ok((value, value_size)) = result {
            assert_eq!(
                // TODO: implement 32 bit reading because it is needed for UIHH_BIPU_GTS2MINI format
                // we can live without it for now
                (value as i32, value_size),
                (-13, 5),
            )
        }
    }

    #[test]
    fn read_31_bit_value() {
        let bytes: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x04];

        let result = variable_width_value_parser(&mut Located::new(&bytes));
        assert!(result.is_ok());
        if let Ok((value, value_size)) = result {
            dbg!((value as i32, value_size));
            assert_eq!(
                (value as i32, value_size),
                (1073741824, 5),
            )
        }
    }

    #[test]
    fn read_32_bit_value() {
        let bytes: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x08];

        let result = variable_width_value_parser(&mut Located::new(&bytes));
        assert!(result.is_ok());
        if let Ok((value, value_size)) = result {
            assert_eq!(
                (value, value_size),
                (2147483648, 5),
            )
        }
    }

    #[test]
    fn read_33_bit_value() {
        let bytes: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x10];

        let result = variable_width_value_parser(&mut Located::new(&bytes));
        assert!(result.is_ok());
        if let Ok((value, value_size)) = result {
            assert_eq!(
                (value, value_size),
                (4294967296, 5),
            )
        }
    }

    #[test]
    fn write_small_value_on_one_byte () {
        assert_eq!(
            write_variable_width_value(0x73),
            vec![0x73],
        )
    }

    #[test]
    fn write_bigger_values_on_multiple_bytes () {
        assert_eq!(
            write_variable_width_value(0x2173),
            vec![0xF3, 0x42],
        )
    }

    #[test]
    fn write_negative_values () {
        assert_eq!(
            write_variable_width_value(-13),
            vec![0xF3, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
        )
    }

    // #[test]
    // fn write_32bit_negative_values () {
    //     assert_eq!(
    //         write_variable_width_value(-13, 32),
    //         vec![0xF3, 0xFF, 0xFF, 0xFF, 0x0F],
    //     )
    // }

    #[test]
    fn write_31_bit_value () {
        assert_eq!(
            write_variable_width_value(1073741824),
            vec![0x80, 0x80, 0x80, 0x80, 0x04],
        )
    }

    #[test]
    fn write_32_bit_value () {
        assert_eq!(
            write_variable_width_value(2147483648),
            vec![0x80, 0x80, 0x80, 0x80, 0x08],
        )
    }

    #[test]
    fn write_33_bit_value () {
        assert_eq!(
            write_variable_width_value(4294967296),
            vec![0x80, 0x80, 0x80, 0x80, 0x10],
        )
    }

}
