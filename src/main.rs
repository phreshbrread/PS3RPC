use clap::Parser;
use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity};
use ps3rpc::*;
use std::{
    error::Error,
    fs::{self, File},
    process, thread,
    time::Duration,
};

// http://<PS3 ID>/klic.ps3 is the main Webman info page we need

/* TODO: Allow for changing IP without touching config file
 * Do this with a --new-ip flag or something */
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    dbg!(&args);

    // Get config file path
    let Some(config_file_path) = get_config_path() else {
        die("ERROR: Failed to get config file path");
    };

    /* Try to read from config file to a string.
     * A loop is needed so we can load right after we create the new config file */
    let user_config: UserConfig;
    loop {
        match load_user_config(&config_file_path) {
            Ok(cfg) => {
                user_config = cfg;
                break;
            },
            Err(_) => {
                /* --- If config doesn't exist --- */
                println!("WARNING: No config file found. Creating a new one.");

                // Create new folder & file
                if let Err(err) = create_empty_config(&config_file_path) {
                    die(format!("ERROR: Failed to create new config file: {err}").as_str());
                }

                // Get PS3 IP as trimmed, boxed str
                let mut ip: Box<str>;
                loop {
                    flushed_print("Please enter PS3 IP: ");
                    ip = Box::from(read_line_as_string().trim());

                    // Shortest possible IP is 7 characters long
                    if ip.len() >= 7 {
                        break;
                    }
                }

                dbg!(&ip);

                // Create new settings struct with user-specified IP
                let user_config = UserConfig::new(&ip, Duration::from_secs(10), Duration::from_secs(20));

                // Write new user_config struct to file for later
                let toml_str = toml::to_string(&user_config).unwrap();
                if let Err(err) = std::fs::write(&config_file_path, &toml_str) {
                    die(format!("ERROR: Failed to write config file: {err}").as_str());
                }
            }
        }
    }

    dbg!(&user_config);

    // Parse toml to struct
    // TODO: Account for blank IP
    //let user_config: UserConfig = match toml::from_str(&cfg_contents) {
    //    Ok(contents) => contents,
    //    Err(err) => {
    //        println!("Failed to read settings.toml: {err}");
    //        any_key_continue();
    //        process::exit(1);
    //    }
    //};

    let mut connected_to_ps3 = false;
    let connection_message = format!(
        "http://{}/notify.ps3mapi?msg=Rich+Presence+connection+established&icon=0&snd=",
        user_config.ps3_ip
    );
    let webman_url = format!("http://{}/klic.ps3", user_config.ps3_ip);

    let mut current_game: Option<CurrentGame>;

    // Create Discord IPC Client
    let mut client = DiscordIpcClient::new("1535304909019418705");
    match client.connect() {
        Ok(()) => (),
        Err(err) => {
            die(format!(
                    "ERROR: Failed to connect to Discord IPC: {err}\n\
                Please make sure Discord and/or ARRPC is running."
            )
                .as_str());
                }
    }

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

                    // Wait for a while and then end the main loop early to try again
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
