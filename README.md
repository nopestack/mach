# Mach
Mach is a CLI tool designed to run WebAssembly modules blazingly fast at mach speed. 

## Overview
It can run a server with a REST API to upload and run a FaaS platform locally and it can also execute arbitrary WebAssembly modules as one-off tasks.

## Features
- **Fast and Efficient Execution**: Leverages the speed and lightweight nature of WASM for optimal performance.
- **Easy Deployment**: Simple CLI for deploying and managing WASM modules.
- **Scalable**: Designed to handle varying function calls concurrently, making it suitable for both small and large-scale applications.
- **Secure Isolation**: Mach runs each WASM module in isolated environments for enhanced security.

## Installation and Running

### Prerequisites
- Rust (latest nightly version)

### Installation from Source
To install Mach from source, follow these steps:

1. Clone the repository:
```
git clone https://github.com/nopestack/mach 
cd mach 
```

2. Build the project:
```
cargo build --release
```
3. Run the CLI:
The executable will be at
```
target/release/mach
```
