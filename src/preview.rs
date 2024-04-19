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
                image_type: ImageType::Id(ImgId(0)),
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
            ..Default::default()
        }));
        assert_eq!(
            preview,
            vec![
                ImageWithCoords {
                    x: 1,
                    y: 258,
                    image_type: ImageType::Id(ImgId(0)),
                }, // background
                ImageWithCoords {
                    x: 10,
                    y: 20,
                    image_type: ImageType::Id(ImgId(2)),
                }, // hours first digit 1
                ImageWithCoords {
                    x: 15,
                    y: 20,
                    image_type: ImageType::Id(ImgId(2)),
                }, // hours second digit 1
                ImageWithCoords {
                    x: 10,
                    y: 40,
                    image_type: ImageType::Id(ImgId(1)),
                }, // minutes first digit 0
                ImageWithCoords {
                    x: 15,
                    y: 40,
                    image_type: ImageType::Id(ImgId(7)),
                }, // minutes second digit 6
            ]
        )
    }

    #[test]
    fn generate_preview_with_steps() {
        let watchface = Watchface {
            parameters: Some(MiBandParams {
                background: Some(Background {
                    image: Some(ImageReference {
                        x: 1,
                        y: 39,
                        image_index: Some(ImgId(0)),
                    }),
                    ..Default::default()
                }),
                activity: Some(Activity {
                    steps: Some(Steps {
                        number: Some(NumberInRect {
                            top_left_x: 10,
                            top_left_y: 20,
                            bottom_right_x: 100,
                            bottom_right_y: 50,
                            alignment: Alignment::Valid(AlignmentInternal::CenterLeft),
                            spacing_x: 1,
                            spacing_y: 0,
                            image_index: Some(ImgId(1)),
                            images_count: Some(10),
                        }),
                        ..Default::default()
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
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
                Image {
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
                Image {
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
                Image {
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
                Image {
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
                Image {
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
                Image {
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
                Image {
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
                Image {
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
                Image {
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
            ],
        };
        let preview = watchface.generate_preview(Some(PreviewParams {
            steps: Some(1284),
            ..Default::default()
        }));
        assert_eq!(
            preview,
            vec![
                ImageWithCoords {
                    x: 1,
                    y: 39,
                    image_type: ImageType::Id(ImgId(0)),
                }, // background
                ImageWithCoords {
                    x: 10,
                    y: 31,
                    image_type: ImageType::Id(ImgId(2)),
                }, // hours first digit 1
                ImageWithCoords {
                    x: 16,
                    y: 31,
                    image_type: ImageType::Id(ImgId(3)),
                }, // hours second digit 1
                ImageWithCoords {
                    x: 22,
                    y: 31,
                    image_type: ImageType::Id(ImgId(9)),
                }, // minutes first digit 0
                ImageWithCoords {
                    x: 28,
                    y: 31,
                    image_type: ImageType::Id(ImgId(5)),
                }, // minutes second digit 6
            ]
        )
    }

    #[test]
    fn generate_preview_with_top_center_alignment() {
        let watchface = Watchface {
            parameters: Some(MiBandParams {
                background: Some(Background {
                    image: Some(ImageReference {
                        x: 16,
                        y: 79,
                        image_index: Some(ImgId(0)),
                    }),
                    ..Default::default()
                }),
                activity: Some(Activity {
                    steps: Some(Steps {
                        number: Some(NumberInRect {
                            top_left_x: 10,
                            top_left_y: 20,
                            bottom_right_x: 100,
                            bottom_right_y: 50,
                            alignment: Alignment::Valid(AlignmentInternal::TopCenter),
                            spacing_x: 1,
                            spacing_y: 0,
                            image_index: Some(ImgId(1)),
                            images_count: Some(10),
                        }),
                        ..Default::default()
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
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
                Image {
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
                Image {
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
                Image {
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
                Image {
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
                Image {
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
                Image {
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
                Image {
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
                Image {
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
                Image {
                    width: 5,
                    height: 8,
                    ..Default::default()
                },
            ],
        };
        let preview = watchface.generate_preview(Some(PreviewParams {
            steps: Some(1284),
            ..Default::default()
        }));
        assert_eq!(
            preview,
            vec![
                ImageWithCoords {
                    x: 16,
                    y: 79,
                    image_type: ImageType::Id(ImgId(0)),
                }, // background
                ImageWithCoords {
                    x: 44,
                    y: 20,
                    image_type: ImageType::Id(ImgId(2)),
                }, // hours first digit 1
                ImageWithCoords {
                    x: 50,
                    y: 20,
                    image_type: ImageType::Id(ImgId(3)),
                }, // hours second digit 1
                ImageWithCoords {
                    x: 56,
                    y: 20,
                    image_type: ImageType::Id(ImgId(9)),
                }, // minutes first digit 0
                ImageWithCoords {
                    x: 62,
                    y: 20,
                    image_type: ImageType::Id(ImgId(5)),
                }, // minutes second digit 6
            ]
        )
    }
}
