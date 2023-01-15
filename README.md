# home-webhooks
I didn't want to pay 4.99 for an app. So here's my solution.

## Debugging

Turn on `export RUST_LOG=home_webhook=trace` for logging

## How to Run

`id={discord_webhook_id} token={discord_webhook_token} cargo run`

## Homekit

1. Add automation for whatever smart thing 
2. Convert to shortcut with `Get Contents of `
3. Add URL of web server `http://{ip}:port/`
3. Method `Post`
4. Request body with `{ applianace: "whatever", status: "stuff" }`
