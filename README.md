# Hello
🧊 a hello iced project by sleet for interacting with a hello conrtact on near

- this project got iced 🧊
- the orgional vision was to get the greeting from near conrtact, but there seems to be some issues, so i just get it form a local file instead. You can contribute to this app to update the default greeting.
- this app is just a hello app and is not meant to be packaged and distributed, you can clone and run with the commands below

---

### Development
running and building
```sh
# Main GUI application
cargo run
cargo run --bin hello

# CLI utility
cargo run --bin get_greeting

# Create release bundle :: Just for testing, this app is not meant to be released
cargo install cargo-bundle
cargo bundle --release

cargo clean
cargo check
cargo test
cargo update
```



---


copyright 2025 by sleet.near