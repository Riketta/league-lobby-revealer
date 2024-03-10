use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub(crate) struct Participant {
    #[serde(rename = "activePlatform")]
    pub active_platform: String,
    pub cid: String,
    pub game_name: String,
    pub game_tag: String,
    pub muted: bool,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    pub region: String,
}
