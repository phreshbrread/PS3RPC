use clap::Parser;
use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity};
use ps3rpc::*;
use std::{error::Error, thread, time::Duration};

/* TODO: Input listener thread
 * Listen for a specified key, which, when activated, will allow the user to stop connection
 * attempt and enter a new IP. */

/* TODO: Allow for changing IP without touching config file
 * Do this with a --new-ip flag or something */
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Get config file path
    let Some(config_file_path) = get_config_path() else {
        die("ERROR: Failed to get config file path");
    };

    let path_str: &str = config_file_path.to_str().unwrap();
    println!("INFO: Config path: {}", path_str);

    /* Try to read from config file to a string.
     * A loop is needed so we can try load again right after we create the new config file */
    let user_config: UserConfig;
    loop {
        match load_user_config(&config_file_path) {
            Ok(cfg) => {
                user_config = cfg;
                break;
            }
            Err(err) => {
                /* --- If config doesn't exist --- */
                println!("WARNING: A valid config file could not be loaded");

                match err {
                    ConfigReadError::TomlDeError(err) => {
                        // Print serde's TOML deserialize error
                        println!("{err}");
                    }

                    // Ignore as a fatal IO error will be handled later
                    ConfigReadError::IoError(_) => (),
                }
                println!("INFO: Creating a new config at {path_str}");

                // Create new folder & file
                if let Err(err) = create_empty_config(&config_file_path) {
                    die(format!("ERROR: Failed to create new config file: {err}").as_str());
                }

                // Get PS3 IP as trimmed, boxed str
                // TODO: Verify that string is a valid IP
                let mut ip: Box<str>;
                loop {
                    flushed_print("Please enter PS3 IP: ");
                    ip = Box::from(read_line_as_string()?.trim());

                    // Shortest possible IP is 7 characters long
                    if ip.len() >= 7 {
                        break;
                    }
                    println!("\nPlease enter a valid IP address.");
                }

                // Create new settings struct with user-specified IP
                let user_config =
                    UserConfig::new(&ip, Duration::from_secs(10), Duration::from_secs(20));

                // Write config to file
                if let Err(err) = save_user_config(&user_config, &config_file_path) {
                    die(format!("ERROR: Failed to write config file: {err}").as_str());
                }
            }
        }
    }

    // Create Discord IPC Client
    let mut client = DiscordIpcClient::new("1535304909019418705");
    if let Err(err) = client.connect() {
        die(format!(
            "ERROR: Failed to connect to Discord IPC: {err}\n\
             INFO: Please make sure Discord and/or ARRPC is running."
        )
        .as_str());
    }

    let mut connected_to_ps3 = false;
    loop {
        // TODO: Consolidate connection attempts
        // Send connection message to PS3 if we haven't already
        if !connected_to_ps3 {
            println!("Attempting to connect to PS3 ({})", user_config.ps3_ip);

            match establish_connection(&user_config.ps3_ip) {
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

                    // Wait for a while and then end the main loop early to try again
                    thread::sleep(user_config.reconnect_delay);
                    continue;
                }
            }
        }

        // Unwrap the Result<> from query_game() into an Option<CurrentGame>
        let current_game = match query_game(&user_config.ps3_ip) {
            Ok(game) => game,
            Err(err) => {
                todo!("{:#?}", err);
            }
        };

        // The following only runs if current_game is Some()
        if let Some(current_game) = current_game {
            // Output to terminal
            println!(
                "Current Game: {} [{}]",
                &current_game.name, &current_game.id
            );

            // Pull image based on game ID
            // TODO: Fallback to generic image if needed
            let dynamic_image_url = format!(
                "https://raw.githubusercontent.com/aldostools/Resources/main/COV/{}.JPG",
                current_game.id
            );

            // Create image asset for status
            let assets = activity::Assets::new().large_image(dynamic_image_url);

            // Update Discord status
            let new_status = activity::Activity::new()
                .status_display_type(activity::StatusDisplayType::Details)
                .state(&current_game.id)
                .details(&current_game.name)
                .assets(assets);

            client.set_activity(new_status)?;
        } else {
            println!("Current game: None");
            client.clear_activity()?;
        }

        // Delay before restarting
        thread::sleep(user_config.sleep_duration);
    }
}
