use crate::router::state::ConnectionState;

pub struct ClientConnectoin {
    Name: String,
    State: ConnectionState,
}

impl ClientConnectoin {
    pub fn new(name: &str) -> Self {
        Self {
            Name: name.to_string(),
            State: ConnectionState::Unknow,
        }
    }

    pub fn connectoin() -> bool {
        true
    }

    pub fn discconnect() -> bool {
        true
    }

    pub fn is_online() -> bool {
        true
    }

    pub fn name(&self) -> String {
        self.Name.to_string()
    }

    pub fn state(&self) -> ConnectionState {
        self.State
    }
}
