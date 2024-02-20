extern crate reqwest;

use std::io::Read;

use ipnetwork::Ipv4Network;

pub async fn get_all_ipv4() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut cidr_strings: String = String::new();
    let file = std::fs::File::open("cloudflare_ipv4.txt").unwrap();
    let mut reader: std::io::BufReader<std::fs::File> = std::io::BufReader::new(file);
    reader.read_to_string(&mut cidr_strings).unwrap();
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
