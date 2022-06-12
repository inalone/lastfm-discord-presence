# lastfm-discord-presence

A simple, configuration-free way to display your now playing last.fm track as Discord Rich Presence!

## Running the program

Simply cd to the directory and run
`cargo run --release`

## Known caveats

- I haven't decided how to handle usernames the best way yet, so to display your user, change the `const USER` in main.rs to your username.
- I haven't implemented repeated retries to connect to the Discord client yet, so in order to run this, Discord must first already be open.