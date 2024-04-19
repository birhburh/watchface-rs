use {
    crate::common::*, // TODO: not use star
    serde::{ser::SerializeSeq, Deserialize, Serialize},
    std::fmt::Debug,
    watchface_rs_derive::TransformDerive,
};

// TODO: check that all fields from UIHH_MIBAND.json copied

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct MiBandParams {
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<Background>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<Time>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<Activity>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,
    #[wfrs_id(6)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weather: Option<Weather>,
    #[wfrs_id(7)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps_progress: Option<StepsProgress>,
    #[wfrs_id(8)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[wfrs_id(9)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery: Option<Battery>,
    #[wfrs_id(10)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analog_dial_face: Option<AnalogDialFace>,
    #[wfrs_id(11)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Other>,
    #[wfrs_id(12)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heart_progress: Option<HeartProgress>,
    #[wfrs_id(14)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week_days_icons: Option<WeekDaysIcons>,
    #[wfrs_id(15)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calories_progress: Option<CaloriesProgress>,
    #[wfrs_id(18)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm: Option<Alarm>,
    #[wfrs_id(20)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status2: Option<Status>,
    #[wfrs_id(21)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<UnknownStruct>,
    #[wfrs_id(22)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lunar_date: Option<LunarDate>,
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
                        image_index: ImgId(on_image_index.0),
                    });
                }
            }
        } else {
            if let Some(off_image_index) = &status_image.off_image_index {
                if let Some(coordinates) = &status_image.coordinates {
                    res.push(ImageWithCoords {
                        x: coordinates.x,
                        y: coordinates.y,
                        image_index: ImgId(off_image_index.0),
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
            image_index: ImgId(*element_image_id),
        });

        x += image.width as i32 + number.spacing_x;
    }

    res
}

fn number_get_image_ids(
    number: &NumberInRect,
    param: f32,
    images: &Vec<Image>,
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
            images,
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
                images,
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

fn image_range_get_images(
    image_range: &Option<ImageRange>,
    value: u32,
    _images: &Vec<Image>,
) -> Vec<ImageWithCoords> {
    let mut res = vec![];

    if let Some(image_range) = &image_range {
        if let Some(image_index) = &image_range.image_index {
            res.push(ImageWithCoords {
                x: image_range.x,
                y: image_range.y,
                image_index: ImgId(image_index.0 + value),
            })
        }
    }

    res
}

fn time_numbers_get_images(
    time_numbers: &Option<TimeNumbers>,
    param: Option<u32>,
    images: &Vec<Image>,
) -> Vec<ImageWithCoords> {
    let mut res = vec![];

    if let Some(time_numbers) = time_numbers {
        if let Some(two_nums) = param {
            res.append(&mut &mut image_range_get_images(
                &time_numbers.tens,
                two_nums / 10,
                images,
            ));
            res.append(&mut &mut image_range_get_images(
                &time_numbers.ones,
                two_nums % 10,
                images,
            ));
        }
    }

    res
}

impl WatchfaceParams for MiBandParams {
    fn get_images(
        &self,
        params: Option<PreviewParams>,
        images: &Vec<Image>,
    ) -> Vec<ImageWithCoords> {
        let mut res = vec![];
        if let Some(background) = &self.background {
            if let Some(image) = &background.image {
                if let Some(image_index) = &image.image_index {
                    res.push(ImageWithCoords {
                        x: image.x,
                        y: image.y,
                        image_index: ImgId(image_index.0),
                    })
                }
            }
        }

        if let Some(time) = &self.time {
            if let Some(params) = &params {
                res.append(&mut &mut time_numbers_get_images(
                    &time.hours,
                    params.hours,
                    images,
                ));
                res.append(&mut &mut time_numbers_get_images(
                    &time.minutes,
                    params.minutes,
                    images,
                ));
                res.append(&mut &mut time_numbers_get_images(
                    &time.seconds,
                    params.seconds,
                    images,
                ));
                if let Some(delimiter_image) = &time.delimiter_image {
                    if let Some(image_index) = &delimiter_image.image_index {
                        res.push(ImageWithCoords {
                            x: delimiter_image.x,
                            y: delimiter_image.y,
                            image_index: ImgId(image_index.0),
                        })
                    }
                }
                if let Some(time_delimiter_image) = &time.time_delimiter_image {
                    if let Some(image_index) = &time_delimiter_image.image_index {
                        res.push(ImageWithCoords {
                            x: time_delimiter_image.x,
                            y: time_delimiter_image.y,
                            image_index: ImgId(image_index.0),
                        })
                    }
                }
            }
        }

        if let Some(activity) = &self.activity {
            if let Some(params) = &params {
                if let Some(steps) = &activity.steps {
                    if let Some(value) = params.steps {
                        res.append(&mut &mut number_get_images(
                            &steps.number,
                            value as f32,
                            images,
                            &steps.prefix_image_index,
                            &None,
                            &None,
                            &steps.suffix_image_index,
                            None,
                        ));
                    }
                }

                if let Some(pulse) = &activity.pulse {
                    if let Some(value) = params.pulse {
                        res.append(&mut &mut number_get_images(
                            &pulse.number,
                            value as f32,
                            images,
                            &pulse.prefix_image_index,
                            &None,
                            &None,
                            &pulse.suffix_image_index,
                            None,
                        ));
                    }
                }

                if let Some(distance) = &activity.distance {
                    if let Some(value) = params.distance {
                        res.append(&mut &mut number_get_images(
                            &distance.number,
                            value as f32,
                            images,
                            &None,
                            &distance.decimal_point_image_index,
                            &None,
                            &distance.km_suffix_image_index,
                            None,
                        ));
                    }
                }

                if let Some(calories) = &activity.calories {
                    if let Some(value) = params.calories {
                        res.append(&mut &mut number_get_images(
                            &calories.number,
                            value as f32,
                            images,
                            &None,
                            &None,
                            &None,
                            &calories.suffix_image_index,
                            None,
                        ));
                    }
                }

                if let Some(pai) = &activity.pai {
                    if let Some(value) = params.pai {
                        res.append(&mut &mut number_get_images(
                            &pai.number,
                            value as f32,
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

        if let Some(heart_progress) = &self.heart_progress {
            if let Some(params) = &params {
                if let Some(value) = params.heart_progress {
                    if let Some(linear) = &heart_progress.linear {
                        if let Some(start_image_index) = &linear.start_image_index {
                            let progress = (value as f32 / 100. * linear.segments.len() as f32)
                                .round() as usize;
                            for i in 0..progress {
                                res.push(ImageWithCoords {
                                    x: linear.segments[i].x,
                                    y: linear.segments[i].y,
                                    image_index: ImgId(start_image_index.0 + i as u32),
                                });
                            }
                        }
                    }

                    if let Some(line_scale) = &heart_progress.line_scale {
                        if let Some(images_count) = line_scale.images_count {
                            res.append(&mut &mut image_range_get_images(
                                &heart_progress.line_scale,
                                (value as f32 / 100. * (images_count - 1) as f32).round() as u32,
                                images,
                            ));
                        }
                    }
                }
            }
        }

        if let Some(week_days_icons) = &self.week_days_icons {
            if let Some(params) = &params {
                if let Some(value) = params.weekday {
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
                    if let Some(day) = day {
                        if let Some(image_index) = &day.image_index {
                            res.push(ImageWithCoords {
                                x: day.x,
                                y: day.y,
                                image_index: ImgId(image_index.0),
                            })
                        }
                    }
                }
            }
        }

        if let Some(alarm) = &self.alarm {
            if let Some(params) = &params {
                if params.alarm_on {
                    if let Some(on_image) = &alarm.on_image {
                        if let Some(image_index) = &on_image.image_index {
                            res.push(ImageWithCoords {
                                x: on_image.x,
                                y: on_image.y,
                                image_index: ImgId(image_index.0),
                            });
                        }
                    }
                } else {
                    if let Some(off_image) = &alarm.off_image {
                        if let Some(image_index) = &off_image.image_index {
                            res.push(ImageWithCoords {
                                x: off_image.x,
                                y: off_image.y,
                                image_index: ImgId(image_index.0),
                            });
                        }
                    }
                }
                if let Some(alarm_hours) = params.alarm_hours {
                    if let Some(alarm_minutes) = params.alarm_minutes {
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

        if let Some(status) = &self.status {
            if let Some(params) = &params {
                res.append(&mut status_image_get_images(
                    &status.do_not_disturb,
                    params.do_not_disturb,
                    images,
                ));
                res.append(&mut status_image_get_images(
                    &status.lock,
                    params.lock,
                    images,
                ));
                res.append(&mut status_image_get_images(
                    &status.bluetooth,
                    params.bluetooth,
                    images,
                ));
            }
        }

        if let Some(date) = &self.date {
            if let Some(params) = &params {
                if let Some(month_and_day_and_year) = &date.month_and_day_and_year {
                    if let Some(separate) = &month_and_day_and_year.separate {
                        if let Some(value) = params.month {
                            res.append(&mut &mut number_get_images(
                                &separate.month,
                                value as f32,
                                images,
                                &None,
                                &None,
                                &None,
                                &None,
                                Some(2),
                            ));

                            res.append(&mut image_range_get_images(
                                &separate.months_en,
                                value - 1,
                                images,
                            ));
                        }
                        if let Some(value) = params.day {
                            res.append(&mut &mut number_get_images(
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
                        if let Some(month) = params.month {
                            if let Some(day) = params.day {
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

            if let Some(params) = &params {
                if let Some(day_am_pm) = &date.day_am_pm {
                    if params.time12h {
                        if params.am {
                            if let Some(image_index_amen) = &day_am_pm.image_index_amen {
                                res.push(ImageWithCoords {
                                    x: day_am_pm.x,
                                    y: day_am_pm.y,
                                    image_index: ImgId(image_index_amen.0),
                                });
                            }
                        } else {
                            if let Some(image_index_pmen) = &day_am_pm.image_index_pmen {
                                res.push(ImageWithCoords {
                                    x: day_am_pm.x,
                                    y: day_am_pm.y,
                                    image_index: ImgId(image_index_pmen.0),
                                });
                            }
                        }
                    }
                }
            }

            if let Some(params) = &params {
                if let Some(weekday) = params.weekday {
                    res.append(&mut &mut image_range_get_images(
                        &date.en_week_days,
                        weekday,
                        images,
                    ));
                }
            }
        }

        if let Some(weather) = &self.weather {
            if let Some(params) = &params {
                if let Some(icon) = &weather.icon {
                    if let Some(value) = params.weather {
                        res.append(&mut &mut image_range_get_images(
                            &icon.custom_icon,
                            value,
                            images,
                        ));
                    }
                }

                if let Some(temperature) = &weather.temperature {
                    if let Some(value) = params.temperature {
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
                                if let Some(value) = params.day_temperature {
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
                                if let Some(value) = params.night_temperature {
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

                if let Some(wind) = &weather.wind {
                    if let Some(value) = params.wind {
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
                                    image_index: ImgId(image_index.0),
                                });
                            }
                        }
                    }
                }
            }
        }

        if let Some(steps_progress) = &self.steps_progress {
            if let Some(params) = &params {
                if let Some(value) = params.steps_progress {
                    if let Some(linear) = &steps_progress.linear {
                        if let Some(start_image_index) = &linear.start_image_index {
                            let progress = (value as f32 / 100. * linear.segments.len() as f32)
                                .round() as usize;
                            for i in 0..progress {
                                res.push(ImageWithCoords {
                                    x: linear.segments[i].x,
                                    y: linear.segments[i].y,
                                    image_index: ImgId(start_image_index.0 + i as u32),
                                });
                            }
                        }
                    }

                    if let Some(line_scale) = &steps_progress.line_scale {
                        if let Some(images_count) = line_scale.images_count {
                            res.append(&mut &mut image_range_get_images(
                                &steps_progress.line_scale,
                                (value as f32 / 100. * (images_count - 1) as f32).round() as u32,
                                images,
                            ));
                        }
                    }
                }
            }
        }

        if let Some(battery) = &self.battery {
            if let Some(params) = &params {
                if let Some(battery_text) = &battery.battery_text {
                    if let Some(value) = params.battery {
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
                    if let Some(value) = params.battery {
                        if let Some(images_count) = battery_icon.images_count {
                            res.append(&mut image_range_get_images(
                                &battery.battery_icon,
                                (value as f32 / 100. * (images_count - 1) as f32).round() as u32,
                                images,
                            ));
                        }
                    }
                }

                if let Some(linear) = &battery.linear {
                    if let Some(start_image_index) = &linear.start_image_index {
                        if let Some(value) = params.battery {
                            let progress = (value as f32 / 100. * linear.segments.len() as f32)
                                .round() as usize;
                            for i in 0..progress {
                                res.push(ImageWithCoords {
                                    x: linear.segments[i].x,
                                    y: linear.segments[i].y,
                                    image_index: ImgId(start_image_index.0 + i as u32),
                                });
                            }
                        }
                    }
                }
            }
        }

        if let Some(other) = &self.other {
            if let Some(params) = &params {
                if let Some(value) = params.animation {
                    for animation in &other.animation.0 {
                        res.append(&mut &mut image_range_get_images(
                            &animation.animation_images,
                            value,
                            images,
                        ));
                    }
                }
            }
        }

        if let Some(status) = &self.status2 {
            if let Some(params) = &params {
                res.append(&mut status_image_get_images(
                    &status.do_not_disturb,
                    params.do_not_disturb,
                    images,
                ));
                res.append(&mut status_image_get_images(
                    &status.lock,
                    params.lock,
                    images,
                ));
                res.append(&mut status_image_get_images(
                    &status.bluetooth,
                    params.bluetooth,
                    images,
                ));
            }
        }

        res
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Background {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageReference>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "PreviewEN")]
    pub preview_en: Option<ImageReference>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "PreviewCN")]
    pub preview_cn: Option<ImageReference>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "PreviewCN2")]
    pub preview_cn2: Option<ImageReference>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Time {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<TimeNumbers>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes: Option<TimeNumbers>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds: Option<TimeNumbers>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter_image: Option<ImageReference>,
    #[wfrs_id(6)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delimiter_image: Option<ImageReference>,
    #[wfrs_id(7)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunset_time_number: Option<NumberInRect>,
    #[wfrs_id(8)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunset_time_delimiter_image_index: Option<ImgId>,
    #[wfrs_id(9)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunrise_time_number: Option<NumberInRect>,
    #[wfrs_id(10)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunrise_time_delimiter_image_index: Option<ImgId>,
    #[wfrs_id(11)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drawing_order: Option<bool>,
    #[wfrs_id(12)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunset_time_no_data_image: Option<ImageReference>,
    #[wfrs_id(13)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunrise_time_no_data_image: Option<ImageReference>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct TimeNumbers {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tens: Option<ImageRange>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ones: Option<ImageRange>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Activity {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Steps>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calories: Option<Calories>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pulse: Option<Pulse>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<Distance>,
    #[wfrs_id(6)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "PAI")]
    pub pai: Option<PAI>,
    #[wfrs_id(7)]
    pub unknown_v7: i32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Steps {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_image_index: Option<ImgId>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix_image_index: Option<ImgId>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Calories {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix_image_index: Option<ImgId>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Pulse {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_image_index: Option<ImgId>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_data_image_index: Option<ImgId>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix_image_index: Option<ImgId>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Distance {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub km_suffix_image_index: Option<ImgId>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_point_image_index: Option<ImgId>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miles_suffix_image_index: Option<ImgId>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct PAI {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Date {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month_and_day_and_year: Option<MonthAndDayAndYear>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_am_pm: Option<DayAmPm>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "ENWeekDays")]
    pub en_week_days: Option<ImageRange>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "CNWeekDays")]
    pub cn_week_days: Option<ImageRange>,
    #[wfrs_id(6)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "CN2WeekDays")]
    pub cn2_week_days: Option<ImageRange>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct MonthAndDayAndYear {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separate: Option<Separate>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_line: Option<OneLine>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_line_with_year: Option<OneLine>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_digits_month: Option<bool>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_digits_day: Option<bool>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Separate {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "MonthsEN")]
    pub months_en: Option<ImageRange>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "MonthsCN")]
    pub months_cn: Option<ImageRange>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<NumberInRect>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct OneLine {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter_image_index: Option<ImgId>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct DayAmPm {
    #[wfrs_id(1)]
    pub x: i32,
    #[wfrs_id(2)]
    pub y: i32,
    #[wfrs_id(3)]
    #[serde(rename = "ImageIndexAMCN", skip_serializing_if = "Option::is_none")]
    pub image_index_amcn: Option<ImgId>,
    #[wfrs_id(4)]
    #[serde(rename = "ImageIndexPMCN", skip_serializing_if = "Option::is_none")]
    pub image_index_pmcn: Option<ImgId>,
    #[wfrs_id(5)]
    #[serde(rename = "ImageIndexAMEN", skip_serializing_if = "Option::is_none")]
    pub image_index_amen: Option<ImgId>,
    #[wfrs_id(6)]
    #[serde(rename = "ImageIndexPMEN", skip_serializing_if = "Option::is_none")]
    pub image_index_pmen: Option<ImgId>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Status {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_not_disturb: Option<StatusImage>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<StatusImage>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluetooth: Option<StatusImage>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Weather {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Temperature>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_quality: Option<AirQuality>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub humidity: Option<Humidity>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind: Option<Wind>,
    #[wfrs_id(6)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "UVIndex")]
    pub uv_index: Option<UVIndex>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Icon {
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_icon: Option<ImageRange>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position1: Option<Coordinates>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position2: Option<Coordinates>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Temperature {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<TemperatureType>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub today: Option<Today>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Today {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separate: Option<TemperatureSeparate>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct TemperatureSeparate {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<TemperatureType>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub night: Option<TemperatureType>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct AirQuality {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageRange>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Humidity {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix_image_index: Option<ImgId>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pos_suffix: Option<ImageReference>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Wind {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "SuffixImageIndexEN")]
    pub suffix_image_index_en: Option<ImgId>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "SuffixImageIndexCN")]
    pub suffix_image_index_cn: Option<ImgId>,
    #[wfrs_id(4)]
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "SuffixImageIndexCN2"
    )]
    pub suffix_image_index_cn2: Option<ImgId>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "ImagePosSuffixEN")]
    pub image_pos_suffix_en: Option<ImageReference>,
    #[wfrs_id(6)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "ImagePosSuffixCN")]
    pub image_pos_suffix_cn: Option<ImageReference>,
    #[wfrs_id(7)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "ImagePosSuffixCN2")]
    pub image_pos_suffix_cn2: Option<ImageReference>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct UVIndex {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "UV")]
    pub uv: Option<UV>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "UVCN")]
    pub uvcn: Option<ImageRange>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "UVCN2")]
    pub uvcn2: Option<ImageRange>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct UV {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "UVCN")]
    pub suffix_image_index: Option<ImgId>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct StepsProgress {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_image: Option<ImageReference>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_scale: Option<ImageRange>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear: Option<Linear>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_scale: Option<CircleScale>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Battery {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_text: Option<BatteryText>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_icon: Option<ImageRange>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear: Option<Linear>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct BatteryText {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_image_index: Option<ImgId>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix_image_index: Option<ImgId>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Linear {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_image_index: Option<ImgId>,
    #[wfrs_id(2)]
    pub segments: Vec<Coordinates>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct AnalogDialFace {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<VectorShape>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes: Option<VectorShape>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds: Option<VectorShape>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
pub struct Animations(Vec<Animation>);

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Other {
    #[wfrs_id(1)]
    pub animation: Animations,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Animation {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_images: Option<ImageRange>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<u32>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_count: Option<u32>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_v4: Option<u32>,
}

impl Transform for Animations {
    fn transform(&mut self, params: &[Param]) {
        for i in 0..params.len() {
            let param = &params[i..=i]; // heh
            let mut animations = None;
            animations.transform(param);
            self.0.push(animations.unwrap());
        }
    }
}

impl Serialize for Animations {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.0.len() == 1 {
            self.0[0].serialize(serializer)
        } else {
            let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
            for element in &self.0 {
                seq.serialize_element(&element)?;
            }
            seq.end()
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct HeartProgress {
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_scale: Option<ImageRange>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear: Option<Linear>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_scale: Option<CircleScale>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct CircleScale {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center_x: Option<u32>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center_y: Option<u32>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_x: Option<u32>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_y: Option<u32>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_angle: Option<u32>,
    #[wfrs_id(6)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_angle: Option<u32>,
    #[wfrs_id(7)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[wfrs_id(8)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct WeekDaysIcons {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monday: Option<ImageReference>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuesday: Option<ImageReference>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wednesday: Option<ImageReference>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thursday: Option<ImageReference>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friday: Option<ImageReference>,
    #[wfrs_id(6)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturday: Option<ImageReference>,
    #[wfrs_id(7)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunday: Option<ImageReference>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct CaloriesProgress {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_image: Option<ImageReference>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_scale: Option<ImageRange>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear: Option<Linear>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Alarm {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter_image_index: Option<ImgId>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_image: Option<ImageReference>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_image: Option<ImageReference>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_data_image: Option<ImageReference>,
    #[wfrs_id(6)]
    unknown_v6: i32,
    #[wfrs_id(7)]
    unknown_v7: i32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct UnknownStruct {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_1: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_2: Option<NumberInRect>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_3: Option<ImgId>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_4: Option<ImgId>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_5: Option<ImgId>,
    #[wfrs_id(6)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_6: Option<ImgId>,
    #[wfrs_id(7)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_7: Option<ImgId>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct LunarDate {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<ImageRange>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<NumberInRect>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "DayOf0X")]
    pub day_of_0x: Option<ImgId>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "DayOf2X")]
    pub day_of_2x: Option<ImgId>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "DayOf10")]
    pub day_of_10: Option<ImgId>,
    #[wfrs_id(6)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "DayOf20")]
    pub day_of_20: Option<ImgId>,
    #[wfrs_id(7)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "DayOf30")]
    pub day_of_30: Option<ImgId>,
    #[wfrs_id(10)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "DayCN2")]
    pub day_cn2: Option<NumberInRect>,
}
