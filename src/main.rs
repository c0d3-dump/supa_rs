use anyhow::{Ok, Result};
use supa_rs::client::SupabaseClient;

#[tokio::main]
async fn main() -> Result<()> {

    let mut client = SupabaseClient::new().load_env();

    let res = client.email_login("test-player@yopmail.com", "12345678").await.unwrap().data.unwrap();

     client.set_session(&res.access_token, &res.refresh_token).await;

    let user = client.user().await;
    
    println!("{:?}", user);

    Ok(())
}