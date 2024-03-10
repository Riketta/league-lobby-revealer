use std::io::Read;

use reqwest::{blocking::Client, Method};

use crate::{
    riot_api_credentials::RiotAPICredentials,
    riot_api_structs::{
        chat_v1_session::ChatV1Session, chat_v5_participants::ChatV5Participants,
        lol_champ_select_legacy_v1_session::LolChampSelectLegacyV1Session,
        riot_client_region_locale::RiotClientRegionLocale,
    },
};

const USER_AGENT: &str = "LeagueOfLegendsClient/14.5.563.9790 (CEF 91)";

pub(crate) struct RiotAPI {
    user_agent: String,
    riot_client_credentials: RiotAPICredentials,
    league_client_credentials: RiotAPICredentials,
}

impl RiotAPI {
    pub(crate) fn new(
        riot_client_credentials: RiotAPICredentials,
        league_client_credentials: RiotAPICredentials,
    ) -> Self {
        Self {
            user_agent: USER_AGENT.to_string(),
            riot_client_credentials,
            league_client_credentials,
        }
    }

    fn make_request(
        &self,
        method: Method,
        endpoint: String,
        credentials: &RiotAPICredentials,
    ) -> String {
        let request_url = format!("https://127.0.0.1:{}/{}", credentials.port, endpoint);
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        let mut result = client
            .request(method, request_url)
            .basic_auth(&credentials.user, Some(&credentials.pass))
            .header(reqwest::header::USER_AGENT, &self.user_agent)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .unwrap();

        let mut content = String::new();
        result.read_to_string(&mut content).unwrap();

        content
    }

    fn make_riot_client_request(&self, method: Method, endpoint: String) -> String {
        self.make_request(method, endpoint, &self.riot_client_credentials)
    }

    fn make_league_client_request(&self, method: Method, endpoint: String) -> String {
        self.make_request(method, endpoint, &self.league_client_credentials)
    }

    pub(crate) fn request_chat_v1_session(&self) -> ChatV1Session {
        let json = self.make_riot_client_request(Method::GET, "chat/v1/session".to_string());
        let data: ChatV1Session = serde_json::from_str(&json).unwrap();
        data
    }

    pub(crate) fn request_chat_v5_participants(&self) -> ChatV5Participants {
        let json = self.make_riot_client_request(Method::GET, "chat/v5/participants".to_string());
        let data: ChatV5Participants = serde_json::from_str(&json).unwrap();
        data
    }

    pub(crate) fn request_lol_champ_select_legacy_v1_session(
        &self,
    ) -> LolChampSelectLegacyV1Session {
        let json = self.make_league_client_request(
            Method::GET,
            "lol-champ-select-legacy/v1/session".to_string(),
        );
        let data: LolChampSelectLegacyV1Session = serde_json::from_str(&json).unwrap();
        data
    }

    pub(crate) fn request_riot_client_region_locale(&self) -> RiotClientRegionLocale {
        let json =
            self.make_riot_client_request(Method::GET, "riotclient/region-locale".to_string());
        let data: RiotClientRegionLocale = serde_json::from_str(&json).unwrap();
        data
    }
}
