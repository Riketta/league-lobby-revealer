use crate::player_stats_provider::PlayerStatsProvider;

pub(crate) struct StatProviderDeeplolGG;

impl PlayerStatsProvider for StatProviderDeeplolGG {
    fn get_player_stats(&self, region: &str, player_names_with_tags: &[String]) -> String {
        let mut url = String::from("https://www.deeplol.gg/multi/");
        url.push_str(format!("{}/", &region).as_str());
        url.push_str(self.prepare_player_list(player_names_with_tags).as_str());

        url
    }
}
