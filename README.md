# Check-hub
Github check user information CLI tool implemented in Rust Lang
![peek 2018-03-09 20-56](https://user-images.githubusercontent.com/24353841/37239141-4c88fa30-2479-11e8-8e21-4d806b0d03c0.gif)
# Configration
Please edit value of ```src/github_api.rs```
```rust
const GITHUB_USER_NAME:&str = "Your user name";
const API_TOKEN:&str = "Your token";
```
Build project 
```
cargo build
```
After that, move to target/debug directory and ./check-hub bianary in there move to your favorite path
# Usage
usage format
```
./check-hub <information>
````
When you check information name 
````
./check-hub -h or ./check-hub --help
````
# Contributing
I welcome it. Im waiting your issue or PR
# LICENCE
This cli tool that released under the MIT licence.
