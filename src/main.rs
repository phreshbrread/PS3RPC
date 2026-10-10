use clap::Parser;
use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity};
use ps3rpc::*;
use std::{error::Error, thread, time::Duration};

// http://<PS3 ID>/klic.ps3 is the main Webman info page we need

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // TODO: Read from config file
    let user_config = UserConfig::new(&args.ps3_ip, Duration::from_secs(5));

    let mut connection_message_sent = false;
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
        // Send connection message to PS3 if we haven't already
        if !connection_message_sent {
            match ureq::get(String::from(&connection_message)).call() {
                Ok(_) => connection_message_sent = true,
                Err(err) => {
                    println!("Failed to connect to PS3 ({}): {}", user_config.ps3_ip, err);

                    // Wait for a while and then go to the end of the loop to try again
                    thread::sleep(Duration::from_secs(20));
                    continue;
                }
            }
        }

        // Handle page manipulation in it's own scope so we can drop whatever we don't need
        {
            let page: String = ureq::get(webman_url.clone())
                .call()?
                .body_mut()
                .read_to_string()?;

            if page.contains("KLIC: Not in-game!") {
                current_game = None;
            } else {
                let (mut trimmed_page, _) = page.split_once("</H2>").unwrap();
                (_, trimmed_page) = trimmed_page.split_once("<h2>").unwrap();

                if trimmed_page.len() > 1 {
                    let (id, game) = trimmed_page.split_at(10);
                    current_game = Some(CurrentGame::new(id, game));
                } else {
                    current_game = None;
                }
            }
        }

        println!("Current game: {current_game:?}");

        // Update Discord status
        if let Some(current_game) = current_game {
            let new_status = activity::Activity::new()
                .status_display_type(activity::StatusDisplayType::Details)
                .state(current_game.id)
                .details(current_game.name);

            client.set_activity(new_status)?;
        } else {
            client.clear_activity()?;
        }

        // Delay before restarting
        thread::sleep(user_config.sleep_duration);
    }
}
