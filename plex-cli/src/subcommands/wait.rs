use async_std::future;
use clap::ArgMatches;
use plex_api::Server;
use std::time::Duration;
use std::{thread, time};

pub(crate) async fn subcommand_wait(
    token: &str,
    matches: Option<&ArgMatches<'_>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let matches = matches.unwrap();
    let timeout: Duration =
        Duration::from_secs(matches.value_of("timeout").unwrap_or("30").parse().unwrap());
    let server_url = matches.value_of("server-url").unwrap();
    let wait_for_settings = matches.is_present("wait-for-settings");

    let result = future::timeout(timeout, async {
        loop {
            let srv = {
                if token.is_empty() {
                    Server::connect(server_url).await
                } else {
                    Server::connect_auth(server_url, token).await
                }
            };
            if let Ok(srv) = srv {
                if wait_for_settings {
                    if let Ok(settings) = srv.get_settings().await {
                        if settings.get("AcceptedEULA").is_ok() {
                            break;
                        }
                    }
                } else {
                    break;
                }
            } else {
                thread::sleep(time::Duration::from_secs(1));
            }
        }
    })
    .await;

    if result.is_err() {
        Err(Box::new(result.err().unwrap()))
    } else {
        Ok(())
    }
}