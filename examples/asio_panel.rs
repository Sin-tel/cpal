use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

// Import the specific ASIO host module directly
#[cfg(target_os = "windows")]
use cpal::host::asio;

#[cfg(target_os = "windows")]
fn main() -> anyhow::Result<()> {
    let host = asio::Host::new()?;

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No ASIO device found"))?;

    println!("Opening control panel for: {}", device.name()?);

    let config = device.default_output_config()?;

    let err_fn = move |err| println!("Stream Error: {:?}", err);

    let stream = device.build_output_stream(
        &config.config(),
        move |_data: &mut [i32], _: &cpal::OutputCallbackInfo| { /* play silence */ },
        err_fn,
        None,
    )?;

    stream.play()?;

    if let Err(e) = device.open_control_panel() {
        eprintln!("Could not open panel: {:?}", e);
    }

    // Keep the thread alive so the window doesn't close immediately
    std::thread::sleep(std::time::Duration::from_secs(60));

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn main() {}
