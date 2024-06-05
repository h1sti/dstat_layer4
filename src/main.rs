mod tcp;
mod interface;
use std::net::{TcpListener};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use humansize::{format_size, DECIMAL};
use crate::interface::create_banner;
use crate::tcp::tcp_listen_for_connections;


pub struct AppState {
    pub counter_tcp: Mutex<u64>,
    pub highest_tcp: Mutex<u64>,
    pub data: Mutex<Vec<u64>>,
}

pub struct Listeners {
    pub tcp: TcpListener,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listeners = Listeners {
        tcp: TcpListener::bind("127.0.0.1:23")?,
    };

    let app_state = Arc::new(AppState {
        counter_tcp: Mutex::new(0u64),
        highest_tcp: Mutex::new(0u64),
        data: Mutex::new(vec![]),
    });

    let thread_state = app_state.clone();
    thread::spawn(move || {
        loop {
            {
                let mut counter_guard = thread_state.counter_tcp.lock().unwrap();
                let mut highest_tcp_guard = thread_state.highest_tcp.lock().unwrap();
                let banner_state = thread_state.clone();

                if *counter_guard > *highest_tcp_guard {
                    *highest_tcp_guard = *counter_guard;
                }

                let formatted_counter = format_size(*counter_guard, DECIMAL);
                let formatted_highest_tcp = format_size(*highest_tcp_guard, DECIMAL);

                create_banner(&formatted_counter, &formatted_highest_tcp, banner_state).expect("TODO: panic message");

                *counter_guard = 0;
            }
            thread::sleep(Duration::from_millis(1000));
        }
    });

    tcp_listen_for_connections(listeners.tcp, app_state).await;
    Ok(())
}
