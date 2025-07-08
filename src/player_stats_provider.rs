use crate::url_encoder;

pub(crate) trait PlayerStatsProvider {
    fn get_player_stats(&self, region: &str, player_names_with_tags: &[String]) -> String;

    fn prepare_player_list(&self, player_names_with_tags: &[String]) -> String {
        let prepared_names = player_names_with_tags.join(",");

        url_encoder::encode(&prepared_names)
    }
}
