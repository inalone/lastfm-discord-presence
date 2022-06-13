mod discord;

use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

async fn reset_activity(discord_client: &discord::Client, last_track_url: &mut String) {
    if last_track_url.is_empty() {
        return;
    }

    discord::clear_activity(discord_client).await;
    *last_track_url = String::new();
}

async fn check_now_playing(
    discord_client: &discord::Client,
    lastfm_user: &String,
    last_track_url: &mut String,
) {
    let mut lastfm_client = lastfm_rs::Client::new("2c01e403bd06ab75896a9b431e138a6c");
    let tracks = lastfm_client.recent_tracks(lastfm_user).await.send().await;
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
                reset_activity(discord_client, last_track_url).await;
                return;
            }

            discord::update_presence(discord_client, current_track).await;
            *last_track_url = current_track.url.to_string();
        } else {
            reset_activity(discord_client, last_track_url).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!(
            "lastfm-discord-presence must be given one argument, the desired last.fm username."
        );
        return;
    }

    let lastfm_user = &args[1];

    let client = discord::make_client(discord_sdk::Subscriptions::ACTIVITY).await;
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut last_track_url = String::new();
    while running.load(Ordering::SeqCst) {
        sleep(Duration::from_millis(3000)).await;
        check_now_playing(&client, lastfm_user, &mut last_track_url).await;
    }

    discord::clear_activity(&client).await;
    client.discord.disconnect().await;
}
