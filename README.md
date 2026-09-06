# What is bajam ?
> -- Bajam ?
> -- Préjan
> -- Baj ?
> -- Préjaaan

I'm planning to go on a long bike trip for which i receive a lot of demands for making updates along the way. Polarstep exists but you don't really like the look of it.

The idea is to have "simple" pipeline for me to tell others about my position and give some additionnal info.
With the help of 'Shortcuts' from the Iphone :
1- Get latitude and longitude from my gps position
2- Connect to tailscale
3- Post to the server on the tailnet
```json
{
  "latitude": XX.XXXXX,
  "longitude": XX.XXXXX
}
```
And maybe more (text, photos... etc).

> "But more.... much more than this... I did it my way..."

4- The pin appears on a web site. I wanted to use map-libre js for displaying this. I love this library.

# Quick start
Clone the repo.
```bash
# Create Sqlite db + .env file with predefined values
cargo run - p setup

# Launch the server
cargo run # --tailnet as an option but you need to define a TCP_TAILNET in the .env file
```
