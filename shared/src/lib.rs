use moonlight::*;

mod var_format;
pub use var_format::VarFormat;

mod signal_to_timeline;
pub use signal_to_timeline::signal_to_timeline;

pub mod wellen_helpers;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "serde")]
pub struct Timeline {
    pub blocks: Vec<TimelineBlock>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "serde")]
pub struct TimelineBlock {
    pub x: i32,
    pub width: u32,
    pub height: u32,
    pub label: Option<TimeLineBlockLabel>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "serde")]
pub struct TimeLineBlockLabel {
    pub text: String,
    pub x: u32,
    pub y: u32,
}
