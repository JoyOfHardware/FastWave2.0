use moonlight::*;

pub mod wellen_helpers;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub struct Timeline {
    pub blocks: Vec<TimelineBlock>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub struct TimelineBlock {
    pub x: u32,
    pub width: u32,
    pub label: Option<TimeLineBlockLabel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub struct TimeLineBlockLabel {
    pub text: String,
    pub x: u32,
    pub y: u32,
}
