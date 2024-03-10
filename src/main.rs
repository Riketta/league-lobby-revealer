mod player_stats_provider;
mod player_stats_providers;
mod riot_api;
mod riot_api_credentials;
mod riot_api_structs;
mod string_extension;
mod wmi_manager;

use std::{cmp, time::Duration};

use crate::{
    player_stats_provider::PlayerStatsProvider,
    player_stats_providers::stat_provider_deeplolgg::StatProviderDeeplolGG, riot_api::RiotAPI,
    riot_api_credentials::RiotAPICredentials, string_extension::StringExt,
};
use wmi_manager::WMI;

fn main() {
    let wmi: WMI = WMI::new();
    let arguments = wmi.get_arguments_for_process_with_name("LeagueClientUX.exe");

    // TODO: handle no client case and wait for client to appear.
    let arguments = arguments.expect("No such process found!");
    println!("{arguments}");

    let arguments_list = arguments.split_as_arguments();

    let mut riot_client_api_credentials = RiotAPICredentials::default();
    let mut league_client_api_credentials = RiotAPICredentials::default();
    riot_client_api_credentials.user = "riot".into();
    league_client_api_credentials.user = "riot".into();

    for i in 0..arguments_list.len() {
        let argument = &arguments_list[i];

        let temp = argument
            .trim_matches('"')
            .splitn(2, '=')
            .collect::<Vec<_>>();
        let key = temp[0];
        let value = if temp.len() > 1 { Some(temp[1]) } else { None };

        let is_matched: bool = match key {
            "--riotclient-auth-token" => {
                riot_client_api_credentials.pass = value.unwrap().into();
                true
            }
            "--riotclient-app-port" => {
                riot_client_api_credentials.port = value.unwrap().parse().unwrap();
                true
            }
            "--remoting-auth-token" => {
                league_client_api_credentials.pass = value.unwrap().into();
                true
            }
            "--app-port" => {
                league_client_api_credentials.port = value.unwrap().parse().unwrap();
                true
            }
            _ => false,
        };

        if is_matched {
            println!(
                "{i}. [{key}]{}",
                if value.is_some() {
                    " ".to_owned() + value.unwrap_or("-")
                } else {
                    "".to_owned()
                }
            );
        }
    }

    let api: RiotAPI = RiotAPI::new(riot_client_api_credentials, league_client_api_credentials);

    let region_locale = api.request_riot_client_region_locale();
    let chat_session = api.request_chat_v1_session();

    let me = format!("{}#{}", chat_session.game_name, chat_session.game_tag);
    let region: String = region_locale.region;

    let stats_provider: Box<dyn PlayerStatsProvider> = Box::new(StatProviderDeeplolGG);
    let mut premade_players: Vec<String> = Vec::new();
    let mut random_players: Vec<String> = Vec::new();
    loop {
        let chat_participants = api.request_chat_v5_participants();
        let champ_select_session = api.request_lol_champ_select_legacy_v1_session();

        if champ_select_session.message.unwrap_or_default() == "Not in Champ Select" {
            // println!("Not in Champ Select!");

            premade_players = Vec::with_capacity(cmp::max(chat_participants.participants.len(), 1));
            // DEBUG: Comment this so you will be counted as random for testing purposes (in practice tool lobby, for example).
            premade_players.push(me.clone());

            for player in chat_participants.participants {
                let full_player_name = format!("{}#{}", player.game_name, player.game_tag);
                if !premade_players.contains(&full_player_name) {
                    premade_players.push(full_player_name);
                }
            }
        } else {
            // println!("In Champ Select!");

            random_players = Vec::with_capacity(chat_participants.participants.len());
            for player in chat_participants.participants {
                let full_player_name = format!("{}#{}", player.game_name, player.game_tag);
                if !premade_players.contains(&full_player_name)
                    && !random_players.contains(&full_player_name)
                {
                    random_players.push(full_player_name);
                }
            }
        }

        print!("\x1B[2J\x1B[1;1H");
        println!("# Premade");
        for player in &premade_players {
            println!("- {player}");
        }

        println!("# Randoms");
        for player in &random_players {
            println!("- {player}");
        }

        println!("# Stats");

        println!(
            "{}",
            stats_provider.get_player_stats(&region, &random_players)
        );

        std::thread::sleep(Duration::from_millis(3000));
    }
}
