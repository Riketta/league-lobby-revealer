use crate::player_stats_provider::PlayerStatsProvider;

pub(crate) struct StatProviderOPGG;

impl PlayerStatsProvider for StatProviderOPGG {
    fn get_player_stats(&self, region: &String, player_names_with_tags: &Vec<String>) -> String {
        let mut url = String::from("https://www.op.gg/multisearch/");
        url.push_str(format!("{}?summoners=", &region).as_str());
        url.push_str(player_names_with_tags.join(",").as_str());

        url
    }
}
