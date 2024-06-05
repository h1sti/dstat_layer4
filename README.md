
---
# Layer4 DStat in Rust

## Overview
This project is a Layer4 DStat (Distributed Statistics) tool built in Rust. It operates over TCP connections running on port 23. The tool is designed to monitor network statistics efficiently and securely.

## Features
- **High Performance**: Written in Rust for speed and safety.
- **TCP Connection**: Utilizes TCP connections for reliable data transmission.
- **Port 23**: Operates on port 23, traditionally used for Telnet services.
- **Real-time Statistics**: Provides real-time monitoring and reporting of network statistics.

## Installation
To install and run this Layer4 DStat tool, follow the steps below:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/layer4-dstat-rust.git
   cd layer4-dstat-rust
   ```

2. **Build the project**:
   Make sure you have Rust installed on your system. If not, you can get it from [rustup.rs](https://rustup.rs/).

   ```bash
   cargo build --release
   ```

3. **Run the application**:
   ```bash
   ./target/release/layer4-dstat
   ```

## Usage
After running the application, it will start listening on port 23. You can connect to it using any TCP client. For example, you can use `telnet`:

```bash
cargo run
```

Once connected, the tool will start displaying network statistics in real-time.


## Contributing
We welcome contributions from the community. If you'd like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch with a descriptive name.
3. Make your changes.
4. Submit a pull request.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
---
