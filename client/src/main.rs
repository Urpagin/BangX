use cpal::traits::StreamTrait;
use env_logger;
use log::{debug, error, info, warn};
use std::collections::VecDeque;
use std::process::exit;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::net::UdpSocket;

use tokio::task;

use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device, Host, SupportedStreamConfig,
};

const REMOTE_ADDR: &str = "127.0.0.1:8888";
const MAX_RETRIES: usize = 3;

type AudioQueue = mpsc::Sender<Vec<f32>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind the UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0").await?;

    socket
        .connect(REMOTE_ADDR)
        .await
        .expect("failed to connect");

    // Create a channel to pass audio data from the callback
    let (audio_tx, audio_rx) = mpsc::channel::<Vec<f32>>();

    // Start the audio input stream in a separate thread
    std::thread::spawn(move || {
        if let Err(e) = enqueue_audio(audio_tx) {
            eprintln!("Error enqueueing audio: {:?}", e);
        }
    });

    // Run the async task to send audio data
    let receiver_task = task::spawn(async move {
        send(socket, audio_rx).await;
    });

    receiver_task.await.unwrap();
    Ok(())
}

fn init_logging() {
    let mut builder = env_logger::builder();
    builder.filter_level(log::LevelFilter::Debug).init();
}

async fn send(socket: UdpSocket, audio_rx: mpsc::Receiver<Vec<f32>>) {
    while let Ok(data) = audio_rx.recv() {
        // Convert Vec<f32> to &[u8] for UDP sending
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * std::mem::size_of::<f32>(),
            )
        };

        if let Err(e) = socket.send(bytes).await {
            eprintln!("Failed to send data: {}", e);
        }
    }
}

fn enqueue_audio(audio_tx: AudioQueue) -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or("No input device available")?;
    let config = device.default_input_config()?;
    let sample_rate = config.sample_rate().0; // Sample rate in Hz (samples per second)
    println!("Sample rate: {} Hz", sample_rate);

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            // Send the audio data to the channel
            let data_vec = data.to_vec();

            let bytes_sent = data.len() * std::mem::size_of::<f32>();
            println!(
                "Buffer size (samples): {}, Buffer size (bytes): {}",
                data.len(),
                bytes_sent
            );

            if let Err(e) = audio_tx.send(data_vec) {
                eprintln!("Failed to send audio data: {:?}", e);
            }
        },
        move |err| {
            eprintln!("Error occurred: {:?}", err);
        },
        None,
    )?;

    stream.play()?;

    // Keep the thread alive to continue streaming
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

fn get_host() -> Host {
    cpal::default_host()
}

fn get_input_device(host: Host) -> Option<Device> {
    host.default_input_device()
}

fn get_input_config(device: &Device) -> Result<SupportedStreamConfig, Box<dyn std::error::Error>> {
    Ok(device.default_input_config()?)

    //let mut supported_configs_range = device.supported_input_configs()?;
    //
    //if let Some(config) = supported_configs_range.next() {
    //    return Ok(config.with_max_sample_rate());
    //} else {
    //    return Err("No supported config".into());
    //}
}
