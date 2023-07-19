use rspotify::{AuthCodePkceSpotify, Config, Token, prelude::BaseClient};


pub struct SpotifyClient {
    auth_client: AuthCodePkceSpotify,
}

impl SpotifyClient {
    async fn token(self) -> Option<Token> {
        match self.auth_client.get_token().lock().await {
            Ok(token) => token.clone(),
            Err(_) => None
        }
    }

    pub async fn is_expired(self) -> bool {
        match self.token().await {
            Some(token) => token.is_expired(),
            None => true,
        }
    }
}

