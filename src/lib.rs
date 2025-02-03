use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    username: String,
    id: u64,
    ping: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    ip: String,
    port: u16,
    game: String,
    hostname: String,
    map: String,
    gametype: String,
    players: Vec<Player>,
    maxplayers: u8,
    hardcore: bool,
    password: bool,
    bots: u8,
    voice: u8,
    description: String,
    #[serde(rename = "codInfo")]
    cod_info: String,
    revision: u32,
}

pub async fn pluto_api() -> Result<HashMap<String, Vec<Server>>, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://plutonium.pw/api/servers")
        .send()
        .await?
        .json::<Vec<Server>>()
        .await?;

    let mut servers_by_game: HashMap<String, Vec<Server>> = HashMap::new();

    for server in response {
        servers_by_game
            .entry(server.game.clone())
            .or_insert_with(Vec::new)
            .push(server);
    }

    Ok(servers_by_game)
}
