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
    fn transform(&mut self, key: u8, params: &[Param]);
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

impl Transform for i32 {
    fn transform(&mut self, _key: u8, params: &[Param]) {
        let subvalue = match params.get(0).unwrap() {
            Param::Number(number) => number,
            _ => panic!("First param should be number param"),
        };

        *self = *subvalue as i32;
    }
}

impl Transform for usize {
    fn transform(&mut self, _key: u8, params: &[Param]) {
        let subvalue = match params.get(0).unwrap() {
            Param::Number(number) => number,
            _ => panic!("First param should be number param"),
        };

        *self = *subvalue as usize;
    }
}

impl Transform for Option<bool> {
    fn transform(&mut self, _key: u8, params: &[Param]) {
        let subvalue = match params.get(0).unwrap() {
            Param::Number(number) => number,
            _ => panic!("First param should be number param"),
        };

        *self = Some(*subvalue != 0);
    }
}

pub type ImgId = u32;

impl Transform for ImgId {
    fn transform(&mut self, _key: u8, params: &[Param]) {
        let subvalue = match params.get(0).unwrap() {
            Param::Number(number) => number,
            _ => panic!("First param should be number param"),
        };

        *self = *subvalue as ImgId;
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageReference {
    pub x: i32,
    pub y: i32,
    pub image_index: ImgId,
}

impl Transform for Option<ImageReference> {
    fn transform(&mut self, _key: u8, params: &[Param]) {
        match self {
            None => {
                *self = Some(ImageReference {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(image_ref) = self {
            for (key, value) in params.iter() {
                match key {
                    1 => image_ref.x.transform(*key, value),
                    2 => image_ref.y.transform(*key, value),
                    3 => image_ref.image_index.transform(*key, value),
                    _ => (),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageRange {
    pub x: i32,
    pub y: i32,
    pub image_index: ImgId,
    pub images_count: u32,
}

impl Transform for Option<ImageRange> {
    fn transform(&mut self, _key: u8, params: &[Param]) {
        match self {
            None => {
                *self = Some(ImageRange {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(image_range) = self {
            for (key, value) in params.iter() {
                match key {
                    1 => image_range.x.transform(*key, value),
                    2 => image_range.y.transform(*key, value),
                    3 => image_range.image_index.transform(*key, value),
                    4 => image_range.images_count.transform(*key, value),
                    _ => (),
                }
            }
        }
    }
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

impl Transform for Option<StatusPosition> {
    fn transform(&mut self, _key: u8, params: &[Param]) {
        match self {
            None => {
                *self = Some(StatusPosition {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(status_position) = self {
            for (key, value) in params.iter() {
                match key {
                    1 => status_position.x.transform(*key, value),
                    2 => status_position.y.transform(*key, value),
                    3 => status_position.alignment.transform(*key, value),
                    4 => status_position.unknown4.transform(*key, value),
                    5 => status_position.unknown5.transform(*key, value),
                    _ => (),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatusImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<StatusPosition>,
    #[serde(skip_serializing_if = "is_zero")]
    pub on_image_index: ImgId,
    #[serde(skip_serializing_if = "is_zero")]
    pub off_image_index: ImgId,
}

impl Transform for Option<StatusImage> {
    fn transform(&mut self, _key: u8, params: &[Param]) {
        match self {
            None => {
                *self = Some(StatusImage {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(status_image) = self {
            for (key, value) in params.iter() {
                match key {
                    1 => status_image.coordinates.transform(*key, value),
                    2 => status_image.on_image_index.transform(*key, value),
                    3 => status_image.off_image_index.transform(*key, value),
                    _ => (),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl Transform for Option<Coordinates> {
    fn transform(&mut self, _key: u8, params: &[Param]) {
        match self {
            None => {
                *self = Some(Coordinates {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(coordinates) = self {
            for (key, value) in params.iter() {
                match key {
                    1 => coordinates.x.transform(*key, value),
                    2 => coordinates.y.transform(*key, value),
                    _ => (),
                }
            }
        }
    }
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

impl TryFrom<i64> for Alignment {
    type Error = ();

    fn try_from(v: i64) -> Result<Self, Self::Error> {
        match v {
            x if x == Alignment::Left as i64 => Ok(Alignment::Left),
            x if x == Alignment::Right as i64 => Ok(Alignment::Right),
            x if x == Alignment::HCenter as i64 => Ok(Alignment::HCenter),
            x if x == Alignment::Top as i64 => Ok(Alignment::Top),
            x if x == Alignment::Bottom as i64 => Ok(Alignment::Bottom),
            x if x == Alignment::VCenter as i64 => Ok(Alignment::VCenter),
            x if x == Alignment::TopLeft as i64 => Ok(Alignment::TopLeft),
            x if x == Alignment::BottomLeft as i64 => Ok(Alignment::BottomLeft),
            x if x == Alignment::CenterLeft as i64 => Ok(Alignment::CenterLeft),
            x if x == Alignment::TopRight as i64 => Ok(Alignment::TopRight),
            x if x == Alignment::BottomRight as i64 => Ok(Alignment::BottomRight),
            x if x == Alignment::CenterRight as i64 => Ok(Alignment::CenterRight),
            x if x == Alignment::TopCenter as i64 => Ok(Alignment::TopCenter),
            x if x == Alignment::BottomCenter as i64 => Ok(Alignment::BottomCenter),
            x if x == Alignment::Center as i64 => Ok(Alignment::Center),
            _ => Err(()),
        }
    }
}

impl Transform for Option<Alignment> {
    fn transform(&mut self, _key: u8, params: &[Param]) {
        match self {
            None => {
                *self = Some(Default::default());
            }
            Some(_) => (),
        }

        let subvalue = match params.get(0).unwrap() {
            Param::Number(number) => number,
            _ => panic!("First param should be number param"),
        };

        if let Some(val) = self.as_mut() {
            match Alignment::try_from(*subvalue) {
                Ok(v) => {
                    *val = v;
                }
                Err(_) => panic!("Wrong aligment"),
            };
        }
    }
}
impl Transform for Alignment {
    fn transform(&mut self, _key: u8, params: &[Param]) {
        let subvalue = match params.get(0).unwrap() {
            Param::Number(number) => number,
            _ => panic!("First param should be number param"),
        };

        match Alignment::try_from(*subvalue) {
            Ok(v) => {
                *self = v;
            }
            Err(_) => panic!("Wrong aligment"),
        };
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
    pub image_index: ImgId,
    pub images_count: u32,
}

impl Transform for Option<NumberInRect> {
    fn transform(&mut self, _key: u8, params: &[Param]) {
        match self {
            None => {
                *self = Some(NumberInRect {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(number_in_rect) = self {
            for (key, value) in params.iter() {
                match key {
                    1 => number_in_rect.top_left_x.transform(*key, value),
                    2 => number_in_rect.top_left_y.transform(*key, value),
                    3 => number_in_rect.bottom_right_x.transform(*key, value),
                    4 => number_in_rect.bottom_right_y.transform(*key, value),
                    5 => number_in_rect.alignment.transform(*key, value),
                    6 => number_in_rect.spacing_x.transform(*key, value),
                    7 => number_in_rect.spacing_y.transform(*key, value),
                    8 => number_in_rect.image_index.transform(*key, value),
                    9 => number_in_rect.images_count.transform(*key, value),
                    _ => (),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TemperatureType {
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<NumberInRect>,
    minus_image_index: ImgId,
    suffix_image_index: ImgId,
}

impl Transform for Option<TemperatureType> {
    fn transform(&mut self, _key: u8, params: &[Param]) {
        match self {
            None => {
                *self = Some(TemperatureType {
                    ..Default::default()
                });
            }
            Some(_) => (),
        }

        let params = match params.get(0).unwrap() {
            Param::Child(child) => child,
            _ => panic!("First param should be child param"),
        };

        if let Some(temperature_type) = self {
            for (key, value) in params.iter() {
                match key {
                    1 => temperature_type.number.transform(*key, value),
                    2 => temperature_type.minus_image_index.transform(*key, value),
                    3 => temperature_type.suffix_image_index.transform(*key, value),
                    _ => (),
                }
            }
        }
    }
}

/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_zero(num: &u32) -> bool {
    *num == 0
}
