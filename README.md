## Jikan-rs
Jikan-Rust is an unofficial Rust wrapper for the [Jikan API](https://jikan.moe/), an open-source API for MyAnimeList. This library allows you to seamlessly interact with MyAnimeList's public data without needing authentication



Jikan-rs uses reqwest that uses tokio for asynchronous tasks, so you will need the tokio runtime as well.

```
https://tokio.rs/

```
Add the following lines to your cargo.toml:
```
[dependencies]

reqwest = "0.12.12"
 
serde = { version = "1.0", features = ["derive"] }
 
tokio = { version = "1.0", features = ["full"] }
 
thiserror = "1.0"
 
chrono = { version = "0.4", features = ["serde"] }
 
url = "2.4"
 
serde_json = "1.0.137"
 
serial_test = "3.2.0"
 
async-trait = "0.1.85"
 
```
