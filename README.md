# lastfm-discord-presence

A simple, configuration-free way to display your now playing last.fm track as Discord Rich Presence!

## Downloading + running

If you want to clone the git repository and build manually, cloning the repo and then running `cargo run --release <username>` (replacing <username>
with your last.fm username) should work as expected.

If you want to download an executable file (Linux only for now, soz), download the latest artifact at https://gitlab.com/inalone/lastfm-discord-presence/-/releases,
place it anywhere you want and just run it like `./<place it's downloaded or whatever>/lastfm-discord-presence <username>`.

## Known caveats

- I haven't implemented repeated retries to connect to the Discord client yet, so in order to run this, Discord must first already be open.
