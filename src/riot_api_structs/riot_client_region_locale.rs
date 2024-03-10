use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub(crate) struct RiotClientRegionLocale {
    pub locale: String,
    pub region: String,
    #[serde(rename = "webLanguage")]
    pub web_language: String,
    #[serde(rename = "webRegion")]
    pub web_region: String,
}
