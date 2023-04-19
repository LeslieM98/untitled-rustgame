#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenetChannel {
    LobbySync = 0,
    PlayerToServerSync = 1,
    ServerToClientSync = 2,
}

impl Into<u8> for RenetChannel {
    fn into(self) -> u8 {
        return self as u8;
    }
}
