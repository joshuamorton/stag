use crate::settings::StagConfig;

use crate::connection::Conn;
use crate::db::Handler;

pub async fn init(cfg: &StagConfig) -> () {
    let spotify = Conn::with_auth(cfg).await;
    println!("Successfully authorized with spotify.");
    let _ = spotify.ensure_playlist().await;
    println!("Ensured that the playlist existed!");
    let db = Handler::new(cfg);
    match db {
        Ok(_) => println!("Db exists!"),
        Err(ref e) => println!("{}", e),
    }
    let mut db = db.expect("This can't fail?");
    let tracks = spotify.get_tracks().await;
    for track in tracks.iter() {
        let v = db.add_song(&track);
        match v {
            Ok(_) => println!("{}, worked", &track.name),
            Err(e) => println!("{}, {}", &track.name, e),
        };
    }
}
