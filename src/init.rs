use crate::settings::StagConfig;

use crate::connection::Conn;

pub async fn init(cfg: &StagConfig) -> () {
    let spotify = Conn::with_auth(cfg).await;
    println!("Successfully authorized with spotify.");
    let _ = spotify.ensure_playlist().await;
    println!("Ensured that the playlist existed!");
}
