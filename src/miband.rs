use {
    crate::common::*, // TODO: not use star
    serde::{Deserialize, Serialize},
    std::fmt::Debug,
    watchface_rs_derive::TransformDerive,
};

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
    #[wfrs_id(11)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Other>,
    #[wfrs_id(14)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week_days_icons: Option<WeekDaysIcons>,
    #[wfrs_id(18)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm: Option<Alarm>,
    #[wfrs_id(20)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status2: Option<Status>,
}

impl WatchfaceParams for MiBandParams {}

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
    unknown_v7: i32,
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
pub struct StepsProgress {
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_scale: Option<ImageRange>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear: Option<Linear>,
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
    pub segments: Segments,
}

type Segments = Vec<Coordinates>;

impl Transform for Segments {
    fn transform(&mut self, params: &[Param]) {
        for i in 0..params.len() {
            let param = &params[i..=i]; // heh
            let mut coordinates = None;
            coordinates.transform(param);
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<u32>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_count: Option<u32>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_v4: Option<u32>,
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
