use crate::settings::StagConfig;
use futures::stream::TryStreamExt;
use rspotify::clients::OAuthClient;
use rspotify::model::playlist::{PlaylistTracksRef, SimplifiedPlaylist};
use rspotify::{scopes, AuthCodeSpotify, Config, Credentials, OAuth};

pub struct Conn<'a> {
    client: AuthCodeSpotify,
    cfg: &'a StagConfig,
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
        return Conn {
            client: spotify,
            cfg: cfg,
        };
    }

    pub async fn ensure_playlist(&self) -> SimplifiedPlaylist {
        // This can fail for a really annoying reason if the user has *any* album with no album art,
        // and it's an issue with our dependencies that we can't fix. This is doubly annyoing because
        // currently we create a playlist without any album art, so followups will break.
        let playlists = self
            .client
            .current_user_playlists()
            .try_collect::<Vec<_>>()
            .await
            .unwrap();
        let pl = playlists
            .iter()
            .filter(|p| p.name == self.cfg.playlist_name)
            .next();

        return match pl {
            Some(p) => p.clone(),
            None => {
                let complex = self
                    .client
                    .user_playlist_create(
                        self.client.me().await.unwrap().id,
                        &self.cfg.playlist_name,
                        Option::from(false),
                        Option::from(false),
                        Option::from("Playlist for STag managed music."),
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
        };
    }
}
