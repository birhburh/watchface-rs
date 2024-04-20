use {
    crate::common::*,
    crate::miband::*,
    std::f32::consts::PI,
    tiny_skia::{
        FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform as Transform2, BYTES_PER_PIXEL,
    },
};

pub enum ParamType {
    Bool(bool),
    U32(Option<u32>),
    I32(Option<i32>),
    F32(Option<f32>),
}

pub trait Preview {
    fn get_images(
        &self,
        all_params: &Option<PreviewParams>,
        params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords>;

    // fn get_params(params: &Option<PreviewParams>) -> &[ParamType];
}

fn compute_position_with_aligment(
    lower_bound: i32,
    upper_bound: i32,
    element_size: i32,
    alignment: u32,
) -> i32 {
    let result = if alignment & 0x02 == 0x02 {
        // lower bound align
        lower_bound
    } else if alignment & 0x04 == 0x04 {
        // upper bound align
        upper_bound - element_size
    } else
    /*if (alignment & 0x08)*/
    {
        // center align (default)
        ((lower_bound + upper_bound) as f32 / 2. - element_size as f32 / 2.).round() as i32
    };

    // don't allow to go lower than the lower bound
    result.max(lower_bound)
}

fn status_image_get_images(
    status_image: &Option<StatusImage>,
    param: bool,
    _images: &Vec<Image>,
) -> Vec<ImageWithCoords> {
    let mut res = vec![];

    if let Some(status_image) = &status_image {
        if param {
            if let Some(on_image_index) = &status_image.on_image_index {
                if let Some(coordinates) = &status_image.coordinates {
                    res.push(ImageWithCoords {
                        x: coordinates.x,
                        y: coordinates.y,
                        image_type: ImageType::Id(ImgId(on_image_index.0)),
                    });
                }
            }
        } else {
            if let Some(off_image_index) = &status_image.off_image_index {
                if let Some(coordinates) = &status_image.coordinates {
                    res.push(ImageWithCoords {
                        x: coordinates.x,
                        y: coordinates.y,
                        image_type: ImageType::Id(ImgId(off_image_index.0)),
                    });
                }
            }
        }
    }

    res
}

fn text_get_images(
    number: &NumberInRect,
    image_ids: Vec<u32>,
    images: &Vec<Image>,
) -> Vec<ImageWithCoords> {
    let mut res = vec![];

    // compute width of text to display
    let text_width = image_ids
        .iter()
        .map(|img_id| images[*img_id as usize].width)
        .reduce(|a, b| ((a + b) as i32 + number.spacing_x).try_into().unwrap())
        .unwrap_or_default();

    let aligment = match &number.alignment {
        Alignment::Valid(valid) => *valid as u32,
        Alignment::Unknown => 0,
    };

    // compute coordinates of the left of the first character
    let mut x = compute_position_with_aligment(
        number.top_left_x,
        number.bottom_right_x,
        text_width.try_into().unwrap(),
        aligment,
    );

    // Add all characters
    for (i, element_image_id) in image_ids.iter().enumerate() {
        let image = &images[*element_image_id as usize];
        let y = compute_position_with_aligment(
            number.top_left_y,
            number.bottom_right_y,
            image.height as i32,
            aligment >> 3,
        ) + i as i32 * number.spacing_y;

        res.push(ImageWithCoords {
            x,
            y,
            image_type: ImageType::Id(ImgId(*element_image_id)),
        });

        x += image.width as i32 + number.spacing_x;
    }

    res
}

fn number_get_image_ids(
    number: &NumberInRect,
    param: f32,
    decimal_point_image_index: &Option<ImgId>,
    minus_image_index: &Option<ImgId>,
    min_width: Option<usize>,
) -> Vec<u32> {
    let mut image_ids = vec![];

    if param < 0.0 {
        if let Some(minus_image_index) = minus_image_index {
            image_ids.push(minus_image_index.0);
        }
    }

    if let Some(image_index) = &number.image_index {
        let mut int_part = param.abs().trunc() as u32;
        let mut int_part_image_ids = vec![];
        while int_part != 0 {
            int_part_image_ids.push(image_index.0 + int_part % 10);
            int_part /= 10;
        }
        if let Some(min_width) = min_width {
            while int_part_image_ids.len() < min_width {
                int_part_image_ids.push(image_index.0 + 0);
            }
        }
        if int_part_image_ids.is_empty() {
            int_part_image_ids.push(image_index.0 + 0);
        }
        int_part_image_ids.reverse();
        image_ids.append(&mut int_part_image_ids);

        let mut fract = (param.fract() * 100.0).round() as u32;
        let mut fract_image_ids = vec![];
        if let Some(decimal_point_image_index) = decimal_point_image_index {
            if int_part_image_ids.len() < 3 {
                image_ids.push(decimal_point_image_index.0);

                while fract != 0 {
                    fract_image_ids.push(image_index.0 + fract % 10);
                    fract /= 10;
                }
                while fract_image_ids.len() < 2 {
                    fract_image_ids.push(image_index.0 + 0);
                }
                fract_image_ids.reverse();
                image_ids.append(&mut fract_image_ids);
            }
        }
    }
    image_ids
}

fn number_get_images(
    number: &Option<NumberInRect>,
    param: f32,
    images: &Vec<Image>,
    prefix_image_index: &Option<ImgId>,
    decimal_point_image_index: &Option<ImgId>,
    minus_image_index: &Option<ImgId>,
    suffix_image_index: &Option<ImgId>,
    min_width: Option<usize>,
) -> Vec<ImageWithCoords> {
    let mut res = vec![];

    if let Some(number) = number {
        let mut image_ids = vec![];

        if let Some(prefix_image_index) = prefix_image_index {
            image_ids.push(prefix_image_index.0);
        }

        image_ids.append(&mut number_get_image_ids(
            number,
            param,
            decimal_point_image_index,
            minus_image_index,
            min_width,
        ));

        if let Some(suffix_image_index) = suffix_image_index {
            image_ids.push(suffix_image_index.0);
        }

        res.append(&mut text_get_images(number, image_ids, images))
    }

    res
}

fn numbers_with_delimiters_get_images(
    number: &Option<NumberInRect>,
    params: &[f32],
    images: &Vec<Image>,
    minus_image_index: &Option<ImgId>,
    delimiter_image_index: &Option<ImgId>,
    suffix_image_index: &Option<ImgId>,
    append_suffix_to_all: bool,
    min_width: Option<usize>,
) -> Vec<ImageWithCoords> {
    let mut res = vec![];

    if let Some(number) = number {
        let mut image_ids = vec![];

        for (i, param) in params.iter().enumerate() {
            image_ids.append(&mut number_get_image_ids(
                number,
                *param,
                &None,
                minus_image_index,
                min_width,
            ));

            if let Some(suffix_image_index) = suffix_image_index {
                if append_suffix_to_all {
                    image_ids.push(suffix_image_index.0);
                }
            }

            if let Some(delimiter_image_index) = delimiter_image_index {
                if i != params.len() - 1 {
                    image_ids.push(delimiter_image_index.0);
                }
            }
        }

        if let Some(suffix_image_index) = suffix_image_index {
            image_ids.push(suffix_image_index.0);
        }

        res.append(&mut text_get_images(number, image_ids, images))
    }
    res
}

fn vector_shape_get_images(
    vector_shape: &Option<VectorShape>,
    param: Option<u32>,
    total_value: f32,
) -> Vec<ImageWithCoords> {
    let mut res = vec![];
    if let Some(value) = param {
        if let Some(vector_shape) = &vector_shape {
            if let Some(color) = &vector_shape.color {
                if let Some(center) = &vector_shape.center {
                    if let Some(first) = &vector_shape.shape.get(0) {
                        let angle = (2. * PI * value as f32 / total_value - PI / 2.) * 180. / PI;

                        // TODO: replace 126, 294 with sizes of watchface screen
                        let mut pixmap = Pixmap::new(126, 294).unwrap();

                        let mut paint = Paint::default();
                        paint.set_color_rgba8(color.0, color.1, color.2, color.3);
                        paint.anti_alias = true;

                        let mut pb = PathBuilder::new();
                        pb.move_to(first.x as f32, first.y as f32);

                        for point in &vector_shape.shape[1..] {
                            pb.line_to(point.x as f32, point.y as f32);
                        }
                        pb.close();
                        let path = pb.finish().unwrap();

                        let only_border = if let Some(only_border) = vector_shape.only_border {
                            only_border
                        } else {
                            false
                        };

                        if only_border {
                            let stroke = Stroke::default();
                            pixmap.stroke_path(
                                &path,
                                &paint,
                                &stroke,
                                Transform2::from_translate(center.x as f32, center.y as f32)
                                    .pre_rotate(angle),
                                None,
                            );
                        } else {
                            pixmap.fill_path(
                                &path,
                                &paint,
                                FillRule::Winding,
                                Transform2::from_translate(center.x as f32, center.y as f32)
                                    .pre_rotate(angle),
                                None,
                            );
                        }

                        let width = pixmap.width() as u16;
                        let height = pixmap.height() as u16;
                        let pixels = pixmap.take();

                        res.push(ImageWithCoords {
                            x: 0,
                            y: 0,
                            image_type: ImageType::Image(Image {
                                pixels,
                                width,
                                height,
                                bits_per_pixel: (BYTES_PER_PIXEL * 8) as u16,
                                pixel_format: 0,
                            }),
                        });
                    }
                }
            }

            if let Some(center_image) = &vector_shape.center_image {
                if let Some(image_index) = &center_image.image_index {
                    res.push(ImageWithCoords {
                        x: center_image.x,
                        y: center_image.y,
                        image_type: ImageType::Id(ImgId(image_index.0)),
                    });
                }
            }
        }
    }

    res
}

impl Preview for Option<ImageReference> {
    fn get_images(
        &self,
        _all_params: &Option<PreviewParams>,
        _params: &Vec<ParamType>,
        _images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(image) = &self {
            if let Some(image_index) = &image.image_index {
                res.push(ImageWithCoords {
                    x: image.x,
                    y: image.y,
                    image_type: ImageType::Id(ImgId(image_index.0)),
                })
            }
        }

        res
    }
}

impl Preview for Option<Background> {
    fn get_images(
        &self,
        all_params: &Option<PreviewParams>,
        _params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(background) = &self {
            res.append(&mut background.image.get_images(all_params, &vec![], images));
        }

        res
    }
}

impl Preview for Option<ImageRange> {
    fn get_images(
        &self,
        _all_params: &Option<PreviewParams>,
        params: &Vec<ParamType>,
        _images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(image_range) = self {
            if let Some(image_index) = &image_range.image_index {
                if let Some(param) = params.get(0) {
                    if let ParamType::U32(param) = param {
                        if let Some(value) = param {
                            res.push(ImageWithCoords {
                                x: image_range.x,
                                y: image_range.y,
                                image_type: ImageType::Id(ImgId(image_index.0 + value)),
                            });
                        }
                    }
                }
            }
        }

        res
    }
}

impl Preview for Option<TimeNumbers> {
    fn get_images(
        &self,
        all_params: &Option<PreviewParams>,
        params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(time_numbers) = self {
            if let Some(param) = params.get(0) {
                if let ParamType::U32(param) = param {
                    if let Some(two_nums) = param {
                        res.append(&mut time_numbers.tens.get_images(
                            all_params,
                            &vec![ParamType::U32(Some(two_nums / 10))],
                            images,
                        ));
                        res.append(&mut time_numbers.ones.get_images(
                            all_params,
                            &vec![ParamType::U32(Some(two_nums % 10))],
                            images,
                        ));
                    }
                }
            }
        }

        res
    }
}

impl Preview for Option<Time> {
    fn get_images(
        &self,
        all_params: &Option<PreviewParams>,
        _params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(time) = &self {
            if let Some(all_params_val) = &all_params {
                res.append(&mut time.hours.get_images(
                    all_params,
                    &vec![ParamType::U32(all_params_val.hours)],
                    images,
                ));
                res.append(&mut time.minutes.get_images(
                    all_params,
                    &vec![ParamType::U32(all_params_val.minutes)],
                    images,
                ));
                res.append(&mut time.seconds.get_images(
                    all_params,
                    &vec![ParamType::U32(all_params_val.seconds)],
                    images,
                ));
            }

            res.append(&mut time.delimiter_image.get_images(all_params, &vec![], images));
            res.append(
                &mut time
                    .time_delimiter_image
                    .get_images(all_params, &vec![], images),
            );
        }

        res
    }
}

impl Preview for Option<Steps> {
    fn get_images(
        &self,
        _all_params: &Option<PreviewParams>,
        params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(steps) = &self {
            if let Some(param) = params.get(0) {
                if let ParamType::U32(param) = param {
                    if let Some(value) = param {
                        res.append(&mut number_get_images(
                            &steps.number,
                            *value as f32,
                            images,
                            &steps.prefix_image_index,
                            &None,
                            &None,
                            &steps.suffix_image_index,
                            None,
                        ));
                    }
                }
            }
        }

        res
    }
}

impl Preview for Option<Pulse> {
    fn get_images(
        &self,
        _all_params: &Option<PreviewParams>,
        params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(pulse) = &self {
            if let Some(param) = params.get(0) {
                if let ParamType::U32(param) = param {
                    if let Some(value) = param {
                        res.append(&mut number_get_images(
                            &pulse.number,
                            *value as f32,
                            images,
                            &pulse.prefix_image_index,
                            &None,
                            &None,
                            &pulse.suffix_image_index,
                            None,
                        ));
                    }
                }
            }
        }

        res
    }
}

impl Preview for Option<Calories> {
    fn get_images(
        &self,
        _all_params: &Option<PreviewParams>,
        params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(calories) = &self {
            if let Some(param) = params.get(0) {
                if let ParamType::U32(param) = param {
                    if let Some(value) = param {
                        res.append(&mut number_get_images(
                            &calories.number,
                            *value as f32,
                            images,
                            &None,
                            &None,
                            &None,
                            &calories.suffix_image_index,
                            None,
                        ));
                    }
                }
            }
        }

        res
    }
}

impl Preview for Option<PAI> {
    fn get_images(
        &self,
        _all_params: &Option<PreviewParams>,
        params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(pai) = &self {
            if let Some(param) = params.get(0) {
                if let ParamType::U32(param) = param {
                    if let Some(value) = param {
                        res.append(&mut number_get_images(
                            &pai.number,
                            *value as f32,
                            images,
                            &None,
                            &None,
                            &None,
                            &None,
                            None,
                        ));
                    }
                }
            }
        }

        res
    }
}

impl Preview for Option<Distance> {
    fn get_images(
        &self,
        _all_params: &Option<PreviewParams>,
        params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(distance) = &self {
            if let Some(param) = params.get(0) {
                if let ParamType::F32(param) = param {
                    if let Some(value) = param {
                        res.append(&mut number_get_images(
                            &distance.number,
                            *value,
                            images,
                            &None,
                            &distance.decimal_point_image_index,
                            &None,
                            &distance.km_suffix_image_index,
                            None,
                        ));
                    }
                }
            }
        }

        res
    }
}

impl Preview for Option<Activity> {
    fn get_images(
        &self,
        all_params: &Option<PreviewParams>,
        _params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(activity) = &self {
            if let Some(all_params_val) = &all_params {
                res.append(&mut activity.steps.get_images(
                    all_params,
                    &vec![ParamType::U32(all_params_val.steps)],
                    images,
                ));
                res.append(&mut activity.calories.get_images(
                    all_params,
                    &vec![ParamType::U32(all_params_val.calories)],
                    images,
                ));
                res.append(&mut activity.pulse.get_images(
                    all_params,
                    &vec![ParamType::U32(all_params_val.pulse)],
                    images,
                ));
                res.append(&mut activity.distance.get_images(
                    all_params,
                    &vec![ParamType::F32(all_params_val.distance)],
                    images,
                ));
                res.append(&mut activity.pai.get_images(
                    all_params,
                    &vec![ParamType::U32(all_params_val.pai)],
                    images,
                ));
            }
        }

        res
    }
}

impl Preview for Option<Linear> {
    fn get_images(
        &self,
        _all_params: &Option<PreviewParams>,
        params: &Vec<ParamType>,
        _images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(linear) = &self {
            if let Some(start_image_index) = &linear.start_image_index {
                if let Some(param) = params.get(0) {
                    if let ParamType::U32(param) = param {
                        if let Some(value) = param {
                            let progress = (*value as f32 / 100.
                                * (linear.segments.len() - 1) as f32)
                                .round() as usize;
                            for i in 0..=progress {
                                res.push(ImageWithCoords {
                                    x: linear.segments[i].x,
                                    y: linear.segments[i].y,
                                    image_type: ImageType::Id(ImgId(
                                        start_image_index.0 + i as u32,
                                    )),
                                });
                            }
                        }
                    }
                }
            }
        }

        res
    }
}

impl Preview for Option<HeartProgress> {
    fn get_images(
        &self,
        all_params: &Option<PreviewParams>,
        _params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(heart_progress) = &self {
            if let Some(all_params_val) = &all_params {
                res.append(&mut heart_progress.linear.get_images(
                    all_params,
                    &vec![ParamType::U32(all_params_val.heart_progress)],
                    images,
                ));

                if let Some(value) = all_params_val.heart_progress {
                    if let Some(line_scale) = &heart_progress.line_scale {
                        if let Some(images_count) = line_scale.images_count {
                            res.append(&mut heart_progress.line_scale.get_images(
                                all_params,
                                &vec![ParamType::U32(Some(
                                    (value as f32 / 100. * (images_count - 1) as f32).round()
                                        as u32,
                                ))],
                                images,
                            ));
                        }
                    }
                }
            }
        }

        res
    }
}

impl Preview for Option<WeekDaysIcons> {
    fn get_images(
        &self,
        all_params: &Option<PreviewParams>,
        _params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(week_days_icons) = &self {
            if let Some(all_params_val) = &all_params {
                if let Some(value) = all_params_val.weekday {
                    let day = match value {
                        0 => &week_days_icons.monday,
                        1 => &week_days_icons.tuesday,
                        2 => &week_days_icons.wednesday,
                        3 => &week_days_icons.thursday,
                        4 => &week_days_icons.friday,
                        5 => &week_days_icons.saturday,
                        6 => &week_days_icons.sunday,
                        _ => unreachable!(),
                    };

                    res.append(&mut day.get_images(all_params, &vec![], images));
                }
            }
        }

        res
    }
}

impl Preview for Option<Alarm> {
    fn get_images(
        &self,
        all_params: &Option<PreviewParams>,
        _params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(alarm) = &self {
            if let Some(all_params_val) = &all_params {
                if all_params_val.alarm_on {
                    res.append(&mut alarm.on_image.get_images(all_params, &vec![], images));
                } else {
                    res.append(&mut alarm.off_image.get_images(all_params, &vec![], images));
                }
                if let Some(alarm_hours) = all_params_val.alarm_hours {
                    if let Some(alarm_minutes) = all_params_val.alarm_minutes {
                        res.append(&mut numbers_with_delimiters_get_images(
                            &alarm.number,
                            &vec![alarm_hours as f32, alarm_minutes as f32],
                            images,
                            &None,
                            &alarm.delimiter_image_index,
                            &None,
                            false,
                            Some(2),
                        ));
                    }
                }
            }
        }

        res
    }
}

impl Preview for Option<Status> {
    fn get_images(
        &self,
        all_params: &Option<PreviewParams>,
        _params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(status) = &self {
            if let Some(all_params_val) = &all_params {
                res.append(&mut status_image_get_images(
                    &status.do_not_disturb,
                    all_params_val.do_not_disturb,
                    images,
                ));
                res.append(&mut status_image_get_images(
                    &status.lock,
                    all_params_val.lock,
                    images,
                ));
                res.append(&mut status_image_get_images(
                    &status.bluetooth,
                    all_params_val.bluetooth,
                    images,
                ));
            }
        }

        res
    }
}

impl Preview for Option<Date> {
    fn get_images(
        &self,
        all_params: &Option<PreviewParams>,
        _params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(date) = &self {
            if let Some(all_params_val) = &all_params {
                if let Some(month_and_day_and_year) = &date.month_and_day_and_year {
                    if let Some(separate) = &month_and_day_and_year.separate {
                        if let Some(value) = all_params_val.month {
                            res.append(&mut number_get_images(
                                &separate.month,
                                value as f32,
                                images,
                                &None,
                                &None,
                                &None,
                                &None,
                                Some(2),
                            ));

                            res.append(&mut separate.months_en.get_images(
                                all_params,
                                &vec![ParamType::U32(Some(value - 1))],
                                images,
                            ));
                        }
                        if let Some(value) = all_params_val.day {
                            res.append(&mut number_get_images(
                                &separate.day,
                                value as f32,
                                images,
                                &None,
                                &None,
                                &None,
                                &None,
                                Some(2),
                            ));
                        }
                    }

                    if let Some(one_line) = &month_and_day_and_year.one_line {
                        if let Some(month) = all_params_val.month {
                            if let Some(day) = all_params_val.day {
                                res.append(&mut numbers_with_delimiters_get_images(
                                    &one_line.number,
                                    &vec![month as f32, day as f32],
                                    images,
                                    &None,
                                    &one_line.delimiter_image_index,
                                    &None,
                                    false,
                                    Some(2),
                                ));
                            }
                        }
                    }
                }
            }

            if let Some(all_params_val) = &all_params {
                if let Some(day_am_pm) = &date.day_am_pm {
                    if all_params_val.time12h {
                        if all_params_val.am {
                            if let Some(image_index_amen) = &day_am_pm.image_index_amen {
                                res.push(ImageWithCoords {
                                    x: day_am_pm.x,
                                    y: day_am_pm.y,
                                    image_type: ImageType::Id(ImgId(image_index_amen.0)),
                                });
                            }
                        } else {
                            if let Some(image_index_pmen) = &day_am_pm.image_index_pmen {
                                res.push(ImageWithCoords {
                                    x: day_am_pm.x,
                                    y: day_am_pm.y,
                                    image_type: ImageType::Id(ImgId(image_index_pmen.0)),
                                });
                            }
                        }
                    }
                }
            }

            if let Some(all_params_val) = &all_params {
                if let Some(weekday) = all_params_val.weekday {
                    res.append(&mut date.en_week_days.get_images(
                        all_params,
                        &vec![ParamType::U32(Some(weekday))],
                        images,
                    ));
                }
            }
        }

        res
    }
}

impl Preview for Option<Weather> {
    fn get_images(
        &self,
        all_params: &Option<PreviewParams>,
        _params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];


        if let Some(weather) = &self {
            if let Some(all_params_val) = &all_params {
                if let Some(icon) = &weather.icon {
                    if let Some(value) = all_params_val.weather {
                        res.append(&mut icon.custom_icon.get_images(
                            all_params,
                            &vec![ParamType::U32(Some(value))],
                            images,
                        ));
                    }
                }

                if let Some(temperature) = &weather.temperature {
                    if let Some(value) = all_params_val.temperature {
                        if let Some(current) = &temperature.current {
                            res.append(&mut number_get_images(
                                &current.number,
                                value as f32,
                                images,
                                &None,
                                &None,
                                &current.minus_image_index,
                                &current.suffix_image_index,
                                None,
                            ));
                        }
                    }

                    if let Some(today) = &temperature.today {
                        if let Some(separate) = &today.separate {
                            if let Some(day) = &separate.day {
                                if let Some(value) = all_params_val.day_temperature {
                                    res.append(&mut number_get_images(
                                        &day.number,
                                        value as f32,
                                        images,
                                        &None,
                                        &None,
                                        &day.minus_image_index,
                                        &day.suffix_image_index,
                                        None,
                                    ));
                                }
                            }

                            if let Some(night) = &separate.night {
                                if let Some(value) = all_params_val.night_temperature {
                                    res.append(&mut number_get_images(
                                        &night.number,
                                        value as f32,
                                        images,
                                        &None,
                                        &None,
                                        &night.minus_image_index,
                                        &night.suffix_image_index,
                                        None,
                                    ));
                                }
                            }
                        }
                    }
                }

                if let Some(humidity) = &weather.humidity {
                    if let Some(value) = all_params_val.humidity {
                        res.append(&mut number_get_images(
                            &humidity.number,
                            value as f32,
                            images,
                            &None,
                            &None,
                            &None,
                            &humidity.suffix_image_index,
                            None,
                        ));
                        if let Some(image_pos_suffix) = &humidity.image_pos_suffix {
                            if let Some(image_index) = &image_pos_suffix.image_index {
                                res.push(ImageWithCoords {
                                    x: image_pos_suffix.x,
                                    y: image_pos_suffix.y,
                                    image_type: ImageType::Id(ImgId(image_index.0)),
                                });
                            }
                        }
                    }
                }

                if let Some(wind) = &weather.wind {
                    if let Some(value) = all_params_val.wind {
                        res.append(&mut number_get_images(
                            &wind.number,
                            value as f32,
                            images,
                            &None,
                            &None,
                            &None,
                            &wind.suffix_image_index_en,
                            None,
                        ));
                        if let Some(image_pos_suffix_en) = &wind.image_pos_suffix_en {
                            if let Some(image_index) = &image_pos_suffix_en.image_index {
                                res.push(ImageWithCoords {
                                    x: image_pos_suffix_en.x,
                                    y: image_pos_suffix_en.y,
                                    image_type: ImageType::Id(ImgId(image_index.0)),
                                });
                            }
                        }
                    }
                }

                if let Some(uv_index) = &weather.uv_index {
                    if let Some(value) = all_params_val.uv {
                        if let Some(uv) = &uv_index.uv {
                            res.append(&mut number_get_images(
                                &uv.number,
                                value as f32,
                                images,
                                &None,
                                &None,
                                &None,
                                &uv.suffix_image_index,
                                None,
                            ));
                        }
                    }
                }
            }
        }

        res
    }
}

impl Preview for Option<StepsProgress> {
    fn get_images(
        &self,
        all_params: &Option<PreviewParams>,
        _params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(steps_progress) = &self {
            if let Some(all_params_val) = &all_params {
                res.append(&mut steps_progress.linear.get_images(
                    all_params,
                    &vec![ParamType::U32(all_params_val.steps_progress)],
                    images,
                ));
                if let Some(value) = all_params_val.steps_progress {
                    if let Some(line_scale) = &steps_progress.line_scale {
                        if let Some(images_count) = line_scale.images_count {
                            res.append(&mut steps_progress.line_scale.get_images(
                                all_params,
                                &vec![ParamType::U32(Some(
                                    (value as f32 / 100. * (images_count - 1) as f32).round()
                                        as u32,
                                ))],
                                images,
                            ));
                        }
                    }
                }
            }
        }

        res
    }
}

impl Preview for Option<Battery> {
    fn get_images(
        &self,
        all_params: &Option<PreviewParams>,
        _params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(battery) = &self {
            if let Some(all_params_val) = &all_params {
                if let Some(battery_text) = &battery.battery_text {
                    if let Some(value) = all_params_val.battery {
                        res.append(&mut number_get_images(
                            &battery_text.number,
                            value as f32,
                            images,
                            &battery_text.prefix_image_index,
                            &None,
                            &None,
                            &battery_text.suffix_image_index,
                            None,
                        ));
                    }
                }

                if let Some(battery_icon) = &battery.battery_icon {
                    if let Some(value) = all_params_val.battery {
                        if let Some(images_count) = battery_icon.images_count {
                            res.append(&mut battery.battery_icon.get_images(
                                all_params,
                                &vec![ParamType::U32(Some(
                                    (value as f32 / 100. * (images_count - 1) as f32).round()
                                        as u32,
                                ))],
                                images,
                            ));
                        }
                    }
                }
                res.append(&mut battery.linear.get_images(
                    all_params,
                    &vec![ParamType::U32(all_params_val.battery)],
                    images,
                ));
            }
        }

        res
    }
}

impl Preview for Option<AnalogDialFace> {
    fn get_images(
        &self,
        all_params: &Option<PreviewParams>,
        _params: &Vec<ParamType>,
        _images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(analog_dial_face) = &self {
            if let Some(all_params_val) = &all_params {
                res.append(&mut vector_shape_get_images(
                    &analog_dial_face.hours,
                    all_params_val.hours,
                    12.,
                ));
                res.append(&mut vector_shape_get_images(
                    &analog_dial_face.minutes,
                    all_params_val.minutes,
                    60.,
                ));
                res.append(&mut vector_shape_get_images(
                    &analog_dial_face.seconds,
                    all_params_val.seconds,
                    60.,
                ));
            }
        }

        res
    }
}

impl Preview for Option<Other> {
    fn get_images(
        &self,
        all_params: &Option<PreviewParams>,
        _params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        if let Some(other) = &self {
            if let Some(all_params_val) = &all_params {
                if let Some(value) = all_params_val.animation {
                    for animation in &other.animation.0 {
                        res.append(&mut animation.animation_images.get_images(
                            all_params,
                            &vec![ParamType::U32(Some(value))],
                            images,
                        ));
                    }
                }
            }
        }

        res
    }
}

impl Preview for MiBandParams {
    fn get_images(
        &self,
        all_params: &Option<PreviewParams>,
        _params: &Vec<ParamType>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];

        res.append(&mut self.background.get_images(all_params, &vec![], images));
        res.append(&mut self.time.get_images(all_params, &vec![], images));
        res.append(&mut self.activity.get_images(all_params, &vec![], images));
        res.append(&mut self.heart_progress.get_images(all_params, &vec![], images));
        res.append(&mut self.week_days_icons.get_images(all_params, &vec![], images));
        res.append(&mut self.alarm.get_images(all_params, &vec![], images));
        res.append(&mut self.status.get_images(all_params, &vec![], images));
        res.append(&mut self.date.get_images(all_params, &vec![], images));
        res.append(&mut self.weather.get_images(all_params, &vec![], images));
        res.append(&mut self.steps_progress.get_images(all_params, &vec![], images));
        res.append(&mut self.battery.get_images(all_params, &vec![], images));
        res.append(&mut self.analog_dial_face.get_images(all_params, &vec![], images));
        res.append(&mut self.other.get_images(all_params, &vec![], images));
        res.append(&mut self.status2.get_images(all_params, &vec![], images));

        res
    }
}

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
