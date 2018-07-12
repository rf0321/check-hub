# Check-hub
![LICENCE](https://img.shields.io/packagist/l/doctrine/orm.svg)

Toy CLI tool which can check github user information that implemented in Rust Lang


# Build Status
| version | status |
----|---- 
| nightly |[![Build Status](https://travis-ci.org/ItinoseSan/check-hub.svg?branch=master)](https://travis-ci.org/ItinoseSan/check-hub)|
| beta   |[![Build Status](https://travis-ci.org/ItinoseSan/check-hub.svg?branch=master)](https://travis-ci.org/ItinoseSan/check-hub)|
| stable| [![Build Status](https://travis-ci.org/ItinoseSan/check-hub.svg?branch=master)](https://travis-ci.org/ItinoseSan/check-hub)|

![peek 2018-03-09 20-56](https://user-images.githubusercontent.com/24353841/37239141-4c88fa30-2479-11e8-8e21-4d806b0d03c0.gif)
# Configuration
1.Clone this repository and move to this directory
```
git clone git@github.com:ItinoseSan/check-hub.git
cd check-hub
```
2.Please touch ```apiconfig.yml```  config file  in ```target/debug```
```
cd target/debug
touch apiconfig.yml
```
3.Edit this yml file like a below

```yaml
name: <github login name>
GITHUB_API_TOKEN: YOUR_GITHUB_API_TOKEN
```

4.Build project 
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
