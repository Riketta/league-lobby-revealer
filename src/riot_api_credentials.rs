pub(crate) struct RiotAPICredentials {
    pub port: u16,
    pub user: String,
    pub pass: String,
}

impl Default for RiotAPICredentials {
    fn default() -> Self {
        Self {
            port: Default::default(),
            user: Default::default(),
            pass: Default::default(),
        }
    }
}
