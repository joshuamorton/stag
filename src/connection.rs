use crate::settings::StagConfig;
use futures::stream::{StreamExt, TryStreamExt};
use futures::Stream;
use rspotify::clients::BaseClient;
use rspotify::clients::OAuthClient;
use rspotify::model::playlist::{PlaylistTracksRef, SimplifiedPlaylist};
use rspotify::model::track::FullTrack;
use rspotify::model::PlayableItem;
use rspotify::{scopes, AuthCodeSpotify, Config, Credentials, OAuth};

pub struct Conn<'a> {
    client: AuthCodeSpotify,
    cfg: &'a StagConfig,
    playlist: rspotify::model::PlaylistId<'a>,
}

impl<'a> Conn<'_> {
    pub async fn with_auth(cfg: &'a StagConfig) -> Conn<'a> {
        let creds = Credentials::new(&cfg.client_id, &cfg.client_secret);
        let config = Config {
            token_cached: true,
            ..Default::default()
        };

        let oauth = OAuth {
            scopes: scopes!("playlist-read-private playlist-modify-private"),
            redirect_uri: cfg.redirect_uri.clone(),
            ..Default::default()
        };
        let spotify = AuthCodeSpotify::with_config(creds, oauth, config);
        spotify
            .prompt_for_token(&spotify.get_authorize_url(false).unwrap())
            .await
            .unwrap();

        // This can fail for a really annoying reason if the user has *any* album with no album art,
        // and it's an issue with our dependencies that we can't fix. This is doubly annyoing because
        // currently we create a playlist without any album art, so followups will break.
        let playlists = spotify
            .current_user_playlists()
            .try_collect::<Vec<_>>()
            .await
            .unwrap();
        let pl = playlists
            .iter()
            .filter(|p| p.name == cfg.playlist_name)
            .next();

        let playlist_id = match pl {
            Some(p) => p.id.clone(),
            None => {
                let complex = spotify
                    .user_playlist_create(
                        spotify.me().await.unwrap().id,
                        &cfg.playlist_name,
                        Option::from(false),
                        Option::from(false),
                        Option::from("Playlist for STag managed music."),
                    )
                    .await
                    .unwrap();
                complex.id.clone()
            }
        };
        return Conn {
            client: spotify,
            cfg: cfg,
            playlist: playlist_id,
        };
    }

    pub async fn ensure_playlist(&self) -> SimplifiedPlaylist {
        let complex = self
            .client
            .user_playlist(
                self.client.me().await.unwrap().id,
                Some(self.playlist.clone()),
                None,
            )
            .await
            .unwrap();
        SimplifiedPlaylist {
            collaborative: complex.collaborative,
            external_urls: complex.external_urls,
            href: complex.href,
            id: complex.id,
            images: complex.images,
            name: complex.name,
            owner: complex.owner,
            public: complex.public,
            snapshot_id: complex.snapshot_id,
            tracks: PlaylistTracksRef::default(),
        }
    }

    pub async fn get_tracks(&self) -> Vec<FullTrack> {
        let items = self
            .client
            .playlist_items(self.playlist.clone(), None, None)
            .map(|i| i.unwrap().track)
            .filter_map(|pl| async { pl })
            .filter_map(|pl| async {
                match pl {
                    PlayableItem::Track(t) => Some(t),
                    PlayableItem::Episode(e) => None,
                }
            });
        items.collect::<Vec<_>>().await
    }
}
