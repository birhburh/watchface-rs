use {
    serde::{Deserialize, Serialize},
    std::{
        collections::{hash_map::Entry, HashMap},
        fmt::Debug,
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
pub struct Image {
    pub pixels: Vec<u8>,
    pub width: u16,
    pub height: u16,
    pub bits_per_pixel: u16,
    pub pixel_format: u16,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ImageReference {
    x: u32,
    y: u32,
    image_index: u32,
}
#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ImageRange {
    x: u32,
    y: u32,
    image_index: u32,
    images_count: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Background {
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<ImageReference>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PreviewEN")]
    preview_en: Option<ImageReference>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PreviewCN")]
    preview_cn: Option<ImageReference>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PreviewCN2")]
    preview_cn2: Option<ImageReference>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TimeNumbers {
    #[serde(skip_serializing_if = "Option::is_none")]
    tens: Option<ImageRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ones: Option<ImageRange>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Time {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<TimeNumbers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<TimeNumbers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<TimeNumbers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drawing_order: Option<bool>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Separate {
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<NumberInRect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<NumberInRect>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct MonthAndDayAndYear {
    #[serde(skip_serializing_if = "Option::is_none")]
    separate: Option<Separate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    two_digits_month: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    two_digits_day: Option<bool>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DayAmPm {
    x: u32,
    y: u32,
    #[serde(rename = "ImageIndexAMCN")]
    image_index_amcn: u32,
    #[serde(rename = "ImageIndexPMCN")]
    image_index_pmcn: u32,
    #[serde(rename = "ImageIndexAMEN")]
    image_index_amen: u32,
    #[serde(rename = "ImageIndexPMEN")]
    image_index_pmen: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Date {
    #[serde(skip_serializing_if = "Option::is_none")]
    month_and_day_and_year: Option<MonthAndDayAndYear>,
    #[serde(skip_serializing_if = "Option::is_none")]
    day_am_pm: Option<DayAmPm>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ENWeekDays")]
    en_week_days: Option<ImageRange>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Coordinates {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Icon {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_icon: Option<ImageRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position1: Option<Coordinates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position2: Option<Coordinates>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TemperatureType {
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<NumberInRect>,
    minus_image_index: u32,
    suffix_image_index: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Temperature {
    #[serde(skip_serializing_if = "Option::is_none")]
    current: Option<TemperatureType>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Weather {
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<Temperature>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Other {
    #[serde(skip_serializing_if = "Option::is_none")]
    animation: Option<Animation>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct BatteryText {
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<NumberInRect>,
    #[serde(skip_serializing_if = "is_zero")]
    prefix_image_index: u32,
    #[serde(skip_serializing_if = "is_zero")]
    suffix_image_index: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Linear {
    start_image_index: u32,
    segments: Vec<Coordinates>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Battery {
    #[serde(skip_serializing_if = "Option::is_none")]
    battery_text: Option<BatteryText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linear: Option<Linear>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Animation {
    #[serde(skip_serializing_if = "Option::is_none")]
    animation_images: Option<ImageRange>,
    speed: u32,
    repeat_count: u32,
    unknown_v4: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
enum Alignment {
    Left = 2,
    Right = 4,
    HCenter = 8,
    Top = 16,
    Bottom = 32,
    VCenter = 64,
    TopLeft = 18,
    BottomLeft = 34,
    CenterLeft = 66,
    TopRight = 20,
    BottomRight = 36,
    CenterRight = 68,
    TopCenter = 24,
    BottomCenter = 40,
    #[default]
    Center = 72,
}

impl TryFrom<usize> for Alignment {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == Alignment::Left as usize => Ok(Alignment::Left),
            x if x == Alignment::Right as usize => Ok(Alignment::Right),
            x if x == Alignment::HCenter as usize => Ok(Alignment::HCenter),
            x if x == Alignment::Top as usize => Ok(Alignment::Top),
            x if x == Alignment::Bottom as usize => Ok(Alignment::Bottom),
            x if x == Alignment::VCenter as usize => Ok(Alignment::VCenter),
            x if x == Alignment::TopLeft as usize => Ok(Alignment::TopLeft),
            x if x == Alignment::BottomLeft as usize => Ok(Alignment::BottomLeft),
            x if x == Alignment::CenterLeft as usize => Ok(Alignment::CenterLeft),
            x if x == Alignment::TopRight as usize => Ok(Alignment::TopRight),
            x if x == Alignment::BottomRight as usize => Ok(Alignment::BottomRight),
            x if x == Alignment::CenterRight as usize => Ok(Alignment::CenterRight),
            x if x == Alignment::TopCenter as usize => Ok(Alignment::TopCenter),
            x if x == Alignment::BottomCenter as usize => Ok(Alignment::BottomCenter),
            x if x == Alignment::Center as usize => Ok(Alignment::Center),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct NumberInRect {
    top_left_x: u32,
    top_left_y: u32,
    bottom_right_x: u32,
    bottom_right_y: u32,
    alignment: Alignment,
    spacing_x: u32,
    spacing_y: u32,
    image_index: u32,
    images_count: u32,
}

/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero(num: &u32) -> bool {
    *num == 0
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Steps {
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<NumberInRect>,
    #[serde(skip_serializing_if = "is_zero")]
    prefix_image_index: u32,
    #[serde(skip_serializing_if = "is_zero")]
    suffix_image_index: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Calories {
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<NumberInRect>,
    #[serde(skip_serializing_if = "is_zero")]
    suffix_image_index: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Pulse {
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<NumberInRect>,
    #[serde(skip_serializing_if = "is_zero")]
    prefix_image_index: u32,
    #[serde(skip_serializing_if = "is_zero")]
    no_data_image_index: u32,
    #[serde(skip_serializing_if = "is_zero")]
    suffix_image_index: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Distance {
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<NumberInRect>,
    km_suffix_image_index: u32,
    decimal_point_image_index: u32,
    miles_suffix_image_index: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Activity {
    #[serde(skip_serializing_if = "Option::is_none")]
    steps: Option<Steps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    calories: Option<Calories>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pulse: Option<Pulse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    distance: Option<Distance>,
    unknown_v7: i32,
}

pub type Params = HashMap<u8, Vec<Param>>;

#[derive(Debug, PartialEq)]
pub enum Param {
    Number(i64),
    Float(f32),
    Child(Params),
}

pub trait WatchfaceParams {
    fn new() -> Self;
    fn append(&mut self, key: u8, parameters: Params);
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MiBandParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    background: Option<Background>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<Time>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activity: Option<Activity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weather: Option<Weather>,
    #[serde(skip_serializing_if = "Option::is_none")]
    battery: Option<Battery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    other: Option<Other>,
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
                image_ref.x = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            2 => {
                image_ref.y = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            3 => {
                image_ref.image_index = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            _ => (),
        }
    }
    image_ref
}

fn parse_image_range(param: &Param) -> Option<ImageRange> {
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
                image_range.x = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            2 => {
                image_range.y = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            3 => {
                image_range.image_index = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            4 => {
                image_range.images_count = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            _ => (),
        }
    }
    Some(image_range)
}

fn parse_number_in_rect(param: &Param) -> Option<NumberInRect> {
    let mut number_in_rect = NumberInRect {
        ..Default::default()
    };

    let subvalue = match param {
        Param::Child(child) => child,
        _ => panic!("First param should be child param"),
    };

    for (key, value) in subvalue.into_iter() {
        match key {
            1 => {
                number_in_rect.top_left_x = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            2 => {
                number_in_rect.top_left_y = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            3 => {
                number_in_rect.bottom_right_x = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            4 => {
                number_in_rect.bottom_right_y = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            5 => {
                number_in_rect.alignment =
                    match number_param_to_usize(value.get(0).unwrap()).try_into() {
                        Ok(v) => v,
                        Err(_) => panic!("Wrong aligment"),
                    };
            }
            6 => {
                number_in_rect.spacing_x = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            7 => {
                number_in_rect.spacing_y = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            8 => {
                number_in_rect.image_index = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            9 => {
                number_in_rect.images_count = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            _ => (),
        }
    }
    Some(number_in_rect)
}

fn parse_bool(param: &Param) -> Option<bool> {
    let subvalue = match param {
        Param::Number(number) => number,
        _ => panic!("First param should be bytes param"),
    };

    Some(*subvalue != 0)
}

fn parse_coordinates(param: &Param) -> Option<Coordinates> {
    let mut coordinates = Coordinates {
        ..Default::default()
    };

    let subvalue = match param {
        Param::Child(child) => child,
        _ => panic!("First param should be child param"),
    };

    for (key, value) in subvalue.into_iter() {
        match key {
            1 => {
                coordinates.x = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            2 => {
                coordinates.y = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            _ => (),
        }
    }
    Some(coordinates)
}

fn parse_temperature_type(param: &Param) -> Option<TemperatureType> {
    let mut temperature_type = TemperatureType {
        ..Default::default()
    };

    let subvalue = match param {
        Param::Child(child) => child,
        _ => panic!("First param should be child param"),
    };

    for (key, value) in subvalue.into_iter() {
        match key {
            1 => {
                temperature_type.number = parse_number_in_rect(value.get(0).unwrap());
            }
            2 => {
                temperature_type.minus_image_index =
                    number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            3 => {
                temperature_type.suffix_image_index =
                    number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            _ => (),
        }
    }
    Some(temperature_type)
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
                3 => {
                    battery.linear = Self::parse_linear(value.get(0).unwrap());
                }
                _ => (),
            }
        }
        Some(battery)
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
                    day_am_pm.x = number_param_to_usize(value.get(0).unwrap()) as u32;
                }
                2 => {
                    day_am_pm.y = number_param_to_usize(value.get(0).unwrap()) as u32;
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

#[derive(Debug, PartialEq)]
pub struct Watchface<T: WatchfaceParams> {
    pub parameters: T,
    pub images: Vec<Image>,
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
            break;
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
        let (field_value, value_size) = variable_width_value_parser(i)?;

        if has_child {
            // When node has Child, field value is size of Child

            let child_size = field_value as usize;
            if child_size <= 0 {
                panic!("Child size of 0 or less");
            }
            // Recursive call to read Child data
            let child = params_parser(i, child_size)?;
            value = Param::Child(child);
        } else {
            value = Param::Number(field_value);
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

    let mut palette_size = 0;
    let mut palette = vec![];

    if palette_colors_count > 0 {
        // Read palette
        for color_number in 0..palette_colors_count {
            let color = (
                u8.parse_next(i)?,
                u8.parse_next(i)?,
                u8.parse_next(i)?,
                if (transparent_palette_color != 0) && (color_number == transparent_palette_color - 1) {
                    0xFF
                } else {
                    0x00
                },
            );
            u8.parse_next(i)?;
            palette.push(color);
        }

        palette_size = palette_colors_count * 4;
    }

    dbg!(&palette);
    dbg!(palette_size);

    let mut prev_byte = -1;
    let mut val = 0;
    // Read pixel data
    let mut pixels = vec![0; 4usize * width as usize * height as usize];
    for y in 0..height {
        for x in 0..width {
            // read pixel color info
            let red;
            let green;
            let blue;
            let mut alpha = 0x00;
            if palette_colors_count != 0 {
                let color_id;
                if bits_per_pixel < 8 {
                    let pixels_per_byte = 8 / bits_per_pixel;

                    let new_byte = ((x / pixels_per_byte) as f32).floor() as i32;
                    let byte = if new_byte > prev_byte {
                        prev_byte = new_byte;
                        val = u8.parse_next(i)?;
                        val
                    } else {
                        val
                    };
                    let bit_mask = (1 << bits_per_pixel) - 1;
                    let bit_position = 8 - ((x % pixels_per_byte) + 1) * bits_per_pixel;
                    color_id = (byte >> bit_position) & bit_mask;
                } else {
                    color_id = u8.parse_next(i)?;
                }
                (red, green, blue, alpha) = palette[color_id as usize];
            } else {
                let byte_per_pixel = bits_per_pixel / 8;

                if byte_per_pixel == 4 {
                    red = u8.parse_next(i)?;
                    green = u8.parse_next(i)?;
                    blue = u8.parse_next(i)?;
                    alpha = u8.parse_next(i)?;
                } else {
                    let rgba;
                    if byte_per_pixel == 3 {
                        // 24 bits is 16 bit color data (big endian) with 8 bit alpha
                        alpha = u8.parse_next(i)?;
                        rgba = be_u16.parse_next(i)?;
                    } else {
                        // for the 16 bit images, the value is little endian
                        rgba = le_u16.parse_next(i)?;
                    }
                    if pixel_format == 0x13 {
                        // color is 16 bit (4:4:4:4) abgr
                        alpha = ((rgba & 0xF000) >> 8) as u8;
                        blue = ((rgba & 0x0F00) >> 4) as u8;
                        green = (rgba & 0x00F0) as u8;
                        red = ((rgba & 0x000F) << 4) as u8;
                    } else if pixel_format == 0x1C || pixel_format == 0x09 {
                        // color is 16bit (5:6:5) rgb
                        red = ((rgba & 0xF800) >> 8) as u8;
                        green = ((rgba & 0x07E0) >> 3) as u8;
                        blue = ((rgba & 0x001F) << 3) as u8;
                    } else {
                        // color is 16bit (5:6:5) bgr
                        blue = ((rgba & 0xF800) >> 8) as u8;
                        green = ((rgba & 0x07E0) >> 3) as u8;
                        red = ((rgba & 0x001F) << 3) as u8;
                    }
                }
            }

            let pixel_position = (y as usize * width as usize + x as usize) * 4;
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

fn number_param_to_usize(param: &Param) -> usize {
    if let Param::Number(number) = param {
        *number as usize
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

    let parameters_size = number_param_to_usize(&first_parameter.get(&1).unwrap()[0]);
    let images_count = number_param_to_usize(&first_parameter.get(&2).unwrap()[0]);

    let mut parameters = T::new();

    let params_start = i.checkpoint();

    // TODO: remove sort, it is needed only for debuging and comparing to watchface-js
    let mut keys = parameter_info.keys().into_iter().collect::<Vec<_>>();
    keys.sort();
    for key in keys {
        let value = parameter_info.get(key).unwrap();
        if *key == 1 {
            continue;
        }

        i.reset(&params_start);

        let subvalue = match value.get(0).unwrap() {
            Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        let offset = number_param_to_usize(&subvalue.get(&1).unwrap()[0]);
        let size = number_param_to_usize(&subvalue.get(&2).unwrap()[0]);

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

pub fn parse_watch_face_bin<T: WatchfaceParams>(bytes: &mut &[u8]) -> PResult<Watchface<T>> {
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
