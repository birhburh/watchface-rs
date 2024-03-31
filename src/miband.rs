use {
    crate::common::*, // TODO: not use star
    serde::{Deserialize, Serialize},
    std::fmt::Debug, watchface_rs_derive::TransformDerive,
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
    pub weather: Option<Weather>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps_progress: Option<StepsProgress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
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
    fn transform(&mut self, key: u8, params: &[Param]) {
        match key {
            2 => self.background.transform(key, params),
            3 => self.time.transform(key, params),
            4 => self.activity.transform(key, params),
            5 => self.date.transform(key, params),
            6 => self.weather.transform(key, params),
            7 => self.steps_progress.transform(key, params),
            8 => self.status.transform(key, params),
            9 => self.battery.transform(key, params),
            11 => self.other.transform(key, params),
            _ => (),
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Background {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageReference>,
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
    #[wfrs_id(11)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drawing_order: Option<bool>,
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
    #[wfrs_id(7)]
    unknown_v7: i32,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Steps {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "is_zero")]
    pub prefix_image_index: ImgId,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "is_zero")]
    pub suffix_image_index: ImgId,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Calories {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "is_zero")]
    pub suffix_image_index: ImgId,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Pulse {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "is_zero")]
    pub prefix_image_index: ImgId,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "is_zero")]
    pub no_data_image_index: ImgId,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "is_zero")]
    pub suffix_image_index: ImgId,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Distance {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "is_zero")]
    pub km_suffix_image_index: ImgId,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "is_zero")]
    pub decimal_point_image_index: ImgId,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "is_zero")]
    pub miles_suffix_image_index: ImgId,
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
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<NumberInRect>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct DayAmPm {
    #[wfrs_id(1)]
    pub x: i32,
    #[wfrs_id(2)]
    pub y: i32,
    #[wfrs_id(3)]
    #[serde(rename = "ImageIndexAMCN", skip_serializing_if = "is_zero")]
    pub image_index_amcn: ImgId,
    #[wfrs_id(4)]
    #[serde(rename = "ImageIndexPMCN", skip_serializing_if = "is_zero")]
    pub image_index_pmcn: ImgId,
    #[wfrs_id(5)]
    #[serde(rename = "ImageIndexAMEN", skip_serializing_if = "is_zero")]
    pub image_index_amen: ImgId,
    #[wfrs_id(6)]
    #[serde(rename = "ImageIndexPMEN", skip_serializing_if = "is_zero")]
    pub image_index_pmen: ImgId,
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
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct StepsProgress {
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_scale: Option<ImageRange>,
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
    #[serde(skip_serializing_if = "is_zero")]
    pub prefix_image_index: ImgId,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "is_zero")]
    pub suffix_image_index: ImgId,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Linear {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "is_zero")]
    pub start_image_index: ImgId,
    #[wfrs_id(2)]
    pub segments: Segments,
}

type Segments = Vec<Coordinates>;

impl Transform for Segments {
    fn transform(&mut self, key: u8, params: &[Param]) {
        for i in 0..params.len() {
            let param = &params[i..=i]; // heh
            let mut coordinates = None;
            coordinates.transform(key, param);
            self.push(coordinates.unwrap());
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Other {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Animation {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_images: Option<ImageRange>,
    #[wfrs_id(2)]
    pub speed: u32,
    #[wfrs_id(3)]
    pub repeat_count: u32,
    #[wfrs_id(4)]
    pub unknown_v4: u32,
}
