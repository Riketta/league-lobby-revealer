use crate::player_stats_provider::PlayerStatsProvider;

pub(crate) struct StatProviderPoroGG;

impl PlayerStatsProvider for StatProviderPoroGG {
    fn get_player_stats(&self, region: &String, player_names_with_tags: &Vec<String>) -> String {
        let mut url = String::from("https://poro.gg/multi?region=");
        url.push_str(format!("{}&q=", &region).as_str());
        url.push_str(player_names_with_tags.join(",").as_str());

        url
    }
}
