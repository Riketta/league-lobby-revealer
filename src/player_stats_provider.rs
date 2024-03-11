pub(crate) trait PlayerStatsProvider {
    fn get_player_stats(&self, region: &String, player_names_with_tags: &Vec<String>) -> String;

    fn prepare_player_list(
        &self,
        player_names_with_tags: &Vec<String>,
        use_hyphen: bool,
    ) -> String {
        let mut players = String::new();
        players.push_str(
            player_names_with_tags
                .iter()
                .map(|player| player.replace(" ", "%20"))
                .map(|player| player.replace("#", if use_hyphen { "-" } else { "%23" }))
                .collect::<Vec<String>>()
                .join(",")
                .as_str(),
        );
        players
    }
}
