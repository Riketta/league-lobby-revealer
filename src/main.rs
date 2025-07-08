mod player_stats_provider;
mod player_stats_providers;
mod riot_api;
mod riot_api_credentials;
mod riot_api_structs;
mod string_extension;
mod url_encoder;
mod wmi_manager;

use std::{cmp, env::args, time::Duration};

use crate::{
    player_stats_provider::PlayerStatsProvider,
    player_stats_providers::{
        stat_provider_deeplolgg::StatProviderDeeplolGG, stat_provider_opgg::StatProviderOPGG,
        stat_provider_porofessorgg::StatProviderPorofessorGG,
        stat_provider_porogg::StatProviderPoroGG, stat_provider_ugg::StatProviderUGG,
    },
    riot_api::RiotAPI,
    riot_api_credentials::RiotAPICredentials,
    string_extension::StringExt,
};
use wmi_manager::Wmi;

static CLIENT_PROCESS_NAME: &str = "LeagueClientUX.exe";

#[allow(clippy::too_many_lines)]
fn main() {
    let mut include_premade = true;

    let args = args();
    for arg in args {
        // Should premade be included to stats?
        if arg.to_lowercase() == "--exclude-premade".to_lowercase() {
            include_premade = false;
        }
    }

    let wmi: Wmi = Wmi::new();
    let mut arguments: Option<String>;

    loop {
        arguments = wmi.get_arguments_for_process_with_name(CLIENT_PROCESS_NAME);
        if arguments.is_some() {
            break;
        }

        clear();
        println!("No process found!");
        std::thread::sleep(Duration::from_millis(3000));
    }
    let arguments: String = arguments.unwrap();
    println!("{arguments}");

    let arguments_list = arguments.split_as_arguments();

    let mut riot_client_api_credentials = RiotAPICredentials::default();
    let mut league_client_api_credentials = RiotAPICredentials::default();
    riot_client_api_credentials.user = "riot".into();
    league_client_api_credentials.user = "riot".into();

    for (i, _) in arguments_list.iter().enumerate() {
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
                    String::new()
                }
            );
        }
    }

    let stats_providers: Vec<Box<dyn PlayerStatsProvider>> = vec![
        Box::new(StatProviderPorofessorGG),
        Box::new(StatProviderDeeplolGG),
        Box::new(StatProviderUGG),
        Box::new(StatProviderOPGG),
        Box::new(StatProviderPoroGG),
    ];

    let api: RiotAPI = RiotAPI::new(riot_client_api_credentials, league_client_api_credentials);

    let region_locale = api.request_riot_client_region_locale();
    let chat_session = api.request_chat_v1_session();

    let me = format!("{}#{}", chat_session.game_name, chat_session.game_tag);
    let region: String = region_locale.region;

    let mut premade_players: Vec<String> = Vec::new();
    let mut random_players: Vec<String>;
    loop {
        let chat_participants = api.request_chat_v5_participants();
        let champ_select_v1_session = api.request_lol_champ_select_v1_session();
        let champ_select_session_error_message =
            champ_select_v1_session.message.unwrap_or_default();

        // Remember group members and wipe old data about randoms if you not if champ select.
        if &champ_select_session_error_message == "No active delegate" {
            // println!("Not in Champ Select!");

            random_players = Vec::new();
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
                if (!premade_players.contains(&full_player_name) || include_premade)
                    && !random_players.contains(&full_player_name)
                {
                    random_players.push(full_player_name);
                }
            }
        }

        clear();
        // println!("{}: {}", champ_select_session_error_message, champ_select_session_error_message == "No active delegate");

        if (include_premade && random_players.len() > 5)
            || (!include_premade && random_players.len() + premade_players.len() > 5)
        {
            println!("[!] Warning: more than 5 players in lobby detected: close active chat tabs!");
        }

        println!("# Premade");
        for player in &premade_players {
            println!("- {player}");
        }
        println!();

        println!("# Randoms");
        for player in &random_players {
            println!("- {player}");
        }
        println!();

        println!("# Stats");
        if random_players.is_empty() {
            println!("Waiting for random players to appear.");
        } else {
            for stats_provider in &stats_providers {
                println!(
                    "{}",
                    stats_provider.get_player_stats(&region, &random_players)
                );
            }
        }

        std::thread::sleep(Duration::from_millis(3000));
    }
}

/// Clear console.
fn clear() {
    let term = console::Term::stdout();
    term.clear_screen().expect("Failed to clear console!");
}

#[cfg(test)]
mod tests {
    use crate::{
        player_stats_provider::PlayerStatsProvider,
        player_stats_providers::stat_provider_porofessorgg::StatProviderPorofessorGG,
    };

    #[test]
    fn test_stat_url() {
        let region: String = "EUW".to_string();
        let stats_provider = StatProviderPorofessorGG;

        let players: Vec<String> = vec![
            "αurl#EUW".to_string(),
            "Shëun#2530".to_string(),
            "FigoHSV#HSV".to_string(),
            "never int hehexd#EUW".to_string(),
            "소년 구세주#cigan".to_string(),
        ];

        let url = stats_provider.get_player_stats(&region, &players);
        assert_eq!(
            &url,
            "https://porofessor.gg/pregame/euw/%CE%B1url%23EUW%2CSh%C3%ABun%232530%2CFigoHSV%23HSV%2Cnever+int+hehexd%23EUW%2C%EC%86%8C%EB%85%84+%EA%B5%AC%EC%84%B8%EC%A3%BC%23cigan/soloqueue/season"
        );
    }
}
