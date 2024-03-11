use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub(crate) struct LolChampSelectAction {
    #[serde(rename = "actorCellId")]
    pub actor_cell_id: i32,
    #[serde(rename = "championId")]
    pub champion_id: i32,
    pub completed: bool,
    pub id: i32,
    #[serde(rename = "isAllyAction")]
    pub is_ally_action: bool,
    #[serde(rename = "isInProgress")]
    pub is_in_progress: bool,
    #[serde(rename = "type")]
    pub action_type: String,
}
