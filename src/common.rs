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

impl Transform for Option<ImgId> {
    fn transform(&mut self, _key: u8, params: &[Param]) {
        let subvalue = match params.get(0).unwrap() {
            Param::Number(number) => number,
            _ => panic!("First param should be number param"),
        };

        *self = Some(*subvalue as ImgId);
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_index: Option<ImgId>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct ImageRange {
    #[wfrs_id(1)]
    pub x: i32,
    #[wfrs_id(2)]
    pub y: i32,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_index: Option<ImgId>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images_count: Option<u32>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    unknown4: Option<u32>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    unknown5: Option<u32>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct StatusImage {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<StatusPosition>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_image_index: Option<ImgId>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_image_index: Option<ImgId>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Coordinates {
    #[wfrs_id(1)]
    pub x: i32,
    #[wfrs_id(2)]
    pub y: i32,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[repr(u8)]
pub enum AlignmentInternal {
    Unknown = 0, // It probably wrong but I found it in one watchface
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

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[repr(u8)]
pub enum Alignment {
    Unknown = 0, // It probably wrong but I found it in one watchface
    Valid(AlignmentInternal),
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Valid(Default::default())
    }
}

impl Serialize for Alignment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Alignment::Unknown => serializer.serialize_u8(i64::from(Alignment::Unknown) as u8),
            Alignment::Valid(v) => serializer.serialize_str(&format!("{v:?}")),
        }
    }
}

impl From<Alignment> for i64 {
    fn from(v: Alignment) -> Self {
        match v {
            Alignment::Unknown => 0,
            Alignment::Valid(v) => v as i64,
        }
    }
}

impl TryFrom<i64> for Alignment {
    type Error = ();

    fn try_from(v: i64) -> Result<Self, Self::Error> {
        match v {
            x if x == Alignment::Unknown.try_into().unwrap() => Ok(Alignment::Unknown),
            x if x == AlignmentInternal::Left as i64 => {
                Ok(Alignment::Valid(AlignmentInternal::Left))
            }
            x if x == AlignmentInternal::Right as i64 => {
                Ok(Alignment::Valid(AlignmentInternal::Right))
            }
            x if x == AlignmentInternal::HCenter as i64 => {
                Ok(Alignment::Valid(AlignmentInternal::HCenter))
            }
            x if x == AlignmentInternal::Top as i64 => Ok(Alignment::Valid(AlignmentInternal::Top)),
            x if x == AlignmentInternal::Bottom as i64 => {
                Ok(Alignment::Valid(AlignmentInternal::Bottom))
            }
            x if x == AlignmentInternal::VCenter as i64 => {
                Ok(Alignment::Valid(AlignmentInternal::VCenter))
            }
            x if x == AlignmentInternal::TopLeft as i64 => {
                Ok(Alignment::Valid(AlignmentInternal::TopLeft))
            }
            x if x == AlignmentInternal::BottomLeft as i64 => {
                Ok(Alignment::Valid(AlignmentInternal::BottomLeft))
            }
            x if x == AlignmentInternal::CenterLeft as i64 => {
                Ok(Alignment::Valid(AlignmentInternal::CenterLeft))
            }
            x if x == AlignmentInternal::TopRight as i64 => {
                Ok(Alignment::Valid(AlignmentInternal::TopRight))
            }
            x if x == AlignmentInternal::BottomRight as i64 => {
                Ok(Alignment::Valid(AlignmentInternal::BottomRight))
            }
            x if x == AlignmentInternal::CenterRight as i64 => {
                Ok(Alignment::Valid(AlignmentInternal::CenterRight))
            }
            x if x == AlignmentInternal::TopCenter as i64 => {
                Ok(Alignment::Valid(AlignmentInternal::TopCenter))
            }
            x if x == AlignmentInternal::BottomCenter as i64 => {
                Ok(Alignment::Valid(AlignmentInternal::BottomCenter))
            }
            x if x == AlignmentInternal::Center as i64 => {
                Ok(Alignment::Valid(AlignmentInternal::Center))
            }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_index: Option<ImgId>,
    #[wfrs_id(9)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images_count: Option<u32>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct TemperatureType {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    minus_image_index: Option<ImgId>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix_image_index: Option<ImgId>,
}

/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_zero(num: &u32) -> bool {
    *num == 0
}
