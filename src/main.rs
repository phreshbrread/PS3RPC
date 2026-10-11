use clap::Parser;
use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity};
use ps3rpc::*;
use std::{error::Error, thread, time::Duration};

// http://<PS3 ID>/klic.ps3 is the main Webman info page we need

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // TODO: Read from config file
    let user_config = UserConfig::new(
        &args.ps3_ip,
        Duration::from_secs(args.sleep_duration.unwrap_or(10)),
        Duration::from_secs(args.reconnect_delay.unwrap_or(20)),
    );

    let mut connected_to_ps3 = false;
    let connection_message = format!(
        "http://{}/notify.ps3mapi?msg=Rich+Presence+connection+established&icon=0&snd=",
        user_config.ps3_ip
    );
    let webman_url = format!("http://{}/klic.ps3", user_config.ps3_ip);

    let mut current_game: Option<CurrentGame>;

    // Create Discord IPC Client
    let mut client = DiscordIpcClient::new("1535304909019418705");
    client.connect()?;

    loop {
        // TODO: Consolidate connection attempts
        // Send connection message to PS3 if we haven't already
        if !connected_to_ps3 {
            println!("Attempting to connect to PS3 ({})", user_config.ps3_ip);

            match ureq::get(connection_message.as_str()).call() {
                Ok(_) => {
                    println!("Connected to PS3 ({})", user_config.ps3_ip);
                    connected_to_ps3 = true;
                }
                Err(err) => {
                    println!(
                        "Failed to connect to PS3 ({}): {}",
                        user_config.ps3_ip,
                        err.into_io()
                    );

                    // Wait for a while and then go to the end of the loop to try again
                    thread::sleep(user_config.reconnect_delay);
                    continue;
                }
            }
        }

        // Handle page manipulation in it's own scope so we can drop whatever data we don't need
        {
            let page: String = match ureq::get(webman_url.as_str()).call() {
                Ok(mut response) => response.body_mut().read_to_string()?,
                Err(err) => {
                    println!(
                        "Failed to connect to PS3 ({}): {}",
                        user_config.ps3_ip,
                        err.into_io()
                    );

                    // Reset connection status and end loop early
                    connected_to_ps3 = false;
                    thread::sleep(user_config.reconnect_delay);
                    continue;
                }
            };

            if page.contains("KLIC: Not in-game!") {
                current_game = None;
            } else {
                let (mut trimmed_page, _) = page.split_once("</H2>").unwrap();
                (_, trimmed_page) = trimmed_page.split_once("<h2>").unwrap();

                if trimmed_page.len() > 1 {
                    let (id, game) = trimmed_page.split_at(10);
                    current_game = Some(CurrentGame::new(id.trim_end(), game));
                } else {
                    current_game = None;
                }
            }
        }

        if let Some(current_game) = current_game {
            // Output to terminal
            println!(
                "Current Game: {} [{}]",
                &current_game.name, &current_game.id
            );

            // Update Discord status
            let new_status = activity::Activity::new()
                .status_display_type(activity::StatusDisplayType::Details)
                .state(&current_game.id)
                .details(&current_game.name);

            client.set_activity(new_status)?;
        } else {
            println!("Current game: None");
            client.clear_activity()?;
        }

        // Delay before restarting
        thread::sleep(user_config.sleep_duration);
    }
}
