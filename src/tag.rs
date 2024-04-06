use crate::connection::Conn;
use crate::db::Handler;
use crate::settings::StagConfig;
use rusqlite::Error;

pub async fn tag(
    current: bool,
    song: &Option<String>,
    tags: &Vec<String>,
    cfg: &StagConfig,
) -> Result<(), Error> {
    let mut db = Handler::new(cfg)?;
    let song = match song {
        Some(s) => db.get_song(s)?,
        None => match current {
            true => {
                let s = Conn::with_auth(cfg).await.get_now_playing().await;
                db.maybe_add_song(&s)?;
                db.get_song(&s.name)?
            }
            false => panic!("You have to provide one of `current` and `song!"),
        },
    };
    println!("Addings tags {:?} to song {}", tags, song.title);
    return db.add_tags(&song, tags);
}
