use crate::player_stats_provider::PlayerStatsProvider;

pub(crate) struct StatProviderPorofessorGG;

impl PlayerStatsProvider for StatProviderPorofessorGG {
    fn get_player_stats(&self, region: &String, player_names_with_tags: &Vec<String>) -> String {
        let mut url = String::from("https://porofessor.gg/pregame/");
        url.push_str(format!("{}/", &region.to_lowercase()).as_str());
        url.push_str(
            PlayerStatsProvider::prepare_player_list(self, player_names_with_tags, true).as_str(),
        );
        url.push_str("/soloqueue/season");

        url
    }
}
