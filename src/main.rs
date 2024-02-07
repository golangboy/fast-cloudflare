use futures::future::join_all;
use indicatif::ProgressBar;
use std::io::Write;
use std::sync::Arc;
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence};
use tokio::io::AsyncWriteExt;
mod utils;
use rand::random;
use std::time::Duration;

async fn first_stage() -> Result<(), Box<dyn std::error::Error>> {
    let cf_ipv4 = utils::get_all_ipv4().await?;
    let mut v = Vec::new();
    let result_fs = tokio::fs::File::create("result/live_ip.txt").await?;
    let tcp_result_fslock: Arc<tokio::sync::Mutex<tokio::fs::File>> =
        Arc::new(tokio::sync::Mutex::new(result_fs));
    let pb = ProgressBar::new(cf_ipv4.len() as u64);
    let pb_t = Arc::new(tokio::sync::Mutex::new(pb));

    let ipv4_int = cf_ipv4.into_iter();
    let ipv4_int_t = Arc::new(tokio::sync::Mutex::new(ipv4_int));
    for _i in 1..500 {
        let _temp = ipv4_int_t.clone();
        let _result_fslock = tcp_result_fslock.clone();
        let _pb_t = pb_t.clone();
        let a = tokio::spawn(async move {
            let __temp = _temp.clone();
            let __result_fslock = _result_fslock.clone();
            let __pb_t = _pb_t.clone();
            loop {
                let ipv4_t = __temp.lock().await.next();
                __pb_t.lock().await.inc(1);
                match ipv4_t {
                    Some(mut ipv4_addr) => {
                        let _temp = tokio::net::TcpStream::connect(ipv4_addr.clone() + ":80");
                        let _temp2 =
                            tokio::time::timeout(tokio::time::Duration::new(1, 0), _temp).await;
                        match _temp2 {
                            Ok(v) => match v {
                                Ok(mut _vv) => {
                                    let mut _temp3 = __result_fslock.lock().await;
                                    ipv4_addr.push('\n');
                                    _temp3.write(ipv4_addr.as_bytes()).await.unwrap();
                                    _ = _vv.shutdown().await;
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
        v.push(a);
    }
    join_all(v).await;
    tcp_result_fslock.lock().await.sync_all().await?;
    return Ok(());
}

async fn second_stage() -> Result<(), Box<dyn std::error::Error>> {
    let ping_result = std::fs::File::create("result/ping_ip.txt").unwrap();
    let ping_result_lock = Arc::new(tokio::sync::Mutex::new(ping_result));
    let tcp_result = std::fs::read_to_string("result/live_ip.txt").unwrap();
    let ipv4s: Vec<String> = tcp_result
        .split("\n")
        .filter(|&s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect();
    let pb = ProgressBar::new(ipv4s.len() as u64);
    let mut v = Vec::new();
    let ipv4_t = Arc::new(tokio::sync::Mutex::new(ipv4s.into_iter()));
    for _i in 1..10 {
        let _ipv4 = ipv4_t.clone();
        let _ping_result_lock = ping_result_lock.clone();
        let _pb = pb.clone();
        let a = tokio::spawn(async move {
            loop {
                let ipv4 = _ipv4.lock().await.next();
                let __ping_result_lock = _ping_result_lock.clone();
                let __pb = _pb.clone();
                __pb.inc(1);
                match ipv4 {
                    Some(ipv4) => {
                        let client_v4 = Client::new(&Config::default()).unwrap();
                        let mut pinger = client_v4
                            .pinger(ipv4.parse().unwrap(), PingIdentifier(random()))
                            .await;
                        pinger.timeout(Duration::from_secs(1));
                        let payload = &[1u8];
                        match pinger.ping(PingSequence::from(1), payload).await {
                            Ok((IcmpPacket::V4(_packet), dur)) => {
                                let mut ipv4_t = String::from(ipv4);
                                ipv4_t.push(' ');
                                ipv4_t.push_str(&dur.as_millis().to_string());
                                ipv4_t.push('\n');
                                let mut t = __ping_result_lock.lock().await;
                                t.write(ipv4_t.as_bytes()).unwrap();
                            }
                            Ok((IcmpPacket::V6(_packet), _dur)) => {}
                            Err(_) => {
                                let mut ipv4_t = String::from(ipv4);
                                ipv4_t.push_str(" timeout\n");
                                let mut t = __ping_result_lock.lock().await;
                                t.write(ipv4_t.as_bytes()).unwrap();
                            }
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
        });
        v.push(a);
    }
    join_all(v).await;
    ping_result_lock.lock().await.sync_all().unwrap();
    return Ok(());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    _ = std::fs::create_dir("result");
    println!("first stage");
    first_stage().await?;
    println!("second stage");
    second_stage().await?;
    return Ok(());
}
