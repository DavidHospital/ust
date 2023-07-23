use log::debug;
use rspotify::{prelude::OAuthClient, scopes, AuthCodePkceSpotify, Config, Credentials, OAuth};

use crate::auth::redirect_uri::redirect_uri_web_server;

pub async fn get_client() -> Result<AuthCodePkceSpotify, ()> {
    let creds = Credentials::from_env().unwrap();

    let scopes = scopes!("user-read-playback-state", "user-read-currently-playing");
    let oauth = OAuth::from_env(scopes).unwrap();

    let spotify_config = Config {
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    };
    let mut spotify =
        AuthCodePkceSpotify::with_config(creds.clone(), oauth.clone(), spotify_config.clone());

    // Obtaining the access token
    match spotify.read_token_cache(true).await {
        Ok(Some(token)) => {
            debug!("Successfully read token from cache");
            *spotify.token.lock().await.unwrap() = Some(token.clone());
        }
        _ => {
            debug!("Failed to retrieve token from cache, requires manual authentication");
            let url = spotify.get_authorize_url(None).unwrap();
            match redirect_uri_web_server(8888, &url) {
                Ok(url) => {
                    let code = spotify.parse_response_code(&url).unwrap();
                    spotify.request_token(&code).await.unwrap();
                }
                Err(()) => {}
            };
        }
    };

    Ok(spotify)
}
