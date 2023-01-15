use anyhow::{Error, Result};
use chrono::Local;
use log::{info, debug, error};
use serde_derive::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::Filter;
use warp::http::StatusCode;
use webhook::client::WebhookClient;

#[derive(Deserialize, Serialize)]
struct Info {
    applianace: String,
    status: String,
}

async fn send_webhook(input: Info) -> Result<(), Error> {
  let id: &'static str = env!("id");
  let token: &'static str = env!("token");
  let url: &'static str = format!("https://discord.com/api/webhooks/{id}/{token}");
  
  let current_time: &'static str = format!("{}", Local::now().format("%H:%M:%S"));
  let content: &'static str = format!("{} : Status: {} - {} \n@everyone", &input.applianace, &input.status, current_time);
  
  let client: WebhookClient = WebhookClient::new(&url);
  match client.send(|message|  message.username("HomeKit").content(&content)).await 
  {
    Ok(..) => {
      info!("Successfully sent webhook");
      debug!("{}", content);
      Ok(())
    },
    Err(_err) => {
      let msg = "Failed to send webhook notification";
      error!("{}", msg);
      Err(Error::msg(msg))
    }
  }
}

async fn notif(input: Info) -> Result<impl warp::Reply, Infallible> {
  info!("applianace: {} status: {}", input.applianace, input.status);

  match send_webhook(input).await {
    Ok(..) => {
      Ok(StatusCode::OK)
    },
    Err(..) => {
      Ok(StatusCode::INTERNAL_SERVER_ERROR)
    },
  }
}

#[tokio::main]
async fn main() {
  env_logger::init();

  // POST /
  let discord_webhook = warp::path::end()
    .and(warp::body::json())
    .and_then(notif);

  // GET /health
  let health = warp::path("health").map(|| {
    info!("/health");
    format!("Status: {}", StatusCode::OK)
  });

  let get_route = warp::get().and(health);
  let put_route = warp::post().and(discord_webhook);
  let routes = get_route.or(put_route);

  info!("Serving: 127.0.0.1 on port: 3333");
  warp::serve(routes).run(([127, 0, 0, 1], 3333)).await;
}
