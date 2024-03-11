use serde::Deserialize;

use super::lol_champ_select_action::LolChampSelectAction;

#[derive(Deserialize, Debug, Default)]
pub(crate) struct LolChampSelectV1Session {
    // #[serde(default)]
    // pub actions: Vec<LolChampSelectAction>,
    #[serde(rename = "errorCode")]
    pub error_code: Option<String>,
    #[serde(rename = "httpStatus")]
    pub http_status: Option<i32>,
    pub message: Option<String>,
}
