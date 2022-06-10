use discord_sdk as ds;
use lastfm_rs::user::recent_tracks::Track;
use std::time::SystemTime;

/// Application identifier for "Andy's Test App" used in the Discord SDK's
/// examples.
pub const APP_ID: ds::AppId = 310270644849737729;

pub struct Client {
    pub discord: ds::Discord,
    pub user: ds::user::User,
    pub wheel: ds::wheel::Wheel,
}

pub async fn clear_activity(discord_client: &Client) {
    match discord_client.discord.clear_activity().await {
        Ok(_) => println!("Cleared Discord activity"),
        Err(_) => println!(),
    }
}

pub async fn make_client(subs: ds::Subscriptions) -> Client {
    let (wheel, handler) = ds::wheel::Wheel::new(Box::new(|err| {
        println!("Encountered an error, {}", err);
    }));

    let mut user = wheel.user();

    let discord = ds::Discord::new(ds::DiscordApp::PlainId(APP_ID), subs, Box::new(handler))
        .expect("unable to create discord client");

    user.0.changed().await.unwrap();

    let user = match &*user.0.borrow() {
        ds::wheel::UserState::Connected(user) => user.clone(),
        ds::wheel::UserState::Disconnected(err) => panic!("failed to connect to Discord: {}", err),
    };

    println!("Connected to Discord, user is {}", user.username);

    Client {
        discord,
        user,
        wheel,
    }
}

pub async fn update_presence(discord_client: &Client, track: &Track) {
    let rp = ds::activity::ActivityBuilder::default()
        .details(&track.name)
        .state(&track.artist.name)
        .assets(ds::activity::Assets {
            large_image: Some(track.images[0].image_url.to_string()),
            large_text: Some(track.album.name.to_string()),
            small_image: None,
            small_text: None,
        })
        .start_timestamp(SystemTime::now());

    match discord_client.discord.update_activity(rp).await {
        Ok(_) => println!(
            "Updated Discord activity, now playing: {} by {}",
            track.name, track.artist.name
        ),
        Err(_) => println!("Unable to update Discord activity - is Discord running?"),
    };
}
