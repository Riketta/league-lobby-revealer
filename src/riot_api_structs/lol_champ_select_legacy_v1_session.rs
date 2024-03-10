use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub(crate) struct LolChampSelectLegacyV1Session {
    #[serde(rename = "errorCode")]
    pub error_code: Option<String>,

    #[serde(rename = "httpStatus")]
    pub http_status: Option<i32>,
    // pub implementationDetails:
    pub message: Option<String>,
}
