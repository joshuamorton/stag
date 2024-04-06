use crate::db::Handler;
use crate::settings::StagConfig;
use rusqlite::Error;

pub fn query(tag: &str, cfg: &StagConfig) -> Result<(), Error> {
    let db = Handler::new(cfg)?;
    let songs = db.get_by_tag(tag)?;
    for song in songs.iter() {
        println!("Song: {}, tag: {}", song.title, song.tag);
    }
    return Ok(());
}
