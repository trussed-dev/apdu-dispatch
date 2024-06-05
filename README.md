# Apdu Dispatch

This repository contains two crates:

- [`apdu-app`](./app): provides an `App` trait for applications that accept APDU requests
- [`apdu-dispatch`](./dispatch): a layer that accepts APDU (application packet data units) from a contact and/or contactless interface and passes them to apps implementing `apdu_app::App`.  It handles parsing APDU's, chaining, T=0, T=1 and keeps track of the selected application.

Run tests via `cargo test --features std,log-all`
