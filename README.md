# Testouille de rust

[![Build Status](https://github.com/mpatron/hello-rocket/workflows/Rust/badge.svg)](https://github.com/mpatron/hello-rocket/actions)
[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/cgbur/badge-maker/blob/master/LICENSE

## Update

Update rust

~~~bash
mickael@deborah:~/Documents/hello-rocket$ [[ -f $(which rustup) ]] && rustup upgrade || echo "Rust non installé"
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
info: checking for self-update

  stable-x86_64-unknown-linux-gnu unchanged - rustc 1.81.0 (eeb90cda1 2024-09-04)

info: cleaning up downloads & tmp directories
~~~

Update dependencies in project

~~~bash
mickael@deborah:~/Documents/hello-rocket$ cargo update
    Updating crates.io index
    Updating git repository `https://github.com/lawliet89/rocket_cors`
     Locking 121 packages to latest compatible versions
    Updating addr2line v0.21.0 -> v0.24.2
    Removing adler v1.0.2
      Adding adler2 v2.0.0
    Updating aes v0.8.3 -> v0.8.4
...
    Updating windows_x86_64_gnullvm v0.52.0 -> v0.52.6
    Updating windows_x86_64_msvc v0.52.0 -> v0.52.6
    Updating winnow v0.5.30 -> v0.6.20
      Adding zerocopy v0.7.35 (latest: v0.8.5)
      Adding zerocopy-derive v0.7.35 (latest: v0.8.5)
note: pass `--verbose` to see 24 unchanged dependencies behind latest
mickael@deborah:~/Documents/hello-rocket$
~~~

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

~~~bash
# https://sagiegurari.github.io/cargo-make/
mpatron@mylinux:hello-rocket$ cargo install --no-default-features --force cargo-make
mpatron@mylinux:hello-rocket$ cargo make my-flow
~~~

See
[https://github.com/wpcodevo/simple-api-rocket/blob/master/src/handler.rs](https://github.com/wpcodevo/simple-api-rocket/blob/master/src/handler.rs)

~~~bash
cargo install cargo-watch
cargo watch -x run
curl -X POST http://localhost:8000/users -H 'Content-Type: application/json' -d "{\"id\": 123,\"name\": \"toto\",\"email\": \"toto@toto.fr\"}"
curl -X POST http://localhost:8000/users -H 'Content-Type: application/json' -d "{\"id\": 456,\"name\": \"tutu\",\"email\": \"tutu@tutu.fr\"}"
curl -X POST http://localhost:8000/users -H 'Content-Type: application/json' -d "{\"id\": 789,\"name\": \"tata\",\"email\": \"tata@tata.fr\"}"
curl http://localhost:8000/users -H 'Content-Type: application/json'
curl -v -X POST http://localhost:8000/users -H 'Content-Type: application/json' -d "{\"id\": 124,\"name\": \"toto\",\"email\": \"toto@toto.fr\"}"
~~~

Construction de l'image docker avec podman

~~~bash
podman build -t rust-alpine-app .
podman run -p 8000:8000 --rm rust-alpine-app
~~~
