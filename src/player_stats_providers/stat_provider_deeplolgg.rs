use crate::player_stats_provider::PlayerStatsProvider;

pub(crate) struct StatProviderDeeplolGG;

impl PlayerStatsProvider for StatProviderDeeplolGG {
    fn get_player_stats(&self, region: &String, player_names_with_tags: &Vec<String>) -> String {
        let mut url = String::from("https://www.deeplol.gg/multi/");
        url.push_str(format!("{}/", &region).as_str());
        url.push_str(player_names_with_tags.join(",").as_str());

        url
    }
}
