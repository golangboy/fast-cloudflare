use futures::future::join_all;
use indicatif::ProgressBar;
use std::io::{BufRead, Write};
use std::sync::Arc;
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence};
use tokio::io::AsyncWriteExt;
mod utils;
use rand::random;
use std::time::Duration;
async fn perform_first_stage() -> Result<(), Box<dyn std::error::Error>> {
    let cloudflare_ipv4_addresses = utils::get_all_ipv4().await?;
    let mut tasks = Vec::new();
    let live_ip_file = tokio::fs::File::create("result/live_ip.txt").await?;
    let live_ip_file_lock: Arc<tokio::sync::Mutex<tokio::fs::File>> =
        Arc::new(tokio::sync::Mutex::new(live_ip_file));
    let progress_bar = ProgressBar::new(cloudflare_ipv4_addresses.len() as u64);
    let progress_bar_lock = Arc::new(tokio::sync::Mutex::new(progress_bar));

    let ipv4_addresses_iter = cloudflare_ipv4_addresses.into_iter();
    let ipv4_addresses_iter_lock = Arc::new(tokio::sync::Mutex::new(ipv4_addresses_iter));
    for _ in 1..500 {
        let addresses_iter_clone = ipv4_addresses_iter_lock.clone();
        let live_ip_file_clone = live_ip_file_lock.clone();
        let progress_bar_clone = progress_bar_lock.clone();
        let task = tokio::spawn(async move {
            loop {
                let next_ipv4_address = addresses_iter_clone.lock().await.next();
                progress_bar_clone.lock().await.inc(1);
                match next_ipv4_address {
                    Some(mut ipv4_address) => {
                        let connection_attempt =
                            tokio::net::TcpStream::connect(ipv4_address.clone() + ":80");
                        let connection_result = tokio::time::timeout(
                            tokio::time::Duration::new(1, 0),
                            connection_attempt,
                        )
                        .await;
                        match connection_result {
                            Ok(result) => match result {
                                Ok(mut connection) => {
                                    let mut file_handle = live_ip_file_clone.lock().await;
                                    ipv4_address.push('\n');
                                    file_handle.write(ipv4_address.as_bytes()).await.unwrap();
                                    _ = connection.shutdown().await;
                                }
                                Err(_) => {}
                            },
                            Err(_) => {}
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
        });
        tasks.push(task);
    }
    join_all(tasks).await;
    live_ip_file_lock.lock().await.sync_all().await?;
    return Ok(());
}

async fn perform_second_stage() -> Result<(), Box<dyn std::error::Error>> {
    let ping_result_file = std::fs::File::create("result/ping_ip.txt").unwrap();
    let ping_result_file_lock = Arc::new(tokio::sync::Mutex::new(ping_result_file));
    let live_ip_file_content = std::fs::read_to_string("result/live_ip.txt").unwrap();
    let live_ipv4_addresses: Vec<String> = live_ip_file_content
        .split("\n")
        .filter(|&s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect();
    let progress_bar = ProgressBar::new(live_ipv4_addresses.len() as u64);
    let mut tasks = Vec::new();
    let live_ipv4_addresses_iter_lock =
        Arc::new(tokio::sync::Mutex::new(live_ipv4_addresses.into_iter()));
    for _ in 1..10 {
        let addresses_iter_clone = live_ipv4_addresses_iter_lock.clone();
        let ping_result_file_clone = ping_result_file_lock.clone();
        let progress_bar_clone = progress_bar.clone();
        let task = tokio::spawn(async move {
            loop {
                let next_ipv4_address = addresses_iter_clone.lock().await.next();
                progress_bar_clone.inc(1);
                match next_ipv4_address {
                    Some(ipv4_address) => {
                        let client_v4 = Client::new(&Config::default()).unwrap();
                        let mut pinger = client_v4
                            .pinger(ipv4_address.parse().unwrap(), PingIdentifier(random()))
                            .await;
                        pinger.timeout(Duration::from_secs(1));
                        let payload = &[1u8];
                        match pinger.ping(PingSequence::from(1), payload).await {
                            Ok((IcmpPacket::V4(_packet), duration)) => {
                                let mut result_line = String::from(ipv4_address);
                                result_line.push(' ');
                                result_line.push_str(&duration.as_millis().to_string());
                                result_line.push('\n');
                                let mut file_handle = ping_result_file_clone.lock().await;
                                file_handle.write(result_line.as_bytes()).unwrap();
                            }
                            Ok((IcmpPacket::V6(_packet), _duration)) => {}
                            Err(_) => {
                                let mut result_line = String::from(ipv4_address);
                                result_line.push_str(" 999999\n");
                                let mut file_handle = ping_result_file_clone.lock().await;
                                file_handle.write(result_line.as_bytes()).unwrap();
                            }
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
        });
        tasks.push(task);
    }
    join_all(tasks).await;
    ping_result_file_lock.lock().await.sync_all().unwrap();
    return Ok(());
}
fn sort_ping_results() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("result/ping_ip.txt")?;
    let reader = std::io::BufReader::new(file);

    let mut ping_results: Vec<(String, u64)> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let ip = parts[0].to_owned();
            let ms = parts[1].parse::<u64>().unwrap_or(0);
            ping_results.push((ip, ms));
        }
    }

    ping_results.sort_by(|a, b| a.1.cmp(&b.1));

    let mut sorted_file = std::fs::File::create("result/sorted_ping_ip.txt")?;
    for (ip, ms) in ping_results {
        writeln!(sorted_file, "{} {}", ip, ms)?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    _ = std::fs::create_dir("result");
    println!("Performing first stage");
    perform_first_stage().await?;
    println!("Performing second stage");
    perform_second_stage().await?;
    println!("Sorting results");
    sort_ping_results()?;
    println!("Done");
    return Ok(());
}
