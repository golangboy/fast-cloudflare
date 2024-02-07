use futures::future::join_all;
use indicatif::ProgressBar;
use std::sync::Arc;
use std::{io::Write, mem};
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence};
use tokio::io::AsyncWriteExt;
mod utils;
use rand::random;
use std::time::Duration;
async fn first_stage() -> Result<(), Box<dyn std::error::Error>> {
    let cf_ipv4 = utils::get_all_ipv4().await?;
    let mut v = Vec::new();
    _ = std::fs::create_dir("result");
    let result_fs = tokio::fs::File::create("result/live_ip.txt").await?;
    let result_fslock: Arc<tokio::sync::Mutex<tokio::fs::File>> =
        Arc::new(tokio::sync::Mutex::new(result_fs));
    let pb = ProgressBar::new(cf_ipv4.len() as u64);
    for ipv4 in cf_ipv4 {
        let mut ipv4_t = ipv4.clone();
        let result_fslock_t = result_fslock.clone();
        let a = tokio::spawn(async move {
            let _temp = tokio::net::TcpStream::connect(ipv4_t.clone() + ":80");
            let _temp2 = tokio::time::timeout(tokio::time::Duration::new(1, 0), _temp).await;
            match _temp2 {
                Ok(v) => match v {
                    Ok(mut _vv) => {
                        let mut _temp3 = result_fslock_t.lock().await;
                        ipv4_t.push('\n');
                        _temp3.write(ipv4_t.as_bytes()).await.unwrap();
                        _ = _vv.shutdown().await;
                    }
                    Err(_) => {}
                },
                Err(_) => {}
            }
        });
        if v.len() >= 200 {
            let v_temp = mem::replace(&mut v, Vec::new());
            join_all(v_temp).await;
            v.clear();
        } else {
            v.push(a);
        }
        pb.inc(1);
    }
    join_all(v).await;
    result_fslock.lock().await.sync_all().await?;
    return Ok(());
}

async fn second_stage() -> Result<(), Box<dyn std::error::Error>> {
    let tcp_result: String = std::fs::read_to_string("result/live_ip.txt").unwrap();
    let ping_result = std::fs::File::create("result/ping_ip.txt").unwrap();
    let ping_result_lock = Arc::new(tokio::sync::Mutex::new(ping_result));
    let ipv4s: Vec<&str> = tcp_result.split("\n").filter(|&s| !s.is_empty()).collect();
    let pb = ProgressBar::new(ipv4s.len() as u64);
    let mut wait_for_ping = Vec::new();
    for ipv4 in ipv4s {
        let mut ipv4_t = String::from(ipv4);
        let ping_result_lock_t = ping_result_lock.clone();
        let a = tokio::spawn(async move {
            let client_v4 = Client::new(&Config::default()).unwrap();
            let mut pinger = client_v4
                .pinger(ipv4_t.parse().unwrap(), PingIdentifier(random()))
                .await;
            pinger.timeout(Duration::from_secs(1));
            let payload = &[1u8];
            match pinger.ping(PingSequence::from(1), payload).await {
                Ok((IcmpPacket::V4(_packet), dur)) => {
                    ipv4_t.push(' ');
                    ipv4_t.push_str(&dur.as_millis().to_string());
                    ipv4_t.push('\n');
                    let mut t = ping_result_lock_t.lock().await;
                    t.write(ipv4_t.as_bytes()).unwrap();
                }
                Ok((IcmpPacket::V6(_packet), _dur)) => {}
                Err(_) => {
                    ipv4_t.push_str(" timeout\n");
                    let mut t = ping_result_lock_t.lock().await;
                    t.write(ipv4_t.as_bytes()).unwrap();
                }
            }
        });
        wait_for_ping.push(a);
        if wait_for_ping.len() > 10 {
            let wait_for_ping_t = mem::replace(&mut wait_for_ping, Vec::new());
            join_all(wait_for_ping_t).await;
            wait_for_ping.clear();
        }
        pb.inc(1);
    }
    join_all(wait_for_ping).await;
    return Ok(());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("first stage");
    first_stage().await?;
    println!("second stage");
    second_stage().await?;
    return Ok(());
}
