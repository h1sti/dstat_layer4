use std::sync::{Arc};
use clearscreen::clear;
use colored::{ColoredString, Colorize};
use humansize::{DECIMAL, FormatSize};
use crate::AppState;

const BANNER: &str = r#"
        ██╗  ██╗ ██╗███████╗████████╗██╗
        ██║  ██║███║██╔════╝╚══██╔══╝██║
        ███████║╚██║███████╗   ██║   ██║
        ██╔══██║ ██║╚════██║   ██║   ██║
        ██║  ██║ ██║███████║   ██║   ██║
        ╚═╝  ╚═╝ ╚═╝╚══════╝   ╚═╝   ╚═╝
            Network Monitoring Tool
"#;
pub fn format_color(formatted_string: &str) -> ColoredString {
    let formatted_string = format!("{}s", formatted_string);

    match formatted_string {
        s if s.contains("k") => s.yellow(),
        s if s.contains("B") && !s.contains("M") => s.yellow(),
        s if s.contains("MB") => s.green(),
        s if s.contains("G") => s.bright_green(),
        _ => formatted_string.white(),
    }
}

pub fn create_banner(current_tcp: &str, highest_tcp: &str, app_state: Arc<AppState>) -> anyhow::Result<()> {
    let data = app_state.data.lock().unwrap();
    let average = if data.len() > 0 {
        data.iter().sum::<u64>() / data.len() as u64
    } else {
        0u64
    };

    let metrics = [
        ("Incoming TCP", format_color(current_tcp)),
        ("Highest TCP", format_color(highest_tcp)),
        ("Average TCP", format_color(&average.format_size(DECIMAL)))
    ];

    clear().expect("failed to clear screen");

    let max_length = metrics.iter()
        .map(|(label, value)| format!("{}: {}", label, value).len())
        .max()
        .unwrap_or(0);

    println!("{}", BANNER.bright_blue());

    for metric_set in metrics.chunks(3) {
        for (label, value) in metric_set {
            let output = format!("{}: {}", label, value);
            print!("{} | ", output);
        }
        println!();
    }

    Ok(())
}
