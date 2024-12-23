use moonlight::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "serde")]
pub enum TerminalUpMsg {
    RequestFullTermState,
    RequestIncrementalTermStateUpdate,
    SendCharacter(char),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "serde")]
pub enum TerminalDownMsg {
    FullTermUpdate(TerminalScreen),
    BackendTermStartFailure(String),
    TermNotStarted
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(crate = "serde")]
pub struct TerminalScreen {
    pub cols    : usize,
    pub rows    : usize,
    pub content : String,
}
