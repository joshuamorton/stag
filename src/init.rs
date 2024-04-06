use crate::settings::StagConfig;
use rspotify::clients::OAuthClient;
use rspotify::{scopes, AuthCodeSpotify, Config, Credentials, OAuth};
use futures::stream::TryStreamExt;

pub async fn init(cfg: StagConfig) -> () {
    let creds = Credentials::new(&cfg.client_id, &cfg.client_secret);
    let config = Config {
        token_cached: true,
        ..Default::default()
    };
    let oauth = OAuth {
        scopes: scopes!("playlist-read-private playlist-modify-private"),
        redirect_uri: cfg.redirect_uri,
        ..Default::default()
    };
    let spotify = AuthCodeSpotify::with_config(creds, oauth, config);
    spotify
        .prompt_for_token(&spotify.get_authorize_url(false).unwrap())
        .await
        .unwrap();
    println!("Successfully authorized with spotify.");
    // unwrap here currently fails if there is any playlist on the account with no album art, and
    // this is an issuein our dependencies that we can't really work around. Currently, when we
    // create a playlist it *doens't* have assoicated album art, so followup runs will explode, we
    // should fix that.
    let playlists = spotify.current_user_playlists().try_collect::<Vec<_>>().await.unwrap();
    if !playlists.iter().any(|p| p.name == cfg.playlist_name) {
       
        println!("Creating new playlist.");
        let _ = spotify.user_playlist_create(
            spotify.me().await.unwrap().id,
            &cfg.playlist_name,
            Option::from(false),
            Option::from(false),
            Option::from("Playlist for STag managed music.")).await;

    }
    println!("Playlist exists!");
}
