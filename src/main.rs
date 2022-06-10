use lastfm_rs::user::recent_tracks::{Album, Artist, Track};
mod discord;
use discord_sdk::activity::{ActivityBuilder, Assets};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use tokio::time::sleep;

const USER: &'static str = "mjghd";

async fn update_discord_presence(discord_client: &discord::Client, track: &Track) {
    let rp = ActivityBuilder::default()
        .details(&track.name)
        .state(&track.artist.name)
        .assets(Assets {
            large_image: Some(track.images[0].image_url.to_string()),
            large_text: Some(track.album.name.to_string()),
            small_image: None,
            small_text: None,
        })
        .start_timestamp(SystemTime::now());

    println!(
        "updated activity: {:?}",
        discord_client.discord.update_activity(rp).await
    );
}

async fn check_now_playing(discord_client: &discord::Client, last_track_url: &mut String) {
    let mut lastfm_client = lastfm_rs::Client::new("2c01e403bd06ab75896a9b431e138a6c");
    let tracks = lastfm_client.recent_tracks(USER).await.send().await;
    if let Ok(tracks) = tracks {
        if tracks.tracks.is_empty() {
            return;
        }

        let current_track = &tracks.tracks[0];
        if current_track.url == last_track_url.as_str() {
            return;
        }

        if let Some(attributes) = &current_track.attrs {
            if attributes.now_playing != "true" {
                return;
            }

            update_discord_presence(discord_client, current_track).await;
            *last_track_url = current_track.url.to_string();
        }
    }
}

#[tokio::main]
async fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let client = discord::make_client(discord_sdk::Subscriptions::ACTIVITY).await;

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut last_track_url = String::new();
    while running.load(Ordering::SeqCst) {
        sleep(Duration::from_millis(3000)).await;
        println!("100 ms have elapsed");
        check_now_playing(&client, &mut last_track_url).await;
    }

    println!(
        "cleared activity: {:?}",
        client.discord.clear_activity().await
    );
    client.discord.disconnect().await;
}
