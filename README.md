# Description
Tips is a Rust CLI application that aims to provide an easy interface to retreive
and store information from the command line.

[![tips-demo-6.gif](https://i.postimg.cc/4ysR9sFg/tips-demo-6.gif)](https://postimg.cc/ft2P4nN2)

# Installation
- Clone repo
- cargo build --release --bin tips --target-dir <path>
- tips init

# Configure Tips
Edit ~/.tipsrc

The following environment variables are used to control specific behaviour:
- TIPS_SHOW_NOHEADER : When set Tips do not print Tip header when running 'show'
