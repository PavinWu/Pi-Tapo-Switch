
use std::env;
use tapo::{ApiClient, requests::Color};

#[allow(dead_code)]
pub async fn test_tapo() -> Result<(), Box<dyn std::error::Error>> {
    // TODO Set environment variables automatically
    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;

    // TODO multiple devices
    // TODO Proper handling error when these don't exist
    let ip_address = env::var("TAPO_IP_ADDRESS")?;

    println!("Tapo username: {}", tapo_username);
    println!("Tapo IP address: {}", ip_address);

    dbg!("Connecting to Tapo device at...");
    let device = ApiClient::new(tapo_username, tapo_password)
        .l530(ip_address)
        .await?;

    println!("Turning device on...");
    device.on().await?;

    println!("Setting the brightness to 30%...");
    device.set_brightness(30).await?;

    println!("Setting the color to `Chocolate`...");
    device.set_color(Color::Chocolate).await?;

    println!("Setting the color to `Chocolate`...");
    device.off().await?;

    Ok(())
}