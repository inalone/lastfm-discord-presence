const USER: &'static str = "mjghd";

async fn check_now_playing() {
    let mut client = lastfm_rs::Client::new("2c01e403bd06ab75896a9b431e138a6c");
    let tracks = client.recent_tracks(USER).await.send().await;
    if let Ok(tracks) = tracks {
        if tracks.tracks.is_empty() {
            println!("No tracks for user {} found", USER);
            return;
        }

        let recent_track = &tracks.tracks[0];
        if let Some(attributes) = &recent_track.attrs {
            if attributes.now_playing != "true" {
                println!("{} is not currently playing any track", USER);
                return;
            }

            println!("Now playing: {}", recent_track.name);
        } else {
            println!("{} is not currently playing any track", USER);
        }
    };
}

#[tokio::main]
async fn main() {
    check_now_playing().await;
}
