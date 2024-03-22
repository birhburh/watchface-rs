use std::{
    io::Read,
    mem::{size_of, transmute},
};

use winnow::{
    binary::{le_u32, u8},
    combinator::repeat,
    token, PResult, Parser,
};

pub type Stream<'i> = &'i [u8];

#[derive(Debug, PartialEq)]
struct Image {
    pixels: Vec<u8>,
    width: u32,
    height: u32,
    bits_per_pixel: u32,
    pixel_format: u32,
}

#[derive(Debug, PartialEq)]
struct ImageWithIndex {
    x: u32,
    y: u32,
    image_index: u32,
}

#[derive(Debug, PartialEq)]
struct Background {
    image: ImageWithIndex,
}

type Params = Vec<Param>;

#[derive(Debug, PartialEq)]
enum ParamValue {
    Values(Vec<Vec<u8>>),
    Children(Params),
}

#[derive(Debug, PartialEq)]
struct Param {
    key: u8,
    value: ParamValue,
}

#[derive(Debug, PartialEq)]
struct WatchfaceBinFile {
    parameters: Params,
    images: Vec<Image>,
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

fn param_parser<'s>(i: &mut Stream<'s>) -> PResult<Param> {
    let (field_descriptor, _) = variable_width_value_parser(i)?;

    let key = (field_descriptor >> 3) as u8;
    let has_children = field_descriptor & 0x02 == 0x02;

    // From the second byte on is the value
    let mut field_value: Vec<u8>;
    let value_size;
    let value;
    let is_float = field_descriptor & 0x05 == 0x05;
    if is_float {
        // float value
        field_value = token::take(size_of::<f32>()).parse_next(i)?.into();
        value_size = size_of::<f32>();
    } else {
        // variable width value
        let temp_field_value;
        (temp_field_value, value_size) = variable_width_value_parser(i)?;
        value = temp_field_value.to_le_bytes();
        field_value = value[0..value_size].to_vec();
        field_value.reverse();
    }

    let value;
    if has_children && !is_float {
        // When node has children, field value is size of children
        let zeros = (0..(size_of::<usize>() - value_size)).map(|_| 0u8).collect::<Vec<_>>();
        let bytes = [zeros, field_value].concat();
        let bytes = bytes[0..size_of::<usize>()].try_into().unwrap();
        let children_size = usize::from_le_bytes(bytes);
        if children_size <= 0 {
            panic!("Children size of 0 or less");
        }
        // Recursive call to read children data
        let children = params_parser(i, children_size)?;
        value = ParamValue::Children(children);
    } else {
        value = ParamValue::Values(vec![field_value]);
    }

    Ok(Param { key, value })
}

fn params_parser<'s>(i: &mut Stream<'s>, max_size: usize) -> PResult<Params> {
    let params = repeat(0..=max_size, param_parser)
        .fold(Vec::new, |mut res, el| {
            res.push(el);
            res
        })
        .parse_next(i)?;
    Ok(Params::from(params))
}

fn bin_parser<'s>(i: &mut Stream<'s>) -> PResult<(u32, u32)> {
    let _signature = token::take(4usize).parse_next(i)?;
    let _header = token::take(75usize).parse_next(i)?;
    let buffer_size = le_u32.parse_next(i)?;
    let info_size = le_u32.parse_next(i)?;
    let params = params_parser(i, info_size as usize)?;
    Ok((buffer_size, info_size))
}

pub fn parse_watch_face_bin(bytes: &mut &[u8]) /* -> PResult<WatchfaceBinFile> */
{
    dbg!(bin_parser(bytes));
    dbg!(bin_parser(&mut &b"\x01"[..]));
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
            0x08, 0x00, 0x00, 0x00, // Size of biggest param
            0x0C, 0x00, 0x00, 0x00, // Size of params info: 12
            // First param info
            0x0a, 0x04, 0x08, 0x08, 0x10, 0x01, // size of params: 8, imagesCount: 1
            0x12, 0x04, 0x08, 0x00, 0x10, 0x08, // Background param info, offset 0, size 8
            // Background param
            0x0a, 0x06, 0x08, 0x00, 0x10, 0x00, 0x18, 0x00, // x: 0, y: 0, imgid: 0
            0x00, 0x00, 0x00, 0x00, // Offset of 1st image: 0
            // Image
            0x42, 0x4D, 0x10, 0x00, 0x02, 0x00, 0x01, 0x00, 0x08, 0x00, 0x20, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x21, 0x31, 0x41, 0x12, 0x22, 0x32, 0x42,
        ];

        let result = parse_watch_face_bin(&mut &bytes[..]);
        dbg!(result);
        // assert_eq!(
        //     result,
        //     WatchfaceBinFile {
        //         parameters: Parameters {
        //             background: Background {
        //                 image: ImageWithIndex {
        //                     x: 0,
        //                     y: 0,
        //                     image_index: 0
        //                 }
        //             }
        //         },
        //         images: vec![Image {
        //             pixels: vec![
        //                 0x11, // 1st pixel
        //                 0x21,
        //                 0x31,
        //                 0xFF - 0x41, // 1st pixel end
        //                 0x12,        // 2nd pixel
        //                 0x22,
        //                 0x32,
        //                 0xFF - 0x42, // 2nd pixel end
        //             ],
        //             width: 2,
        //             height: 1,
        //             bits_per_pixel: 32,
        //             pixel_format: 0x10,
        //         }]
        //     }
        // );
    }

    #[test]
    fn parse_keys_and_values() {
        let bytes: Vec<u8> = vec![0x08, 0x04, 0x10, 0x6B];

        let result = params_parser(&mut &bytes[..], 2);
        assert!(result.is_ok());
        if let Ok(result) = result {
            assert_eq!(
                result,
                Params::from(vec![
                    Param {
                        key: 1,
                        value: ParamValue::Values(vec![vec![0x04]])
                    },
                    Param {
                        key: 2,
                        value: ParamValue::Values(vec![vec![0x6B]])
                    }
                ])
            )
        }
    }

    #[test]
    fn parse_nested_structure() {
        let bytes: Vec<u8> = vec![0x0A, 0x05, 0x08, 0xBC, 0x04, 0x10, 0x6B];

        let result = params_parser(&mut &bytes[..], 2);
        assert!(result.is_ok());
        if let Ok(result) = result {
            assert_eq!(
                result,
                Params::from(vec![Param {
                    key: 1,
                    value: ParamValue::Children(Params::from(vec![
                        Param {
                            key: 1,
                            value: ParamValue::Values(vec![vec![0x02, 0x3C]]),
                        },
                        Param {
                            key: 2,
                            value: ParamValue::Values(vec![vec![0x6B]])
                        }
                    ]))
                }])
            )
        }
    }

    #[test]
    fn parse_lists() {
        let bytes: Vec<u8> = vec![0x08, 0x04, 0x08, 0x7F, 0x10, 0x6B];

        let result = params_parser(&mut &bytes[..], 3);
        assert!(result.is_ok());
        if let Ok(result) = result {
            assert_eq!(
                result,
                Params::from(vec![Param {
                            key: 1,
                            value: ParamValue::Values(vec![vec![0x04], vec![0x7F]]),
                        },
                        Param {
                            key: 2,
                            value: ParamValue::Values(vec![vec![0x6B]])
                        }
                    ])
            )
        }
    }

    // fn parse_multi_byte_id() {
    //     expect(parseParameters(Uint8Array.of(0x80, 0x02, 0x04))).toStrictEqual(
    //         {
    //             "32": 0x04
    //         }
    //     )
    // }

    // fn parse_float_values() {
    //     expect(parseParameters(Uint8Array.of(0x0A, 0x0A, 0x0D, 0x00, 0x00, 0xA0, 0x3F, 0x3D, 0x00, 0x00, 0xB4, 0x43))).toStrictEqual(
    //         {
    //             "1": {
    //                 "1": 1.25,
    //                 "7": 360.0
    //             }
    //         }
    //     )
    // }
}
