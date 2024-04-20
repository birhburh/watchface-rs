mod common;
mod miband;
mod parser;
mod preview;

use {
    common::*, // TODO: not use star
    parser::*, // TODO: not use star
    winnow::{stream::Located, PResult},
};

pub use common::ImageType;
pub use common::PreviewParams;
pub use common::Watchface;
pub use miband::MiBandParams;
use preview::Preview;

pub fn parse_watch_face_bin<T>(bytes: &mut &[u8]) -> PResult<Watchface<T>>
where
    T: WatchfaceParams + Preview,
    Option<T>: Transform,
{
    bin_parser(Located::new(bytes))
}

#[cfg(test)]
mod tests {
    use {super::*, miband::*};

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

        let result: Watchface<MiBandParams> = parse_watch_face_bin(&mut &bytes[..]).unwrap();
        assert_eq!(
            result,
            Watchface {
                parameters: Some(MiBandParams {
                    background: Some(Background {
                        image: Some(ImageReference {
                            x: 1,
                            y: 258,
                            image_index: Some(ImgId(0)),
                        }),
                        ..Default::default()
                    }),
                    time: Some(Time {
                        minutes: Some(TimeNumbers {
                            tens: Some(ImageRange {
                                x: 16,
                                y: 32,
                                image_index: Some(ImgId(0)),
                                images_count: Some(2)
                            }),
                            ones: Some(ImageRange {
                                x: 731,
                                y: 12,
                                image_index: Some(ImgId(1)),
                                images_count: Some(7)
                            })
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
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
}
