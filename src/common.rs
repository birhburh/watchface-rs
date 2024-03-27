use {
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, fmt::Debug},
};

pub type Params = HashMap<u8, Vec<Param>>;

#[derive(Debug, PartialEq)]
pub struct Watchface<T: WatchfaceParams> {
    pub parameters: T,
    pub images: Vec<Image>,
}
pub trait Transform {
    fn transform(&mut self, key: u8, params: &Vec<Param>);
}

pub trait WatchfaceParams: Transform {
    fn new() -> Self;
}

#[derive(Debug, PartialEq)]
pub enum Param {
    Number(i64),
    Float(f32),
    Child(Params),
}

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
pub struct ImageReference {
    pub x: i32,
    pub y: i32,
    pub image_index: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageRange {
    pub x: i32,
    pub y: i32,
    pub image_index: u32,
    pub images_count: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatusPosition {
    pub x: i32,
    pub y: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<Alignment>,
    #[serde(skip_serializing_if = "is_zero")]
    unknown4: u32,
    #[serde(skip_serializing_if = "is_zero")]
    unknown5: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatusImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<StatusPosition>,
    #[serde(skip_serializing_if = "is_zero")]
    pub on_image_index: u32,
    #[serde(skip_serializing_if = "is_zero")]
    pub off_image_index: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Alignment {
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
pub struct NumberInRect {
    pub top_left_x: i32,
    pub top_left_y: i32,
    pub bottom_right_x: i32,
    pub bottom_right_y: i32,
    pub alignment: Alignment,
    pub spacing_x: i32,
    pub spacing_y: i32,
    pub image_index: u32,
    pub images_count: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TemperatureType {
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<NumberInRect>,
    minus_image_index: u32,
    suffix_image_index: u32,
}

/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_zero(num: &u32) -> bool {
    *num == 0
}

pub fn parse_image_ref(param: &Param) -> Option<ImageReference> {
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
                image_ref.x = number_param_to_usize(value.get(0).unwrap()) as i32;
            }
            2 => {
                image_ref.y = number_param_to_usize(value.get(0).unwrap()) as i32;
            }
            3 => {
                image_ref.image_index = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            _ => (),
        }
    }
    Some(image_ref)
}

pub fn parse_image_range(param: &Param) -> Option<ImageRange> {
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
                image_range.x = number_param_to_usize(value.get(0).unwrap()) as i32;
            }
            2 => {
                image_range.y = number_param_to_usize(value.get(0).unwrap()) as i32;
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

pub fn parse_status_image(param: &Param) -> Option<StatusImage> {
    let mut status_image = StatusImage {
        ..Default::default()
    };

    let subvalue = match param {
        Param::Child(child) => child,
        _ => panic!("First param should be child param"),
    };

    for (key, value) in subvalue.into_iter() {
        match key {
            1 => {
                status_image.coordinates = parse_status_position(value.get(0).unwrap());
            }
            2 => {
                status_image.on_image_index = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            3 => {
                status_image.off_image_index = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            _ => (),
        }
    }
    Some(status_image)
}

pub fn parse_status_position(param: &Param) -> Option<StatusPosition> {
    let mut status_position = StatusPosition {
        ..Default::default()
    };

    let subvalue = match param {
        Param::Child(child) => child,
        _ => panic!("First param should be child param"),
    };

    for (key, value) in subvalue.into_iter() {
        match key {
            1 => {
                status_position.x = number_param_to_usize(value.get(0).unwrap()) as i32;
            }
            2 => {
                status_position.y = number_param_to_usize(value.get(0).unwrap()) as i32;
            }
            3 => {
                status_position.alignment =
                    match number_param_to_usize(value.get(0).unwrap()).try_into() {
                        Ok(v) => Some(v),
                        Err(_) => panic!("Wrong aligment"),
                    };
            }
            4 => {
                status_position.unknown4 = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            5 => {
                status_position.unknown5 = number_param_to_usize(value.get(0).unwrap()) as u32;
            }
            _ => (),
        }
    }
    Some(status_position)
}

pub fn parse_number_in_rect(param: &Param) -> Option<NumberInRect> {
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
                number_in_rect.top_left_x = number_param_to_usize(value.get(0).unwrap()) as i32;
            }
            2 => {
                number_in_rect.top_left_y = number_param_to_usize(value.get(0).unwrap()) as i32;
            }
            3 => {
                number_in_rect.bottom_right_x = number_param_to_usize(value.get(0).unwrap()) as i32;
            }
            4 => {
                number_in_rect.bottom_right_y = number_param_to_usize(value.get(0).unwrap()) as i32;
            }
            5 => {
                number_in_rect.alignment =
                    match number_param_to_usize(value.get(0).unwrap()).try_into() {
                        Ok(v) => v,
                        Err(_) => panic!("Wrong aligment"),
                    };
            }
            6 => {
                number_in_rect.spacing_x = number_param_to_usize(value.get(0).unwrap()) as i32;
            }
            7 => {
                number_in_rect.spacing_y = number_param_to_usize(value.get(0).unwrap()) as i32;
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

pub fn parse_bool(param: &Param) -> Option<bool> {
    let subvalue = match param {
        Param::Number(number) => number,
        _ => panic!("First param should be bytes param"),
    };

    Some(*subvalue != 0)
}

pub fn parse_coordinates(param: &Param) -> Option<Coordinates> {
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
                coordinates.x = number_param_to_usize(value.get(0).unwrap()) as i32;
            }
            2 => {
                coordinates.y = number_param_to_usize(value.get(0).unwrap()) as i32;
            }
            _ => (),
        }
    }
    Some(coordinates)
}

pub fn parse_temperature_type(param: &Param) -> Option<TemperatureType> {
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

pub fn number_param_to_usize(param: &Param) -> usize {
    if let Param::Number(number) = param {
        *number as usize
    } else {
        unreachable!();
    }
}
