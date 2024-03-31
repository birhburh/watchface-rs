use {
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, fmt::Debug},
    watchface_rs_derive::TransformDerive,
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

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct ImageReference {
    #[wfrs_id(1)]
    pub x: i32,
    #[wfrs_id(2)]
    pub y: i32,
    #[wfrs_id(3)]
    pub image_index: ImgId,
    // #[wfrs_id(4)]
    // pub z: f64,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct ImageRange {
    #[wfrs_id(1)]
    pub x: i32,
    #[wfrs_id(2)]
    pub y: i32,
    #[wfrs_id(3)]
    pub image_index: ImgId,
    #[wfrs_id(4)]
    pub images_count: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct StatusPosition {
    #[wfrs_id(1)]
    pub x: i32,
    #[wfrs_id(2)]
    pub y: i32,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<Alignment>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "is_zero")]
    unknown4: u32,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "is_zero")]
    unknown5: u32,
}
#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct StatusImage {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<StatusPosition>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "is_zero")]
    pub on_image_index: ImgId,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "is_zero")]
    pub off_image_index: ImgId,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Coordinates {
    #[wfrs_id(1)]
    pub x: i32,
    #[wfrs_id(2)]
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

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct NumberInRect {
    #[wfrs_id(1)]
    pub top_left_x: i32,
    #[wfrs_id(2)]
    pub top_left_y: i32,
    #[wfrs_id(3)]
    pub bottom_right_x: i32,
    #[wfrs_id(4)]
    pub bottom_right_y: i32,
    #[wfrs_id(5)]
    pub alignment: Alignment,
    #[wfrs_id(6)]
    pub spacing_x: i32,
    #[wfrs_id(7)]
    pub spacing_y: i32,
    #[wfrs_id(8)]
    pub image_index: ImgId,
    #[wfrs_id(9)]
    pub images_count: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct TemperatureType {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<NumberInRect>,
    #[wfrs_id(2)]
    minus_image_index: ImgId,
    #[wfrs_id(3)]
    suffix_image_index: ImgId,
}

/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_zero(num: &u32) -> bool {
    *num == 0
}
