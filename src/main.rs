
mod tapo_control;
mod sensor_control;


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

// Use arm-unknown-linux-gnueabi to build

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tapo_control::test_tapo().await?;

    Ok(())
}