use rust_game::network::client::*;

fn main() {
    println!("Yo this works");
    let mut connection_server = ConnectionServer::new(String::from("localhost"), 42069);
    let mut game_server = GameConnection::new();
    let mut player_id = PlayerId { player_id: 256 };
    connect_to_server(&mut connection_server, &mut game_server, &mut player_id);
}
