use crate::player_stats_provider::PlayerStatsProvider;

pub(crate) struct StatProviderUGG;

impl PlayerStatsProvider for StatProviderUGG {
    fn get_player_stats(&self, region: &str, player_names_with_tags: &[String]) -> String {
        let mut url = String::from("https://u.gg/multisearch?region=");
        url.push_str(format!("{}1&summoners=", &region.to_lowercase()).as_str());
        url.push_str(self.prepare_player_list(player_names_with_tags).as_str());

        url
    }
}
