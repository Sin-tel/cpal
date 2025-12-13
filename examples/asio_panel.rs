#[cfg(all(windows, feature = "asio"))]
fn main() -> anyhow::Result<()> {
    use cpal::platform::DeviceInner;
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use cpal::HostId;

    let host = cpal::host_from_id(HostId::Asio)?;

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No ASIO device found"))?;

    println!(
        "Opening control panel for: {}",
        device.description()?.name()
    );

    let config = device.default_output_config()?;

    let err_fn = move |err| println!("Stream Error: {:?}", err);

    let stream = device.build_output_stream(
        &config.config(),
        move |_data: &mut [i32], _: &cpal::OutputCallbackInfo| { /* play silence */ },
        err_fn,
        None,
    )?;

    stream.play()?;

    let device = device.as_inner();

    if let DeviceInner::Asio(asio) = device {
        if let Err(e) = asio.open_control_panel() {
            eprintln!("Could not open panel: {:?}", e);
        }
    }

    // Keep the thread alive so the window doesn't close immediately
    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(())
}

#[cfg(not(all(windows, feature = "asio")))]
fn main() {}
