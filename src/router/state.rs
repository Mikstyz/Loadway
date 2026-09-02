use std::fmt;

#[derive(Clone, Copy)]
pub enum ConnectionState {
    Offline,
    Online,
    Unknow,
}

impl fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state_str = match self {
            ConnectionState::Offline => "Offline",
            ConnectionState::Online => "Online",
            ConnectionState::Unknow => "Unknow",
        };
        write!(f, "{}", state_str)
    }
}
