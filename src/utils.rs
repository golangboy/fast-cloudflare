extern crate reqwest;

use std::io::{Read, Write};

use ipnetwork::Ipv4Network;
use reqwest::header;

pub async fn get_all_ipv4() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let cidr_strings_result = get_cidr_from_cf().await;
    let mut cidr_strings: String = String::new();
    match cidr_strings_result {
        Ok(v) => {
            cidr_strings = v;
            let file = std::fs::File::create("cloudflare_ipv4.txt").unwrap();
            let mut writer: std::io::BufWriter<std::fs::File> = std::io::BufWriter::new(file);
            writer.write_all(cidr_strings.as_bytes()).unwrap();
        }
        Err(err) => {
            println!("Error: {}", err.to_string());
            let file = std::fs::File::open("cloudflare_ipv4.txt").unwrap();
            let mut reader: std::io::BufReader<std::fs::File> = std::io::BufReader::new(file);
            reader.read_to_string(&mut cidr_strings).unwrap();

        }
    }
    let cidr_list = cidr_strings.split("\n");
    let mut ipv4_addresses = Vec::<String>::new();
    for cidr in cidr_list {
        let network: Ipv4Network = cidr.parse().unwrap();
        for address in network.iter() {
            ipv4_addresses.push(String::from(address.to_string()));
        }
    }
    return Ok(ipv4_addresses);
}

pub async fn get_cidr_from_cf() -> Result<String, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("authority", "www.cloudflare.com".parse().unwrap());
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
    headers.insert("accept-language", "zh-CN,zh;q=0.9".parse().unwrap());
    headers.insert("cache-control", "no-cache".parse().unwrap());
    headers.insert("pragma", "no-cache".parse().unwrap());
    headers.insert(
        "referer",
        "https://www.cloudflare.com/zh-cn/ips/".parse().unwrap(),
    );
    headers.insert(
        "sec-ch-ua",
        "\"Not A(Brand\";v=\"99\", \"Google Chrome\";v=\"121\", \"Chromium\";v=\"121\""
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-arch", "\"arm\"".parse().unwrap());
    headers.insert("sec-ch-ua-bitness", "\"64\"".parse().unwrap());
    headers.insert(
        "sec-ch-ua-full-version",
        "\"121.0.6167.139\"".parse().unwrap(),
    );
    headers.insert("sec-ch-ua-full-version-list", "\"Not A(Brand\";v=\"99.0.0.0\", \"Google Chrome\";v=\"121.0.6167.139\", \"Chromium\";v=\"121.0.6167.139\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-model", "\"\"".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"macOS\"".parse().unwrap());
    headers.insert("sec-ch-ua-platform-version", "\"14.3.0\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36".parse().unwrap());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client
        .get("https://www.cloudflare.com/ips-v4/")
        .headers(headers)
        .send()
        .await?;

    Ok(res.text().await.unwrap())
}
