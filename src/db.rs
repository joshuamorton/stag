use crate::settings::StagConfig;
use rspotify::model::track::FullTrack;
use rusqlite::{Connection, Error, Error::QueryReturnedNoRows, Result};
use rspotify::prelude::Id;

const CREATE: &str = "
BEGIN;
CREATE TABLE IF NOT EXISTS song(uri TEXT PRIMARY KEY, title TEXT, artist TEXT, duration INTEGER);
CREATE TABLE IF NOT EXISTS tag(uri TEXT, tag TEXT);
COMMIT;
";

const ADD_SONG: &str = " INSERT INTO song(uri, title, artist, duration) VALUES (?1, ?2, ?3, ?4) ";

#[derive(Debug)]
pub struct Song {
    pub uri: String,
    pub title: String,
    pub artist: String,
    pub duration: usize,
}

#[derive(Debug)]
pub struct Tag {
    pub uri: String,
    pub tag: String,
}

pub struct Handler {
    c: Connection,
}

impl Handler {
    pub fn new(cfg: &StagConfig) -> Result<Self> {
        let c = Connection::open(cfg.db_path.clone()).unwrap();
        c.execute_batch(CREATE)?;
        Ok(Self { c: c })
    }

    pub fn add_song(&self, track: &FullTrack) -> Result<(), Error> {
        let mut stmt = self.c.prepare_cached(ADD_SONG)?;
        let id = track.id.as_ref().unwrap();
        let artist = &track.artists.first().unwrap().name;
        stmt.execute((id.id(), &track.name, artist, track.duration.num_milliseconds()))?;
        return Result::Ok(());
    }
}
