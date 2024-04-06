pub use crate::common::Watchface;
pub use crate::miband::MiBandParams;

#[cfg(test)]
mod tests {
    use {crate::common::*, crate::miband::*};

    #[test]
    fn generate_simple_preview() {
        let watchface = Watchface {
            parameters: Some(MiBandParams {
                background: Some(Background {
                    image: Some(ImageReference {
                        x: 1,
                        y: 128,
                        image_index: Some(ImgId(0)),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            images: vec![Image {
                width: 2,
                height: 1,
                ..Default::default()
            }],
        };
        let preview = watchface.generate_preview(None);
        assert_eq!(
            preview,
            vec![ImageWithCoords {
                x: 1,
                y: 128,
                image_index: ImgId(0),
            },]
        )
    }

    #[test]
    fn generate_preview_with_time() {
        let watchface = Watchface {
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
                    hours: Some(TimeNumbers {
                        tens: Some(ImageRange {
                            x: 10,
                            y: 20,
                            image_index: Some(ImgId(1)),
                            images_count: Some(10),
                        }),
                        ones: Some(ImageRange {
                            x: 15,
                            y: 20,
                            image_index: Some(ImgId(1)),
                            images_count: Some(10),
                        }),
                    }),
                    minutes: Some(TimeNumbers {
                        tens: Some(ImageRange {
                            x: 10,
                            y: 40,
                            image_index: Some(ImgId(1)),
                            images_count: Some(10),
                        }),
                        ones: Some(ImageRange {
                            x: 15,
                            y: 40,
                            image_index: Some(ImgId(1)),
                            images_count: Some(10),
                        }),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            images: vec![
                Image {
                    width: 2,
                    height: 1,
                    ..Default::default()
                },
                Image {
                    width: 2,
                    height: 1,
                    ..Default::default()
                },
                Image {
                    width: 2,
                    height: 1,
                    ..Default::default()
                },
                Image {
                    width: 2,
                    height: 1,
                    ..Default::default()
                },
                Image {
                    width: 2,
                    height: 1,
                    ..Default::default()
                },
                Image {
                    width: 2,
                    height: 1,
                    ..Default::default()
                },
                Image {
                    width: 2,
                    height: 1,
                    ..Default::default()
                },
                Image {
                    width: 2,
                    height: 1,
                    ..Default::default()
                },
                Image {
                    width: 2,
                    height: 1,
                    ..Default::default()
                },
                Image {
                    width: 2,
                    height: 1,
                    ..Default::default()
                },
                Image {
                    width: 2,
                    height: 1,
                    ..Default::default()
                },
            ],
        };
        let preview = watchface.generate_preview(Some(PreviewParams {
            hours: Some(11),
            minutes: Some(6),
        }));
        assert_eq!(
            preview,
            vec![
                ImageWithCoords {
                    x: 1,
                    y: 258,
                    image_index: ImgId(0),
                }, // background
                ImageWithCoords {
                    x: 10,
                    y: 20,
                    image_index: ImgId(2),
                }, // hours first digit 1
                ImageWithCoords {
                    x: 15,
                    y: 20,
                    image_index: ImgId(2),
                }, // hours second digit 1
                ImageWithCoords {
                    x: 10,
                    y: 40,
                    image_index: ImgId(1),
                }, // minutes first digit 0
                ImageWithCoords {
                    x: 15,
                    y: 40,
                    image_index: ImgId(7),
                }, // minutes second digit 6
            ]
        )
    }
}
