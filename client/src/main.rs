use env_logger;
use log::{debug, error, info, warn};
use std::{net::UdpSocket, os::unix::process, process::exit};

use cpal::{
    platform::DeviceInner,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Host, SupportedStreamConfig,
};

const REMOTE_ADDR: &str = "127.0.0.1:8888";
const MAX_RETRIES: usize = 3;

fn main() {
    init_logging();
    let _ = connect();
}

fn init_logging() {
    let mut builder = env_logger::builder();
    builder.filter_level(log::LevelFilter::Debug).init();
}

fn connect() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?; // Bind to any available port

    let mut retries: usize = 0;
    loop {
        retries += 1;
        if let Err(e) = socket.connect(REMOTE_ADDR) {
            warn!(
                "Failed to connect to {REMOTE_ADDR}: {}. Retrying {}/{}",
                e,
                retries + 1,
                MAX_RETRIES
            );
            retries += 1;
            if retries == MAX_RETRIES {
                exit(1);
            }
        } else {
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    if let Err(e) = socket.send(b"helo, server") {
        eprint!("error sending to socket: {e}");
    } else {
        println!("sent data to server");
    }

    let mut buf = [0; 1024];
    loop {
        println!("reading recv");
        if let Ok((read_bytes, src)) = socket.recv_from(&mut buf) {
            println!(
                "Received {} bytes from {}: {}",
                read_bytes,
                src,
                String::from_utf8_lossy(&buf[..read_bytes])
            );
        }
    }

    Ok(())
}

fn get_host() -> Host {
    cpal::default_host()
}

fn get_input_device(host: Host) -> Option<Device> {
    host.default_input_device()
}

fn get_input_config(device: Device) -> Result<SupportedStreamConfig, Box<dyn std::error::Error>> {
    let mut supported_configs_range = device.supported_input_configs()?;

    if let Some(config) = supported_configs_range.next() {
        return Ok(config.with_max_sample_rate());
    } else {
        return Err("No supported config".into());
    }
}

