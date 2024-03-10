use serde::Deserialize;

use super::chat_participant::Participant;

#[derive(Deserialize, Debug, Default)]
pub(crate) struct ChatV5Participants {
    pub participants: Vec<Participant>,
}
