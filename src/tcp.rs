use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc};
use tokio::task;
use crate::{AppState};

const HEADER_SIZE: usize = 4;


pub async fn tcp_listen_for_connections(listener: TcpListener, app_state: Arc<AppState>) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let app_state = app_state.clone();

                task::spawn(async move {
                    if let Err(e) = handle_connection(stream, app_state).await {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream, app_state: Arc<AppState>) -> anyhow::Result<()> {
    let mut header = [0u8; HEADER_SIZE];
    let mut counter = app_state.counter_tcp.lock().unwrap();
    let mut data = app_state.data.lock().unwrap();

    stream.read_exact(&mut header)?;
    let message_size = u32::from_be_bytes(header) as usize;

    *counter += message_size as u64;

    data.push(message_size as u64);

    let mut message = vec![0u8; message_size];
    stream.read_exact(&mut message)?;

    Ok(())
}