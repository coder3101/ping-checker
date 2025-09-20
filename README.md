# Ping Checker

A lightweight HTTP service that performs ping connectivity checks via a REST API. This service is designed to be used by remote hosts (such as Tailscale nodes) to perform connectivity checks to homelab or other network destinations.

> Requires `net.ipv4.ping_group_range` sysctl parameter to allow unprivileged ICMP sockets. Check with: `sysctl net.ipv4.ping_group_range`
## Features

- Simple REST API for ping operations
- IPv4 and IPv6 support
- Configurable listen address
- Structured logging with tracing
- Docker-friendly design
- Debian package support
- Lightweight and fast

## Installation

### From Source

```bash
# Clone the repository
git clone <repository-url>
cd ping-checker

# Build the project
cargo build --release

# Run the binary
./target/release/ping-checker
```

### Using Cargo

```bash
cargo install --path .
ping-checker
```

## Usage

### Starting the Server

By default, the server listens on `0.0.0.0:8088`:

```bash
ping-checker
```

### Configuration

The server can be configured using environment variables:

- `LISTEN_ADDR`: The address and port to listen on (default: `0.0.0.0:8088`)

Example:
```bash
LISTEN_ADDR=127.0.0.1:3000 ping-checker
```

### API Endpoints

#### `GET /ping/{ip}`

Performs a ping to the specified IP address.

**Parameters:**
- `ip`: The IP address to ping (IPv4 or IPv6)

**Responses:**
- `200 OK`: Ping successful
- `400 Bad Request`: Invalid IP address format
- `503 Service Unavailable`: Ping failed (host unreachable, timeout, etc.)

**Examples:**

```bash
# Ping IPv4 address
curl http://localhost:8088/ping/8.8.8.8

# Ping IPv6 address
curl http://localhost:8088/ping/2001:4860:4860::8888

# Ping local address
curl http://localhost:8088/ping/192.168.1.1
```

## Deployment

### Systemd Service

The project includes a systemd service file for deployment:

```bash
# Copy binary to system location
sudo cp target/release/ping-checker /usr/bin/

# Copy and enable service (if using debian package)
sudo systemctl enable ping-checker
sudo systemctl start ping-checker
```

### Debian Package

The project supports building Debian packages:

```bash
# Install cargo-deb
cargo install cargo-deb

# Build debian package
cargo deb

# Install the package
sudo dpkg -i target/debian/ping-checker_*.deb
```


## Logging

The service uses structured logging with different levels:

- `INFO`: Server startup and basic operations
- `DEBUG`: Detailed ping operations
- `WARN`: Failed pings and invalid requests

Set log level with `RUST_LOG` environment variable:
```bash
RUST_LOG=debug ping-checker
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Author

Mohammad Ashar Khan

---
