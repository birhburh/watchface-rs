use {
    crate::common::*,
    serde::{ser::SerializeSeq, Deserialize, Serialize},
    std::fmt::Debug,
    watchface_rs_derive::TransformDerive,
};

// TODO: check that all fields from UIHH_MIBAND.json copied

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
    #[wfrs_id(10)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analog_dial_face: Option<AnalogDialFace>,
    #[wfrs_id(11)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Other>,
    #[wfrs_id(12)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heart_progress: Option<HeartProgress>,
    #[wfrs_id(14)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week_days_icons: Option<WeekDaysIcons>,
    #[wfrs_id(15)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calories_progress: Option<CaloriesProgress>,
    #[wfrs_id(18)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm: Option<Alarm>,
    #[wfrs_id(20)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status2: Option<Status>,
    #[wfrs_id(21)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<UnknownStruct>,
    #[wfrs_id(22)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lunar_date: Option<LunarDate>,
}

impl WatchfaceParams for MiBandParams {}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Background {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageReference>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
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
    pub unknown_v7: i32,
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
    #[wfrs_id(7)]
    #[serde(rename = "X_EN", skip_serializing_if = "Option::is_none")]
    pub x_en: Option<u32>,
    #[wfrs_id(8)]
    #[serde(rename = "Y_EN", skip_serializing_if = "Option::is_none")]
    pub y_en: Option<u32>,
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
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind: Option<Wind>,
    #[wfrs_id(6)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "UVIndex")]
    pub uv_index: Option<UVIndex>,
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
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_line: Option<TodayOneLine>,
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
pub struct TodayOneLine {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minus_image_index: Option<ImgId>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter_image_index: Option<ImgId>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append_suffix_to_all: Option<bool>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix_image_index: Option<ImgId>,
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
pub struct Wind {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "SuffixImageIndexEN")]
    pub suffix_image_index_en: Option<ImgId>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "SuffixImageIndexCN")]
    pub suffix_image_index_cn: Option<ImgId>,
    #[wfrs_id(4)]
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "SuffixImageIndexCN2"
    )]
    pub suffix_image_index_cn2: Option<ImgId>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "ImagePosSuffixEN")]
    pub image_pos_suffix_en: Option<ImageReference>,
    #[wfrs_id(6)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "ImagePosSuffixCN")]
    pub image_pos_suffix_cn: Option<ImageReference>,
    #[wfrs_id(7)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "ImagePosSuffixCN2")]
    pub image_pos_suffix_cn2: Option<ImageReference>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct UVIndex {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "UV")]
    pub uv: Option<UV>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "UVCN")]
    pub uvcn: Option<ImageRange>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "UVCN2")]
    pub uvcn2: Option<ImageRange>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct UV {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "UVCN")]
    pub suffix_image_index: Option<ImgId>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct StepsProgress {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_image: Option<ImageReference>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_scale: Option<ImageRange>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear: Option<Linear>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_scale: Option<CircleScale>,
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
    pub segments: Vec<Coordinates>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct AnalogDialFace {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<VectorShape>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes: Option<VectorShape>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds: Option<VectorShape>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
pub struct Animations(pub Vec<Animation>);

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct Other {
    #[wfrs_id(1)]
    pub animation: Animations,
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

impl Transform for Animations {
    fn transform(&mut self, params: &[Param]) {
        for i in 0..params.len() {
            let param = &params[i..=i]; // heh
            let mut animations = None;
            animations.transform(param);
            self.0.push(animations.unwrap());
        }
    }
}

impl Serialize for Animations {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.0.len() == 1 {
            self.0[0].serialize(serializer)
        } else {
            let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
            for element in &self.0 {
                seq.serialize_element(&element)?;
            }
            seq.end()
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct HeartProgress {
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_scale: Option<ImageRange>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear: Option<Linear>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_scale: Option<CircleScale>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct CircleScale {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center_x: Option<u32>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center_y: Option<u32>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_x: Option<u32>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_y: Option<u32>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_angle: Option<u32>,
    #[wfrs_id(6)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_angle: Option<u32>,
    #[wfrs_id(7)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[wfrs_id(8)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
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
pub struct CaloriesProgress {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_image: Option<ImageReference>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_scale: Option<ImageRange>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear: Option<Linear>,
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

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct UnknownStruct {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_1: Option<NumberInRect>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_2: Option<NumberInRect>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_3: Option<ImgId>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_4: Option<ImgId>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_5: Option<ImgId>,
    #[wfrs_id(6)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_6: Option<ImgId>,
    #[wfrs_id(7)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_7: Option<ImgId>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, TransformDerive)]
#[serde(rename_all = "PascalCase")]
pub struct LunarDate {
    #[wfrs_id(1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<ImageRange>,
    #[wfrs_id(2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<NumberInRect>,
    #[wfrs_id(3)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "DayOf0X")]
    pub day_of_0x: Option<ImgId>,
    #[wfrs_id(4)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "DayOf2X")]
    pub day_of_2x: Option<ImgId>,
    #[wfrs_id(5)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "DayOf10")]
    pub day_of_10: Option<ImgId>,
    #[wfrs_id(6)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "DayOf20")]
    pub day_of_20: Option<ImgId>,
    #[wfrs_id(7)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "DayOf30")]
    pub day_of_30: Option<ImgId>,
    #[wfrs_id(10)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "DayCN2")]
    pub day_cn2: Option<NumberInRect>,
}
