use crate::settings::StagConfig;
use rspotify::clients::OAuthClient;
use rspotify::{scopes, AuthCodeSpotify, Config, Credentials, OAuth};

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
    println!("Successfully authorized with spotify.")
}
