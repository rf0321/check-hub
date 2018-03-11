# Check-hub
![LICENCE](https://img.shields.io/packagist/l/doctrine/orm.svg)
[![Build Status](https://travis-ci.org/ItinoseSan/check-hub.svg?branch=master)](https://travis-ci.org/ItinoseSan/check-hub)

Toy CLI tool which can check github user information that implemented in Rust Lang
![peek 2018-03-09 20-56](https://user-images.githubusercontent.com/24353841/37239141-4c88fa30-2479-11e8-8e21-4d806b0d03c0.gif)
# Configuration
1.Clone this repository and move to this directory
```
git clone git@github.com:ItinoseSan/check-hub.git
cd check-hub
```
2.Please edit value of ```src/github_api.rs```
```rust
const GITHUB_USER_NAME:&str = "Your github user name";
const API_TOKEN:&str = "Your token";
```
3.Build project 
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
