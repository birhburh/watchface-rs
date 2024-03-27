use {
    crate::common::*,
    serde::{Deserialize, Serialize},
    std::fmt::Debug,
};

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MiBandParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<Background>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<Time>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<Activity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weather: Option<Weather>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery: Option<Battery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Other>,
}

impl MiBandParams {
    fn parse_background(params: Params) -> Option<Background> {
        let mut background = Background {
            ..Default::default()
        };

        for (key, value) in params.into_iter() {
            match key {
                1 => {
                    background.image = Some(parse_image_ref(value.get(0).unwrap()));
                }
                3 => {
                    background.preview_en = Some(parse_image_ref(value.get(0).unwrap()));
                }
                4 => {
                    background.preview_cn = Some(parse_image_ref(value.get(0).unwrap()));
                }
                5 => {
                    background.preview_cn2 = Some(parse_image_ref(value.get(0).unwrap()));
                }
                _ => (),
            }
        }
        Some(background)
    }

    fn parse_time(params: Params) -> Option<Time> {
        let mut time = Time {
            ..Default::default()
        };

        for (key, value) in params.into_iter() {
            match key {
                1 => {
                    time.hours = Self::parse_time_numbers(value.get(0).unwrap());
                }
                2 => {
                    time.minutes = Self::parse_time_numbers(value.get(0).unwrap());
                }
                3 => {
                    time.seconds = Self::parse_time_numbers(value.get(0).unwrap());
                }
                11 => {
                    time.drawing_order = parse_bool(value.get(0).unwrap());
                }
                _ => (),
            }
        }
        Some(time)
    }

    fn parse_activity(params: Params) -> Option<Activity> {
        let mut activity = Activity {
            ..Default::default()
        };

        for (key, value) in params.into_iter() {
            match key {
                1 => {
                    activity.steps = Self::parse_steps(value.get(0).unwrap());
                }
                3 => {
                    activity.calories = Self::parse_calories(value.get(0).unwrap());
                }
                4 => {
                    activity.pulse = Self::parse_pulse(value.get(0).unwrap());
                }
                5 => {
                    activity.distance = Self::parse_distance(value.get(0).unwrap());
                }
                _ => (),
            }
        }
        Some(activity)
    }

    fn parse_date(params: Params) -> Option<Date> {
        let mut date = Date {
            ..Default::default()
        };

        for (key, value) in params.into_iter() {
            match key {
                1 => {
                    date.month_and_day_and_year =
                        Self::parse_month_and_day_and_year(value.get(0).unwrap());
                }
                2 => {
                    date.day_am_pm = Self::parse_day_am_pm(value.get(0).unwrap());
                }
                4 => {
                    date.en_week_days = parse_image_range(value.get(0).unwrap());
                }
                _ => (),
            }
        }
        Some(date)
    }

    fn parse_weather(params: Params) -> Option<Weather> {
        let mut weather = Weather {
            ..Default::default()
        };

        for (key, value) in params.into_iter() {
            match key {
                1 => {
                    weather.icon = Self::parse_icon(value.get(0).unwrap());
                }
                2 => {
                    weather.temperature = Self::parse_temperature(value.get(0).unwrap());
                }
                _ => (),
            }
        }
        Some(weather)
    }

    fn parse_battery(params: Params) -> Option<Battery> {
        let mut battery = Battery {
            ..Default::default()
        };

        for (key, value) in params.into_iter() {
            match key {
                1 => {
                    battery.battery_text = Self::parse_battery_text(value.get(0).unwrap());
                }
                2 => {
                    battery.battery_icon = parse_image_range(value.get(0).unwrap());
                }
                3 => {
                    battery.linear = Self::parse_linear(value.get(0).unwrap());
                }
                _ => (),
            }
        }
        Some(battery)
    }

    fn parse_status(params: Params) -> Option<Status> {
        let mut status = Status {
            ..Default::default()
        };

        for (key, value) in params.into_iter() {
            match key {
                1 => {
                    status.do_not_disturb = parse_status_image(value.get(0).unwrap());
                }
                2 => {
                    status.lock = parse_status_image(value.get(0).unwrap());
                }
                3 => {
                    status.bluetooth = parse_status_image(value.get(0).unwrap());
                }
                _ => (),
            }
        }
        Some(status)
    }

    fn parse_other(params: Params) -> Option<Other> {
        let mut other = Other {
            ..Default::default()
        };

        for (key, value) in params.into_iter() {
            match key {
                1 => {
                    other.animation = Self::parse_animation(value.get(0).unwrap());
                }
                _ => (),
            }
        }
        Some(other)
    }

    fn parse_steps(param: &Param) -> Option<Steps> {
        let mut steps = Steps {
            ..Default::default()
        };
        let subvalue = match param {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        for (key, value) in subvalue.into_iter() {
            match key {
                1 => {
                    steps.number = parse_number_in_rect(value.get(0).unwrap());
                }
                2 => {
                    steps.prefix_image_index = number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                3 => {
                    steps.suffix_image_index = number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                _ => (),
            }
        }
        Some(steps)
    }

    fn parse_calories(param: &Param) -> Option<Calories> {
        let mut calories = Calories {
            ..Default::default()
        };
        let subvalue = match param {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        for (key, value) in subvalue.into_iter() {
            match key {
                1 => {
                    calories.number = parse_number_in_rect(value.get(0).unwrap());
                }
                2 => {
                    calories.suffix_image_index =
                        number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                _ => (),
            }
        }
        Some(calories)
    }

    fn parse_pulse(param: &Param) -> Option<Pulse> {
        let mut pulse = Pulse {
            ..Default::default()
        };
        let subvalue = match param {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        for (key, value) in subvalue.into_iter() {
            match key {
                1 => {
                    pulse.number = parse_number_in_rect(value.get(0).unwrap());
                }
                2 => {
                    pulse.prefix_image_index = number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                3 => {
                    pulse.no_data_image_index = number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                4 => {
                    pulse.suffix_image_index = number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                _ => (),
            }
        }
        Some(pulse)
    }

    fn parse_distance(param: &Param) -> Option<Distance> {
        let mut distance = Distance {
            ..Default::default()
        };
        let subvalue = match param {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        for (key, value) in subvalue.into_iter() {
            match key {
                1 => {
                    distance.number = parse_number_in_rect(value.get(0).unwrap());
                }
                2 => {
                    distance.km_suffix_image_index =
                        number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                3 => {
                    distance.decimal_point_image_index =
                        number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                4 => {
                    distance.miles_suffix_image_index =
                        number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                _ => (),
            }
        }
        Some(distance)
    }

    fn parse_separate(param: &Param) -> Option<Separate> {
        let mut separate = Separate {
            ..Default::default()
        };
        let subvalue = match param {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        for (key, value) in subvalue.into_iter() {
            match key {
                1 => {
                    separate.month = parse_number_in_rect(value.get(0).unwrap());
                }
                4 => {
                    separate.day = parse_number_in_rect(value.get(0).unwrap());
                }
                _ => (),
            }
        }
        Some(separate)
    }

    fn parse_month_and_day_and_year(param: &Param) -> Option<MonthAndDayAndYear> {
        let mut month_and_day_and_year = MonthAndDayAndYear {
            ..Default::default()
        };
        let subvalue = match param {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        for (key, value) in subvalue.into_iter() {
            match key {
                1 => {
                    month_and_day_and_year.separate = Self::parse_separate(value.get(0).unwrap());
                }
                4 => {
                    month_and_day_and_year.two_digits_month = parse_bool(value.get(0).unwrap());
                }
                5 => {
                    month_and_day_and_year.two_digits_day = parse_bool(value.get(0).unwrap());
                }
                _ => (),
            }
        }
        Some(month_and_day_and_year)
    }

    fn parse_day_am_pm(param: &Param) -> Option<DayAmPm> {
        let mut day_am_pm = DayAmPm {
            ..Default::default()
        };
        let subvalue = match param {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        for (key, value) in subvalue.into_iter() {
            match key {
                1 => {
                    day_am_pm.x = number_param_to_usize(value.get(0).unwrap()) as i32;
                }
                2 => {
                    day_am_pm.y = number_param_to_usize(value.get(0).unwrap()) as i32;
                }
                3 => {
                    day_am_pm.image_index_amcn =
                        number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                4 => {
                    day_am_pm.image_index_pmcn =
                        number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                5 => {
                    day_am_pm.image_index_amen =
                        number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                6 => {
                    day_am_pm.image_index_pmen =
                        number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                _ => (),
            }
        }
        Some(day_am_pm)
    }

    fn parse_icon(param: &Param) -> Option<Icon> {
        let mut icon = Icon {
            ..Default::default()
        };
        let subvalue = match param {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        for (key, value) in subvalue.into_iter() {
            match key {
                2 => {
                    icon.custom_icon = parse_image_range(value.get(0).unwrap());
                }
                3 => {
                    icon.position1 = parse_coordinates(value.get(0).unwrap());
                }
                4 => {
                    icon.position2 = parse_coordinates(value.get(0).unwrap());
                }
                _ => (),
            }
        }
        Some(icon)
    }

    fn parse_segments(params: &Vec<Param>) -> Vec<Coordinates> {
        let mut result = vec![];

        for param in params {
            result.push(parse_coordinates(param).unwrap());
        }
        result
    }

    fn parse_animation(param: &Param) -> Option<Animation> {
        let mut animation = Animation {
            ..Default::default()
        };
        let subvalue = match param {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        for (key, value) in subvalue.into_iter() {
            match key {
                1 => {
                    animation.animation_images = parse_image_range(value.get(0).unwrap());
                }
                2 => {
                    animation.speed = number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                3 => {
                    animation.repeat_count = number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                4 => {
                    animation.unknown_v4 = number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                _ => (),
            }
        }
        Some(animation)
    }

    fn parse_temperature(param: &Param) -> Option<Temperature> {
        let mut temperature = Temperature {
            ..Default::default()
        };
        let subvalue = match param {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        for (key, value) in subvalue.into_iter() {
            match key {
                1 => {
                    temperature.current = parse_temperature_type(value.get(0).unwrap());
                }
                _ => (),
            }
        }
        Some(temperature)
    }

    fn parse_linear(param: &Param) -> Option<Linear> {
        let mut linear = Linear {
            ..Default::default()
        };
        let subvalue = match param {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        for (key, value) in subvalue.into_iter() {
            match key {
                1 => {
                    linear.start_image_index = number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                2 => {
                    linear.segments = Self::parse_segments(value);
                }
                _ => (),
            }
        }
        Some(linear)
    }

    fn parse_battery_text(param: &Param) -> Option<BatteryText> {
        let mut battery_text = BatteryText {
            ..Default::default()
        };
        let subvalue = match param {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        for (key, value) in subvalue.into_iter() {
            match key {
                1 => {
                    battery_text.number = parse_number_in_rect(value.get(0).unwrap());
                }
                3 => {
                    battery_text.prefix_image_index =
                        number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                4 => {
                    battery_text.suffix_image_index =
                        number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                _ => (),
            }
        }
        Some(battery_text)
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
                    numbers.tens = parse_image_range(value.get(0).unwrap());
                }
                2 => {
                    numbers.ones = parse_image_range(value.get(0).unwrap());
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
                self.background = Self::parse_background(params);
            }
            3 => {
                self.time = Self::parse_time(params);
            }
            4 => {
                self.activity = Self::parse_activity(params);
            }
            5 => {
                self.date = Self::parse_date(params);
            }
            6 => {
                self.weather = Self::parse_weather(params);
            }
            8 => {
                self.status = Self::parse_status(params);
            }
            9 => {
                self.battery = Self::parse_battery(params);
            }
            11 => {
                self.other = Self::parse_other(params);
            }
            _ => (),
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Background {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageReference>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PreviewEN")]
    pub preview_en: Option<ImageReference>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PreviewCN")]
    pub preview_cn: Option<ImageReference>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PreviewCN2")]
    pub preview_cn2: Option<ImageReference>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Time {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<TimeNumbers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes: Option<TimeNumbers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds: Option<TimeNumbers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drawing_order: Option<bool>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimeNumbers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tens: Option<ImageRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ones: Option<ImageRange>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Activity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Steps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calories: Option<Calories>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pulse: Option<Pulse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<Distance>,
    unknown_v7: i32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Steps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[serde(skip_serializing_if = "is_zero")]
    pub prefix_image_index: u32,
    #[serde(skip_serializing_if = "is_zero")]
    pub suffix_image_index: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Calories {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[serde(skip_serializing_if = "is_zero")]
    pub suffix_image_index: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Pulse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[serde(skip_serializing_if = "is_zero")]
    pub prefix_image_index: u32,
    #[serde(skip_serializing_if = "is_zero")]
    pub no_data_image_index: u32,
    #[serde(skip_serializing_if = "is_zero")]
    pub suffix_image_index: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Distance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[serde(skip_serializing_if = "is_zero")]
    pub km_suffix_image_index: u32,
    #[serde(skip_serializing_if = "is_zero")]
    pub decimal_point_image_index: u32,
    #[serde(skip_serializing_if = "is_zero")]
    pub miles_suffix_image_index: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Date {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month_and_day_and_year: Option<MonthAndDayAndYear>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_am_pm: Option<DayAmPm>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ENWeekDays")]
    pub en_week_days: Option<ImageRange>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MonthAndDayAndYear {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separate: Option<Separate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_digits_month: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_digits_day: Option<bool>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Separate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<NumberInRect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<NumberInRect>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DayAmPm {
    pub x: i32,
    pub y: i32,
    #[serde(rename = "ImageIndexAMCN")]
    pub image_index_amcn: u32,
    #[serde(rename = "ImageIndexPMCN")]
    pub image_index_pmcn: u32,
    #[serde(rename = "ImageIndexAMEN")]
    pub image_index_amen: u32,
    #[serde(rename = "ImageIndexPMEN")]
    pub image_index_pmen: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Status {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_not_disturb: Option<StatusImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<StatusImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluetooth: Option<StatusImage>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Weather {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Temperature>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Icon {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_icon: Option<ImageRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position1: Option<Coordinates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position2: Option<Coordinates>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Temperature {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<TemperatureType>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Battery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_text: Option<BatteryText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_icon: Option<ImageRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear: Option<Linear>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BatteryText {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[serde(skip_serializing_if = "is_zero")]
    pub prefix_image_index: u32,
    #[serde(skip_serializing_if = "is_zero")]
    pub suffix_image_index: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Linear {
    pub start_image_index: u32,
    pub segments: Vec<Coordinates>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Other {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Animation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_images: Option<ImageRange>,
    pub speed: u32,
    pub repeat_count: u32,
    pub unknown_v4: u32,
}
