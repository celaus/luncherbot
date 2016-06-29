# Luncherbot - a bot for choosing your lunchspot

This bot is an experiment that connects to a Slack channel of your choice in order to provide a simple (currently: random) proposition for where to go for lunch.

## Setup

Create a file `config.toml` with the following content:

```
[keys]
google = "my-google-api-key"
slack = "my-slack-api-key-for-bots"
fs_client_id = "foursquare-client-id"
fs_client_secret = "foursquare-client-secret"
```


## Run Luncherbot

After cloning, you can use `cargo run` to build and run the bot. Make sure both config files (`logging.yml` and `config.toml`).
