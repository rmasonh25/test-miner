# Test Miner

A simulated Stratum miner client designed to test and communicate with the `solo-pool` server.

## Features

- Connects to a local solo mining pool (default `127.0.0.1:3333`)
- Sends `mining.subscribe` and `mining.authorize` messages
- Prints server responses
- Easy to extend for test harnesses

## Usage

1. Start your `solo-pool` server:
   ```bash
   cargo run
