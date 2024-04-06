use crate::settings::StagConfig;
use rspotify::model::track::FullTrack;
use rspotify::prelude::Id;
use rusqlite::{Connection, Error, Result};

const CREATE: &str = "
BEGIN;
CREATE TABLE IF NOT EXISTS song(uri TEXT PRIMARY KEY, title TEXT, artist TEXT, duration INTEGER);
CREATE TABLE IF NOT EXISTS tag(uri TEXT, tag TEXT);
COMMIT;
";

const ADD_SONG: &str = " INSERT INTO song(uri, title, artist, duration) VALUES (?1, ?2, ?3, ?4) ";

const MAYBE_ADD_SONG: &str =
    "INSERT OR IGNORE INTO song(uri, title, artist, duration) VALUES (?1, ?2, ?3, ?4) ";

const ADD_TAG_BY_ID: &str = "INSERT INTO tag(uri, tag) VALUES (?1, ?2)";

const GET_SONG: &str = " SELECT * FROM song WHERE title = ?1 ";

const GET_TAGS: &str = "SELECT * FROM tags WHERE uri = ?1";

const GET_BY_TAG: &str = "
SELECT * FROM song
INNER JOIN tag ON song.uri = tag.uri
WHERE tag.tag LIKE ?1
";

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

#[derive(Debug)]
pub struct TaggedSong {
    pub uri: String,
    pub title: String,
    pub artist: String,
    pub duration: usize,
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

    pub fn add_song(&mut self, track: &FullTrack) -> Result<(), Error> {
        let mut stmt = self.c.prepare_cached(ADD_SONG)?;
        let id = track.id.as_ref().unwrap();
        let artist = &track.artists.first().unwrap().name;
        stmt.execute((
            id.id(),
            &track.name,
            artist,
            track.duration.num_milliseconds(),
        ))?;
        return Result::Ok(());
    }

    pub fn maybe_add_song(&mut self, track: &FullTrack) -> Result<(), Error> {
        let mut stmt = self.c.prepare_cached(MAYBE_ADD_SONG)?;
        let id = track.id.as_ref().unwrap();
        let artist = &track.artists.first().unwrap().name;
        stmt.execute((
            id.id(),
            &track.name,
            artist,
            track.duration.num_milliseconds(),
        ))?;
        return Result::Ok(());
    }

    pub fn get_song(&self, title: &str) -> Result<Song, Error> {
        let mut stmt = self.c.prepare_cached(GET_SONG)?;
        stmt.query_row([title], |r| {
            Ok(Song {
                uri: r.get(0)?,
                title: r.get(1)?,
                artist: r.get(2)?,
                duration: r.get(3)?,
            })
        })
    }

    pub fn add_tags(&mut self, song: &Song, tags: &Vec<String>) -> Result<(), Error> {
        let tx = self.c.transaction()?;
        {
            let stmt = &mut tx.prepare_cached(ADD_TAG_BY_ID)?;
            for t in tags.iter() {
                stmt.execute((song.uri.clone(), t))?;
            }
        }
        let _ = tx.commit();
        Ok(())
    }

    pub fn get_tags(&self, title: &str) -> Result<Vec<String>, Error> {
        let s = self.get_song(title)?;
        let mut stmt = self.c.prepare_cached(GET_TAGS)?;
        let results = stmt.query_map([s.uri], |r| {
            Ok(Tag {
                uri: r.get(0)?,
                tag: r.get(1)?,
            })
        })?;
        let mut v: Vec<String> = Vec::new();
        for r in results {
            match r {
                Ok(t) => {
                    v.push(t.tag);
                }
                Err(e) => return Err(e),
            }
        }
        return Ok(v);
    }

    pub fn get_by_tag(&self, tag: &str) -> Result<Vec<TaggedSong>, Error> {
        let mut stmt = self.c.prepare_cached(GET_BY_TAG)?;
        let results = stmt.query_map([format!("%{}%", tag)], |r| {
            Ok(TaggedSong {
                uri: r.get(0)?,
                title: r.get(1)?,
                artist: r.get(2)?,
                duration: r.get(3)?,
                tag: r.get(5)?,
            })
        })?;
        let mut v: Vec<TaggedSong> = Vec::new();
        for r in results {
            match r {
                Ok(s) => {
                    v.push(s);
                }
                Err(e) => return Err(e),
            }
        }
        return Ok(v);
    }
}
