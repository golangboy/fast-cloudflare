use fastping_rs::PingResult::Receive;
use fastping_rs::Pinger;
use futures::future::join_all;
use indicatif::ProgressBar;
use std::mem;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
mod utils;

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
            let _temp2 = tokio::time::timeout(tokio::time::Duration::new(5, 0), _temp).await;
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
        if v.len() >= 1000 {
            let v_temp = mem::replace(&mut v, Vec::new());
            join_all(v_temp).await;
            v.clear();
            break;
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
    let (pinger, results) = match Pinger::new(None, Some(56)) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}", e),
    };

    let tcp_result: String = std::fs::read_to_string("result/live_ip.txt").unwrap();
    let ping_result = std::fs::File::create("result/ping_ip.txt").unwrap();
    let ipv4s: Vec<&str> = tcp_result.split("\n").filter(|&s| !s.is_empty()).collect();
    let pb = ProgressBar::new(ipv4s.len() as u64);
    for ipv4 in ipv4s {
        
        pb.inc(1);
    }
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
