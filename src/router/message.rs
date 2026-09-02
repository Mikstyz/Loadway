use crate::helper::time::time_stump;
use bytes::Bytes;
use std::fmt;

pub enum MsgState {
    Created,
    Expectation,
    Close,
    Freze,
    ToBackend,
    ToUser,
}

impl fmt::Display for MsgState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MsgState::Created => write!(f, "Created"),
            MsgState::Expectation => write!(f, "Expectation"),
            MsgState::Close => write!(f, "Close"),
            MsgState::Freze => write!(f, "Freze"),
            MsgState::ToBackend => write!(f, "ToBackend"),
            MsgState::ToUser => write!(f, "ToUser"),
        }
    }
}

pub struct Msg {
    pub idh: String, //hash
    pub at: u128,
    pub status: MsgState,
    pub data: Bytes,
}

impl Msg {
    pub fn new(idh: &str, data: Bytes) -> Self {
        Self {
            idh: idh.to_string(),
            at: time_stump(),
            status: MsgState::Created,
            data: data,
        }
    }

    pub fn upd_state(&mut self, new_status: MsgState) {
        self.status = new_status
    }

    //geters
    pub fn idh(&self) {}
    pub fn at(&self) {}
    pub fn status(&self) {}
    pub fn data(&self) {}
}
