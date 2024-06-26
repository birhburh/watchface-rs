use {
    crate::preview::Preview,
    derive::TransformDerive,
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, fmt::Debug},
};

pub type Params = HashMap<u8, Vec<Param>>;

#[derive(Debug, PartialEq)]
pub struct Watchface<T>
where
    T: WatchfaceParams,
    Option<T>: Transform + Preview,
{
    pub parameters: Option<T>,
    pub images: Vec<Image>,
}

pub trait Transform {
    fn transform(&mut self, params: &[Param]);
}

impl<T> Watchface<T>
where
    T: WatchfaceParams,
    Option<T>: Transform + Preview,
{
    pub fn generate_preview(&self, params: Option<PreviewParams>) -> Vec<ImageWithCoords> {
        self.parameters.get_images(&params, &vec![], &self.images)
    }
}

pub trait WatchfaceParams {}

#[derive(Debug, PartialEq)]
pub enum ImageType {
    Id(ImgId),
    Image(Image),
}

#[derive(Debug, PartialEq)]
pub struct ImageWithCoords {
    pub x: i32,
    pub y: i32,
    pub image_type: ImageType,
}

#[derive(Debug, Default)]
pub struct PreviewParams {
    pub hours: Option<u32>,
    pub minutes: Option<u32>,
    pub seconds: Option<u32>,
    pub time12h: bool,
    pub am: bool,
    pub month: Option<u32>,
    pub day: Option<u32>,
    pub weekday: Option<u32>,

    pub steps: Option<u32>,
    pub steps_progress: Option<u32>,
    pub distance: Option<f32>,
    pub pulse: Option<u32>,
    pub heart_progress: Option<u32>,
    pub calories: Option<u32>,
    pub calories_progress: Option<u32>,
    pub pai: Option<u32>,

    pub weather: Option<u32>,
    pub temperature: Option<i32>,
    pub day_temperature: Option<i32>,
    pub night_temperature: Option<i32>,
    pub humidity: Option<i32>,
    pub wind: Option<i32>,
    pub uv: Option<i32>,

    pub battery: Option<u32>,
    pub do_not_disturb: bool,
    pub lock: bool,
    pub bluetooth: bool,
    pub alarm_hours: Option<u32>,
    pub alarm_minutes: Option<u32>,
    pub alarm_on: bool,

    pub animation: Option<u32>,
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
    fn transform(&mut self, params: &[Param]) {
        let subvalue = match params.get(0).unwrap() {
            Param::Number(number) => number,
            _ => panic!("First param should be number param"),
        };

        *self = *subvalue as i32;
    }
}

impl Transform for usize {
    fn transform(&mut self, params: &[Param]) {
        let subvalue = match params.get(0).unwrap() {
            Param::Number(number) => number,
            _ => panic!("First param should be number param"),
        };

        *self = *subvalue as usize;
    }
}

impl Transform for Option<u32> {
    fn transform(&mut self, params: &[Param]) {
        let subvalue = match params.get(0).unwrap() {
            Param::Number(number) => number,
            _ => panic!("First param should be number param"),
        };

        *self = Some(*subvalue as u32);
    }
}

impl Transform for Option<bool> {
    fn transform(&mut self, params: &[Param]) {
        let subvalue = match params.get(0).unwrap() {
            Param::Number(number) => number,
            _ => panic!("First param should be number param"),
        };

        *self = Some(*subvalue != 0);
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ImgId(pub u32);

impl Transform for Option<ImgId> {
    fn transform(&mut self, params: &[Param]) {
        let subvalue = match params.get(0).unwrap() {
            Param::Number(number) => number,
            _ => panic!("First param should be number param"),
        };

        *self = Some(ImgId(*subvalue as u32));
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct ImageReference {
    #[wfrs(id = 1)]
    pub x: i32,
    #[wfrs(id = 2)]
    pub y: i32,
    #[wfrs(id = 3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_index: Option<ImgId>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct ImageRange {
    #[wfrs(id = 1)]
    pub x: i32,
    #[wfrs(id = 2)]
    pub y: i32,
    #[wfrs(id = 3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_index: Option<ImgId>,
    #[wfrs(id = 4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images_count: Option<u32>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct StatusPosition {
    #[wfrs(id = 1)]
    pub x: i32,
    #[wfrs(id = 2)]
    pub y: i32,
    #[wfrs(id = 3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<Alignment>,
    #[wfrs(id = 4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    unknown4: Option<u32>,
    #[wfrs(id = 5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    unknown5: Option<u32>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct StatusImage {
    #[wfrs(id = 1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<StatusPosition>,
    #[wfrs(id = 2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_image_index: Option<ImgId>,
    #[wfrs(id = 3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_image_index: Option<ImgId>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Coordinates {
    #[wfrs(id = 1)]
    pub x: i32,
    #[wfrs(id = 2)]
    pub y: i32,
}

impl Transform for Vec<Coordinates> {
    fn transform(&mut self, params: &[Param]) {
        for i in 0..params.len() {
            let param = &params[i..=i]; // heh
            let mut coordinates = None;
            coordinates.transform(param);
            self.push(coordinates.unwrap());
        }
    }
}

#[derive(Debug, PartialEq, Default, Deserialize, Clone, Copy)]
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
    fn transform(&mut self, params: &[Param]) {
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
    fn transform(&mut self, params: &[Param]) {
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
    #[wfrs(id = 1)]
    pub top_left_x: i32,
    #[wfrs(id = 2)]
    pub top_left_y: i32,
    #[wfrs(id = 3)]
    pub bottom_right_x: i32,
    #[wfrs(id = 4)]
    pub bottom_right_y: i32,
    #[wfrs(id = 5)]
    pub alignment: Alignment,
    #[wfrs(id = 6)]
    pub spacing_x: i32,
    #[wfrs(id = 7)]
    pub spacing_y: i32,
    #[wfrs(id = 8)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_index: Option<ImgId>,
    #[wfrs(id = 9)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images_count: Option<u32>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct TemperatureType {
    #[wfrs(id = 1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[wfrs(id = 2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minus_image_index: Option<ImgId>,
    #[wfrs(id = 3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix_image_index: Option<ImgId>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct VectorShape {
    #[wfrs(id = 1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_border: Option<bool>,
    #[wfrs(id = 2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[wfrs(id = 3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<Coordinates>,
    #[wfrs(id = 4)]
    pub shape: Vec<Coordinates>,
    #[wfrs(id = 5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center_image: Option<ImageReference>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Transform for Option<Color> {
    fn transform(&mut self, params: &[Param]) {
        let subvalue = match params.get(0).unwrap() {
            Param::Number(number) => number,
            _ => panic!("First param should be number param"),
        };

        let vals = [
            (*subvalue >> 24) as u8,
            (*subvalue >> 16) as u8,
            (*subvalue >> 8) as u8,
            *subvalue as u8,
        ];

        let mut started = false;
        let mut left = 4;
        let mut cur = 0;
        let mut res = Color::default();
        for val in vals {
            if !started && val != 0 {
                started = true;
            }
            if started {
                match cur {
                    0 => res.0 = val,
                    1 => res.1 = val,
                    2 => res.2 = val,
                    3 => res.3 = val,
                    _ => unreachable!(),
                }
                left -= 1;
                cur += 1;
            }
        }

        while left > 0 {
            match cur {
                0 | 1 | 2 => (),
                3 => res.3 = 255,
                _ => unreachable!(),
            }
            left -= 1;
            cur += 1;
        }

        *self = Some(res);
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let vals = [self.0, self.1, self.2, self.3];
        let mut hex_num = vec![];
        let mut started = false;
        let mut first = true;

        for (i, value) in vals.iter().enumerate() {
            if !started && *value != 0 {
                started = true;
            }
            if started {
                if first && *value <= 16 {
                    hex_num.push(format!("{:X}", value));
                } else if i != 3 || *value != 255 {
                    hex_num.push(format!("{:02X}", value));
                }
                first = false;
            }
        }

        if hex_num.len() == 0 {
            hex_num.push("0".to_string());
        }

        serializer.serialize_str(&format!("0x{}", hex_num.concat()))
    }
}
