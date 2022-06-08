#![feature(io_read_to_string)]
#![allow(while_true)]
use std::str::FromStr;

use failing::api::user::get_user_handler;

pub fn main() -> std::io::Result<()> {
    while true {
        println!("Please enter the api input:");
        let mut raw_json = String::new();
        std::io::stdin().read_line(&mut raw_json)?;
        let json = serde_json::Value::from_str(&raw_json).expect("Invalid JSON");

        let result = match json {
            serde_json::Value::Object(map) => get_user_handler(
                map.into_iter()
                    .map(|(k, v)| (k, v.as_str().unwrap().to_string()))
                    .collect(),
            ),
            _ => panic!("Expected a JSON object"),
        };

        match result {
            Ok(id) => println!("Fetched id: {:?}", id),
            Err(error) => eprintln!("Error: {:?}", error),
        }
        print!("\n");
    }
    Ok(())
}
