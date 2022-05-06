use std::collections::HashMap;

use crate::custom_error::CustomError;
pub mod custom_error;
#[tokio::main]
async fn main() -> Result<()> {
    let first_parameter: i32 = std::env::args()
        .nth(1)
        .expect("sai roi, dung nhu nay ne reg 1 100")
        .parse()
        .expect("dung so thoi");
    let second_parameter: i32 = std::env::args()
        .nth(2)
        .expect("sai roi, dung nhu vay ne: reg 1 100")
        .parse()
        .expect("truyen so thoi");

    let client = reqwest::Client::new();
    for i in first_parameter..second_parameter {
        println!("creating account for {}", i);
        let result = create_account(&client, i).await;
        if result.is_err() {
            println!("Failed for id {}", result.unwrap_err());
        }
    }

    Ok(())
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

async fn create_account(client: &reqwest::Client, id: i32) -> Result<()> {
    let map = craft_body(id);

    let response = client
        .post("https://api.privato.chloeting.com/app/user")
        .json(&map)
        .send()
        .await?;
    if response.status().is_success() {
        return Ok(());
    }

    Err(Box::new(CustomError::BusinessErr(id.to_string())))
}

fn craft_body(id: i32) -> HashMap<&'static str, String> {
    let mut map = HashMap::new();
    map.insert("firstName", format!("firstname {}", id));
    map.insert("lastName", format!("lastname {}", id));
    map.insert("email", format!("hong{}@yopmail.com", id));
    map.insert("password", format!("Hong12345@"));
    map.insert("confirmPw", format!("Hong12345@"));
    map
}
