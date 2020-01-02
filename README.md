# gpmdp_rc

GPMDP command line interface

## Installation

* Install Rust/Cargo
* Start GPMDP (it must be running for gpmdp_rc to work)

```
% git clone https://github.com/insanum/gpmdp_rc 
% cd gpmdp_rc
% cargo build
% cat > ~/.gpmdp_rc
url: ws://127.0.0.1:5672
^D
% ./target/debug/gpmdb_rc -c ~/.gpmdb_rc auth
```

Follow the authentication instructions to get the auth token.

```
% cat >> ~/.gpmdp_rc
token: <auth_token>
^D
```

## Usage

```
Usage: gpmdp_rc -c <config_file> <command> [ args ]
  auth
  status
  play [ <track#> ]
  pause
  next
  prev
  replay
  seek [ +<secs> | -<secs> | forward | backward ]
  lyrics
  thumbs < up | down >
  shuffle < on | off >
  repeat < all | single | off >
  queue
  clear
  playlists
  playlist <playlist#>
  search "<text>"
  results [ <result#> ]
  volume [ <0-100> | up | down ]
```

