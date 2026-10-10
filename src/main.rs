use ps3rpc::*;
use std::{error::Error, thread, time::Duration};

// http://<PS3 ID>/klic.ps3 is the main WebManMOD info page we need

fn main() -> Result<(), Box<dyn Error>> {
    let test_config = UserConfig::testing_defaults();

    let webman_url = format!("http://{}/klic.ps3", test_config.ps3_ip);

    loop {
        let id_game_name: String;

        // Handle page manipulation in it's own scope so we can drop whatever we don't need
        {
            let page: String = ureq::get(webman_url.clone())
                .call()?
                .body_mut()
                .read_to_string()?;

            let (mut trimmed_page, _) = page.split_once("</H2>").unwrap();
            (_, trimmed_page) = trimmed_page.split_once("<h2>").unwrap();
            id_game_name = trimmed_page.to_string();
        }

        // Example id_game_name: NPUB30576 Warhammer 40,000: Space Marine
        println!("Current game: {id_game_name}");
        thread::sleep(Duration::from_secs(10));
    }

    //return Ok(());
}
