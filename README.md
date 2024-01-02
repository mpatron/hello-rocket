# Testouille de rust

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
[![Build Status](https://github.com/mpatron/hello-rocket/workflows/rust/badge.svg)](https://github.com/mpatron/hello-rocket/actions)

## Test

~~~bash
mpatron@mylinux:hello-rocket$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/main.rs (target/debug/deps/hello_rocket-f94f7957440695b6)

running 1 test
test test::hello_world ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

mpatron@mylinux:hello-rocket$ 
~~~

## Construction d'une release

~~~bash
mpatron@mylinux:hello-rocket$ cargo build --all-targets --release 
   Compiling proc-macro2 v1.0.71
   Compiling unicode-ident v1.0.12
   ....
   Compiling toml_edit v0.21.0
   Compiling h2 v0.3.22
   Compiling toml v0.8.8
   Compiling hello-rocket v0.1.0 (/home/mpatron/hello-rocket)
    Finished release [optimized] target(s) in 36.74s
mpatron@mylinux:hello-rocket$ 
~~~

## Exécution

~~~bash
mpatron@mylinux:hello-rocket$ cargo run 
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/hello-rocket`
🔧 Configured for debug.
   >> address: 127.0.0.1
   >> port: 8000
   >> workers: 16
   >> max blocking threads: 512
   >> ident: Rocket
   >> IP header: X-Real-IP
   >> limits: bytes = 8KiB, data-form = 2MiB, file = 1MiB, form = 32KiB, json = 1MiB, msgpack = 1MiB, string = 8KiB
   >> temp dir: /tmp
   >> http/2: true
   >> keep-alive: 5s
   >> tls: disabled
   >> shutdown: ctrlc = true, force = true, signals = [SIGTERM], grace = 2s, mercy = 3s
   >> log level: normal
   >> cli colors: true
📬 Routes:
   >> (hello) GET /
📡 Fairings:
   >> Shield (liftoff, response, singleton)
🛡️ Shield:
   >> Permissions-Policy: interest-cohort=()
   >> X-Content-Type-Options: nosniff
   >> X-Frame-Options: SAMEORIGIN
🚀 Rocket has launched from http://127.0.0.1:8000
GET /:
   >> Matched: (hello) GET /
   >> Outcome: Success(200 OK)
   >> Response succeeded.
~~~

~~~bash
mpatron@mylinux:hello-rocket$ curl -v http://127.0.0.1:8000
*   Trying 127.0.0.1:8000...
* Connected to 127.0.0.1 (127.0.0.1) port 8000 (#0)
> GET / HTTP/1.1
> Host: 127.0.0.1:8000
> User-Agent: curl/7.81.0
> Accept: */*
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-type: text/plain; charset=utf-8
< server: Rocket
< permissions-policy: interest-cohort=()
< x-content-type-options: nosniff
< x-frame-options: SAMEORIGIN
< content-length: 13
< date: Tue, 02 Jan 2024 10:03:16 GMT
< 
Hello, world!
* Connection #0 to host 127.0.0.1 left intact
mpatron@mylinux:hello-rocket$ 
~~~

## Builder

~~~txt
https://sagiegurari.github.io/cargo-make/
cargo install --no-default-features --force cargo-make
cargo make my-flow
~~~
