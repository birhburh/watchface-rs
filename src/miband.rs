use {
    crate::common::*, // TODO: not use star
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

impl WatchfaceParams for MiBandParams {
    fn new() -> Self {
        MiBandParams {
            ..Default::default()
        }
    }
}

impl Transform for MiBandParams {
    fn transform(&mut self, key: u8, params: &Vec<Param>) {
        match key {
            2 => self.background.transform(key, params),
            3 => self.time.transform(key, params),
            4 => self.activity.transform(key, params),
            5 => self.date.transform(key, params),
            6 => self.weather.transform(key, params),
            8 => self.status.transform(key, params),
            9 => self.battery.transform(key, params),
            11 => self.other.transform(key, params),
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

impl Transform for Option<Background> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(Background {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(background) = self {
            for (key, value) in params.into_iter() {
                match key {
                    1 => {
                        background.image = parse_image_ref(value.get(0).unwrap());
                    }
                    3 => {
                        background.preview_en = parse_image_ref(value.get(0).unwrap());
                    }
                    4 => {
                        background.preview_cn = parse_image_ref(value.get(0).unwrap());
                    }
                    5 => {
                        background.preview_cn2 = parse_image_ref(value.get(0).unwrap());
                    }
                    _ => (),
                }
            }
        }
    }
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

impl Transform for Option<Time> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(Time {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(time) = self {
            for (key, value) in params.into_iter() {
                match key {
                    1 => time.hours.transform(*key, value),
                    2 => time.minutes.transform(*key, value),
                    3 => time.seconds.transform(*key, value),
                    11 => {
                        time.drawing_order = parse_bool(value.get(0).unwrap());
                    }
                    _ => (),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimeNumbers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tens: Option<ImageRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ones: Option<ImageRange>,
}

impl Transform for Option<TimeNumbers> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(TimeNumbers {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(numbers) = self {
            for (key, value) in params.into_iter() {
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
        }
    }
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

impl Transform for Option<Activity> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(Activity {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(activity) = self {
            for (key, value) in params.into_iter() {
                match key {
                    1 => activity.steps.transform(*key, value),
                    3 => activity.calories.transform(*key, value),
                    4 => activity.pulse.transform(*key, value),
                    5 => activity.distance.transform(*key, value),
                    _ => (),
                }
            }
        }
    }
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

impl Transform for Option<Steps> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(Steps {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(steps) = self {
            for (key, value) in params.into_iter() {
                match key {
                    1 => {
                        steps.number = parse_number_in_rect(value.get(0).unwrap());
                    }
                    2 => {
                        steps.prefix_image_index =
                            number_param_to_usize(value.get(0).unwrap()) as u32;
                    }
                    3 => {
                        steps.suffix_image_index =
                            number_param_to_usize(value.get(0).unwrap()) as u32;
                    }
                    _ => (),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Calories {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[serde(skip_serializing_if = "is_zero")]
    pub suffix_image_index: u32,
}

impl Transform for Option<Calories> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(Calories {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(calories) = self {
            for (key, value) in params.into_iter() {
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
        }
    }
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

impl Transform for Option<Pulse> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(Pulse {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(pulse) = self {
            for (key, value) in params.into_iter() {
                match key {
                    1 => {
                        pulse.number = parse_number_in_rect(value.get(0).unwrap());
                    }
                    2 => {
                        pulse.prefix_image_index =
                            number_param_to_usize(value.get(0).unwrap()) as u32;
                    }
                    3 => {
                        pulse.no_data_image_index =
                            number_param_to_usize(value.get(0).unwrap()) as u32;
                    }
                    4 => {
                        pulse.suffix_image_index =
                            number_param_to_usize(value.get(0).unwrap()) as u32;
                    }
                    _ => (),
                }
            }
        }
    }
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

impl Transform for Option<Distance> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(Distance {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(distance) = self {
            for (key, value) in params.into_iter() {
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
        }
    }
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

impl Transform for Option<Date> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(Date {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(date) = self {
            for (key, value) in params.into_iter() {
                match key {
                    1 => {
                        date.month_and_day_and_year.transform(*key, value);
                    }
                    2 => {
                        date.day_am_pm.transform(*key, value);
                    }
                    4 => {
                        date.en_week_days = parse_image_range(value.get(0).unwrap());
                    }
                    _ => (),
                }
            }
        }
    }
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

impl Transform for Option<MonthAndDayAndYear> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(MonthAndDayAndYear {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(month_and_day_and_year) = self {
            for (key, value) in params.into_iter() {
                match key {
                    1 => {
                        month_and_day_and_year.separate.transform(*key, value);
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
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Separate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<NumberInRect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<NumberInRect>,
}

impl Transform for Option<Separate> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(Separate {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(separate) = self {
            for (key, value) in params.into_iter() {
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
        }
    }
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

impl Transform for Option<DayAmPm> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(DayAmPm {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(day_am_pm) = self {
            for (key, value) in params.into_iter() {
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
        }
    }
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

impl Transform for Option<Status> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(Status {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(status) = self {
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
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Weather {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Temperature>,
}

impl Transform for Option<Weather> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(Weather {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(weather) = self {
            for (key, value) in params.into_iter() {
                match key {
                    1 => weather.icon.transform(*key, value),
                    2 => weather.temperature.transform(*key, value),
                    _ => (),
                }
            }
        }
    }
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

impl Transform for Option<Icon> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(Icon {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(icon) = self {
            for (key, value) in params.into_iter() {
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
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Temperature {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<TemperatureType>,
}

impl Transform for Option<Temperature> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(Temperature {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(temperature) = self {
            for (key, value) in params.into_iter() {
                match key {
                    1 => {
                        temperature.current = parse_temperature_type(value.get(0).unwrap());
                    }
                    _ => (),
                }
            }
        }
    }
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

impl Transform for Option<Battery> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(Battery {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(battery) = self {
            for (key, value) in params.into_iter() {
                match key {
                    1 => battery.battery_text.transform(*key, value),
                    2 => {
                        battery.battery_icon = parse_image_range(value.get(0).unwrap());
                    }
                    3 => {
                        battery.linear.transform(*key, value);
                    }
                    _ => (),
                }
            }
        }
    }
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

impl Transform for Option<BatteryText> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(BatteryText {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(battery_text) = self {
            for (key, value) in params.into_iter() {
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
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Linear {
    pub start_image_index: u32,
    pub segments: Segments,
}

impl Transform for Option<Linear> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(Linear {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(linear) = self {
            for (key, value) in params.into_iter() {
                match key {
                    1 => {
                        linear.start_image_index =
                            number_param_to_usize(value.get(0).unwrap()) as u32;
                    }
                    2 => {
                        linear.segments.transform(*key, value);
                    }
                    _ => (),
                }
            }
        }
    }
}

type Segments = Vec<Coordinates>;

impl Transform for Segments {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        for param in params {
            self.push(parse_coordinates(param).unwrap());
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Other {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
}

impl Transform for Option<Other> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(Other {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(other) = self {
            for (key, value) in params.into_iter() {
                match key {
                    1 => other.animation.transform(*key, value),
                    _ => (),
                }
            }
        }
    }
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

impl Transform for Option<Animation> {
    fn transform(&mut self, _key: u8, params: &Vec<Param>) {
        match self {
            None => {
                *self = Some(Animation {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(animation) = self {
            for (key, value) in params.into_iter() {
                match key {
                    1 => {
                        animation.animation_images = parse_image_range(value.get(0).unwrap());
                    }
                    2 => {
                        animation.speed = number_param_to_usize(value.get(0).unwrap()) as u32;
                    }
                    3 => {
                        animation.repeat_count =
                            number_param_to_usize(value.get(0).unwrap()) as u32;
                    }
                    4 => {
                        animation.unknown_v4 = number_param_to_usize(value.get(0).unwrap()) as u32;
                    }
                    _ => (),
                }
            }
        }
    }
}
