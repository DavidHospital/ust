use auth::client::get_client;
use dotenv::dotenv;
use log::error;
use rspotify::{model::PlayableItem, prelude::OAuthClient};

mod auth;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let spotify_client = get_client().await.unwrap();

    // Running the requests
    match spotify_client.current_playing(None, None::<Vec<_>>).await {
        Ok(Some(currently_playing)) => match currently_playing.item {
            Some(PlayableItem::Track(full_track)) => {
                let artists = full_track
                    .artists
                    .iter()
                    .map(|a| a.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                let artists = artists.trim_end_matches(",");
                println!("{}: {}", full_track.name, artists);
            }
            _ => {}
        },
        _ => error!("error getting currently playing"),
    };
}
