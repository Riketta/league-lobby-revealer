pub(crate) trait PlayerStatsProvider {
    fn get_player_stats(&self, region: &String, player_names_with_tags: &Vec<String>) -> String;
}
