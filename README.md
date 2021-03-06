![loraxe](./img/logo.png)


[![Build Status](https://travis-ci.org/ajmwagar/lor-axe.svg?branch=master)](https://travis-ci.org/ajmwagar/lor-axe)
[![codecov](https://codecov.io/gh/ajmwagar/lor-axe/branch/master/graph/badge.svg)](https://codecov.io/gh/ajmwagar/lor-axe)
[![dependency status](https://deps.rs/repo/github/ajmwagar/lor-axe/status.svg)](https://deps.rs/repo/github/ajmwagar/lor-axe)


`loraxe` is a multi-threaded, low-bandwidth HTTP DOS tool that handles connections and sockets in parallel. __Written with 100% Safe Rust__ 

## 🎁 Features

- Fully Configurable CLI options via `structopt`
- Colored logs, via `pretty_env_logger` (set `RUST_LOG` for more verbose logging)
- Sockets are handled in parallel via `rayon`
- Low-bandwidth usage
- Built in DNS
- SSL (Coming Soon)

### 🖧 DOS Modes

- __Slow HTTP (Slowloris)__: Holds connections open by slowly completing the http request after sending a complete header.
- __Slow POST__: Sends a POST request with a content length of `1m` and then sends random data at a rate of 1 byte / <delay>.
- __[WIP] Slow READ__: Requests a file larger than a servers given send buffer (~65Kb - 124Kb) and then reads the result at a user defined rate.
- __UDP Flood mode__: Sends a UDP packet to all ports on a given host. If the port is closed the host must respond with an error saying `ICMP error packet`. Spamming UDP connections on closed ports can cripple **some** (as reddit user /u/UndeniablyRexer pointed out) machines very quickly.


## 📦 Installation

The installation of `lor-axe` is easy if you have `cargo` installed.


**Build from source:**

```bash
git clone https://github.com/ajmwagar/lor-axe
cd lor-axe
cargo install --path .
loraxe --help
```

**Use `cargo`:**

```bash
cargo install loraxe
loraxe --help
```

## 💯 Usage

```bash
# Start a Slowloris attack on 0.0.0.0:8080 with 200 concurrent connections
loraxe 0.0.0.0 -p 8080 -s 200

# Start a HTTP POST attack on 0.0.0.0:80 with 150 concurrent connections
loraxe 0.0.0.0 --post

# Start a Slow READ attack on 0.0.0.0:80 with 150 concurrent connections and a read buffer of 8 bytes
loraxe 0.0.0.0 --read -b 8

# Start a UDP flood attack on 0.0.0.0 with 150 concurrent connections.
loraxe 0.0.0.0 -f

# Print a help menu
loraxe --help
```

## Disclaimer

Any actions and or activities related to the code provided is solely your responsibility. The misuse of the information in this website can result in criminal charges brought against the persons in question. The authors will not be held responsible in the event any criminal charges be brought against any individuals misusing the information in this tool to break the law.


