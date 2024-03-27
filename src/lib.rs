mod common;
mod miband;
mod parser;

use {
    common::*, // TODO: not use star
    parser::*, // TODO: not use star
    winnow::{stream::Located, PResult},
};

pub use miband::MiBandParams;

pub fn parse_watch_face_bin<T: WatchfaceParams>(bytes: &mut &[u8]) -> PResult<Watchface<T>> {
    let res = bin_parser::<T>(Located::new(bytes));
    res
}

#[cfg(test)]
mod tests {
    use {super::*, miband::*, std::collections::HashMap};

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
                        image: Some(ImageReference {
                            x: 1,
                            y: 258,
                            image_index: 0,
                        }),
                        ..Default::default()
                    }),
                    time: Some(Time {
                        minutes: Some(TimeNumbers {
                            tens: Some(ImageRange {
                                x: 16,
                                y: 32,
                                image_index: 0,
                                images_count: 2
                            }),
                            ones: Some(ImageRange {
                                x: 731,
                                y: 12,
                                image_index: 1,
                                images_count: 7
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
                    (1, vec![Param::Number(0x04)]),
                    (2, vec![Param::Number(0x6B)]),
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
                        (1, vec![Param::Number(0x023C)]),
                        (2, vec![Param::Number(0x6B)]),
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
                    (1, vec![Param::Number(0x04), Param::Number(0x7F)]),
                    (2, vec![Param::Number(0x6B)]),
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
                Params::from(HashMap::from([(32, vec![Param::Number(0x04)]),]))
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
            assert_eq!((value, value_size), (0x73, 1),)
        }
    }

    #[test]
    fn read_multi_byte_value() {
        let bytes: Vec<u8> = vec![0xF3, 0x42];

        let result = variable_width_value_parser(&mut Located::new(&bytes));
        assert!(result.is_ok());
        if let Ok((value, value_size)) = result {
            assert_eq!((value, value_size), (0x2173, 2),)
        }
    }

    #[test]
    fn read_negative_values() {
        let bytes: Vec<u8> = vec![0xF3, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];

        let result = variable_width_value_parser(&mut Located::new(&bytes));
        assert!(result.is_ok());
        if let Ok((value, value_size)) = result {
            assert_eq!((value, value_size), (-13, 10),)
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
            assert_eq!((value as i32, value_size), (1073741824, 5),)
        }
    }

    #[test]
    fn read_32_bit_value() {
        let bytes: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x08];

        let result = variable_width_value_parser(&mut Located::new(&bytes));
        assert!(result.is_ok());
        if let Ok((value, value_size)) = result {
            assert_eq!((value, value_size), (2147483648, 5),)
        }
    }

    #[test]
    fn read_33_bit_value() {
        let bytes: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x10];

        let result = variable_width_value_parser(&mut Located::new(&bytes));
        assert!(result.is_ok());
        if let Ok((value, value_size)) = result {
            assert_eq!((value, value_size), (4294967296, 5),)
        }
    }

    #[test]
    fn write_small_value_on_one_byte() {
        assert_eq!(write_variable_width_value(0x73), vec![0x73],)
    }

    #[test]
    fn write_bigger_values_on_multiple_bytes() {
        assert_eq!(write_variable_width_value(0x2173), vec![0xF3, 0x42],)
    }

    #[test]
    fn write_negative_values() {
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
    fn write_31_bit_value() {
        assert_eq!(
            write_variable_width_value(1073741824),
            vec![0x80, 0x80, 0x80, 0x80, 0x04],
        )
    }

    #[test]
    fn write_32_bit_value() {
        assert_eq!(
            write_variable_width_value(2147483648),
            vec![0x80, 0x80, 0x80, 0x80, 0x08],
        )
    }

    #[test]
    fn write_33_bit_value() {
        assert_eq!(
            write_variable_width_value(4294967296),
            vec![0x80, 0x80, 0x80, 0x80, 0x10],
        )
    }
}
