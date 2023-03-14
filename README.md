# DopplerCLI: Weather from the Terminal 🌦️

## About ℹ️

DopplerCLI is a simple CLI interface for obtaining weather information from the command line.

## Features 🔔

- Gets the daily weather for your city, automatically determined using your public IP address
- Choose what type of weather data you want to see via CLI arguments (TODO)
- Simple configuration for default program behavior and settings (TODO)

## Usage 🛠️

### Pre-requisites 💾

In order to run this project, you will need the Rust programming language installed on your machine, which should include Cargo. If you need to install this software, you can do so here: https://www.rust-lang.org/tools/install

### Installation ⚙️

1. Download this project via Git: `https://github.com/moconn68/dopplercli.git`
2. In the root directory, execute the command cargo run in your terminal.

- Alternatively, if you'd like to build a standalone executable to run anywhere without building with Cargo each time, you can run `cargo build --release` which should generate an executable at `src/target/release/dopplercli`.

3. That's it!

## Software used 👨‍💻

This project leverages the Rust programming language including it's robust toolchain. In addition, the following open-source dependencies are utilized:

- [`ipapi`](https://ipapi.co/): convenient site & API for getting simple public user info based on IP, no registration or API key required
- [`Open-Meteo`](https://open-meteo.com/): versatile HTTP weather API, no registration or API key required
- [`reqwest`](https://docs.rs/reqwest/latest/reqwest/): simple API for making HTTP requests
- [`serde-json`](https://github.com/serde-rs/json): parse JSON from HTTP responses
