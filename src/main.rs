use lastfm_rs::user::recent_tracks::Track;
mod discord;
use discord_sdk::activity::{ActivityBuilder, Assets};
use std::time::SystemTime;

const USER: &'static str = "mjghd";

async fn update_discord_presence(discord_client: &discord::Client, track: &Track) {
    let rp = ActivityBuilder::default()
        .details(&track.name)
        .state(&track.artist.name)
        .assets(Assets {
            large_image: Some(track.images[0].image_url.as_str().to_string()),
            large_text: Some(track.album.name.as_str().to_string()),
            small_image: None,
            small_text: None,
        })
        .start_timestamp(SystemTime::now());

    println!(
        "updated activity: {:?}",
        discord_client.discord.update_activity(rp).await
    );
}

async fn check_now_playing(discord_client: &discord::Client) {
    let mut lastfm_client = lastfm_rs::Client::new("2c01e403bd06ab75896a9b431e138a6c");
    let tracks = lastfm_client.recent_tracks(USER).await.send().await;
    if let Ok(tracks) = tracks {
        if tracks.tracks.is_empty() {
            return;
        }

        let recent_track = &tracks.tracks[0];
        if let Some(attributes) = &recent_track.attrs {
            if attributes.now_playing != "true" {
                return;
            }

            update_discord_presence(discord_client, recent_track).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let client = discord::make_client(discord_sdk::Subscriptions::ACTIVITY).await;

    check_now_playing(&client).await;

    let mut r = String::new();
    let _ = std::io::stdin().read_line(&mut r);

    println!(
        "cleared activity: {:?}",
        client.discord.clear_activity().await
    );
    client.discord.disconnect().await;
}
