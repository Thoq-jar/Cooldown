mod sh;
mod constants;

use crate::constants::{NAME, THRESHOLD, VERSION};

fn main() {
    println!("Starting {} v{}...", NAME, VERSION);

    if sh::sh("macmon", "-h").contains("command not found") {
        println!("Macmon is not installed! Install it with `brew install macmon`");
        std::process::exit(1);
    }

    println!();

    let mut last_notification_time = 0;

    loop {
        let cpu_temp = sh::sh(
            "macmon",
            "pipe -s 1 -i 500 | jq",
        ).lines()
            .find(|line| line.contains("cpu_temp_avg"))
            .unwrap_or_default()
            .replace(",", "")
            .replace("\"cpu_temp_avg\":", "")
            .trim()
            .to_string();

        if cpu_temp.is_empty() {
            println!("Failed to get CPU temperature!");
            continue;
        }

        let cpu_temp: f32 = match cpu_temp.parse::<f32>() {
            Ok(temp) => temp.round(),
            Err(_) => {
                println!("Failed to parse temperature: {}", cpu_temp);
                continue;
            }
        };

        println!("Current CPU temperature: {}˚C", cpu_temp);

        if cpu_temp > THRESHOLD && std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - last_notification_time >= 60 {
            push_notification("Consider taking a break, CPU is going to throttle soon!");
            last_notification_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
    }
}

fn push_notification(message: &str) {
    use notify_rust::Notification;
    Notification::new()
        .summary(NAME)
        .body(message)
        .show().expect("Failed to push notification");
}