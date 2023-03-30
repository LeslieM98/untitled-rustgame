pub struct ConnectedPlayersPacket {
    ids: [u64; 5],
}

impl ConnectedPlayersPacket {
    pub fn new(ids: &[u64; 5]) -> ConnectedPlayersPacket {
        ConnectedPlayersPacket { ids: ids.clone() }
    }
}
fn send_connected_players() {}
