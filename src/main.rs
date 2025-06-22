use std::env;
use std::io;

// use log::info;
use tapo::{ApiClient, requests::Color};


// TODO Process
// 1. Test: the tapo crate 
// 2. Test rppal
// 3. design
// 4. implement

// TODO
/*
- On startup, setup APDS9960 
    - enable proximity and the interrupt.
    - connect to Tapo device.
    - periodic 'petting' of the Tapo device.
- GPIO connected to interrupt pin of APDS9960.
- Has infinite loop in main that checks for 'process_gesture' flag.
    - Once flag is set, enable the gesture mode.
    - Process the gesture TODO and perform appropriate tapo action.
    - Timeout after x seconds/minutes, and turns process_gesture off.
- ISR of the GPIO pin to toggle 'process_gesture' flag.

- What to do if  the Tapo device is not reachable?
*/

#[allow(dead_code)]
async fn test_tapo() -> Result<(), Box<dyn std::error::Error>> {
    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;
    let ip_address = env::var("IP_ADDRESS")?;

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

    Ok(())
}

#[allow(dead_code)]
fn test_apds9960() -> Result<(), io::Error>
{
    // let mut sensor = APDS9960::new()?;
    // sensor.enable_proximity()?;
    // sensor.enable_color()?;
    // sensor.enable_gesture()?;

    // loop {
    //     if let Some(proximity) = sensor.read_proximity()? {
    //         println!("Proximity: {}", proximity);
    //     }
    //     if let Some(color) = sensor.read_color()? {
    //         println!("Color: {:?}", color);
    //     }
    //     if let Some(gesture) = sensor.read_gesture()? {
    //         println!("Gesture: {:?}", gesture);
    //     }
    // }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // test_tapo().await?;

    Ok(())
}