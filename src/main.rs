use ps3rpc::*;
use std::{error::Error, thread, time::Duration};

// http://<PS3 ID>/klic.ps3 is the main Webman info page we need

fn main() -> Result<(), Box<dyn Error>> {
    let test_config = UserConfig::testing_defaults();

    let webman_url = format!("http://{}/klic.ps3", test_config.ps3_ip);

    let mut current_game: Option<CurrentGame>;

    loop {
        let id_game_name: Option<String>;

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
                dbg!(&trimmed_page);
                current_game = Some(CurrentGame::new(trimmed_page, trimmed_page));
            }
        }

        // Example id_game_name: NPUB30576 Warhammer 40,000: Space Marine
        println!("Current game: {current_game:?}");
        thread::sleep(Duration::from_secs(10));
    }

    //return Ok(());
}

#[derive(Debug)]
pub struct CurrentGame {
   pub id: String,
   pub name: String,
}

impl CurrentGame {
    pub fn new(id: &str, name: &str) -> Self {
        return CurrentGame {
            id: id.to_string(),
            name: name.to_string(),
        };
    }
}
