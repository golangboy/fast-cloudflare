[中文版](./README_zh.MD) | [English](./README.MD)
# Cloudflare IPv4 Address Optimizer

This Rust-based application is designed to optimize your Cloudflare experience by helping you identify the IPv4 address with the lowest latency. The program operates in two stages, scanning all of Cloudflare's IP segments and ultimately identifying the IP address that responds the fastest.

## Overview

- **Stage 1:** The application initiates a TCP connection to each IPv4 address provided by Cloudflare on port 80. All addresses that are reachable are stored for the next stage.
- **Stage 2:** The application sends a ping request to each IP address that was reachable in the first stage. The response times are recorded, and the IP address with the lowest latency is identified.

The results of each stage are stored in a `result` directory that the application creates. After the first stage, reachable IP addresses are stored in `live_ip.txt`. After the second stage, the ping results are stored in `ping_ip.txt`.

## Usage

To run this program, you need Rust and Cargo installed on your machine. Once these are installed, you can run the program using the command `cargo run`.

Please note that the `live_ip.txt` and `ping_ip.txt` files will be created in the `result` directory, which is automatically created by the program in the location where it is run.

## Contributing

Contributions are welcome! Please feel free to submit a pull request.

## License

This project is licensed under the terms of the MIT license.