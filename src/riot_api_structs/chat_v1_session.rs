use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub(crate) struct ChatV1Session {
    pub federated: bool,
    pub game_name: String,
    pub game_tag: String,
    pub loaded: bool,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    pub region: String,
    pub resource: String,
    pub state: String,
}
