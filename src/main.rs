use std::collections::HashMap;

use crate::custom_error::CustomError;
pub mod custom_error;
#[tokio::main]
async fn main() -> Result<()> {
    let std_in = std::io::stdin();
    println!("Nhap so bat dau: ");
    let mut first_parameter: String = "".into();
    std_in
        .read_line(&mut first_parameter)
        .expect("sai roi, nhap so thoi");
    let first_parameter: i32 = first_parameter.trim().parse().expect("nhap so thoi");

    let mut second_parameter: String = "".into();
    println!("Nhap so ket thuc: ");
    std_in
        .read_line(&mut second_parameter)
        .expect("nhap so thoi");
    let second_parameter: i32 = second_parameter.trim().parse().expect("nhap so thoi");

    let client = reqwest::Client::new();
    for i in first_parameter..second_parameter {
        println!("Tao account id {}", i);
        let result = create_account(&client, i).await;
        if result.is_err() {
            println!("That bai id {}", result.unwrap_err());
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
