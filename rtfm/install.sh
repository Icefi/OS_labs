#!/bin/bash

cargo build --release

sudo mv target/release/rtfm /opt/rtfm/rtfm
sudo ln -s /opt/rtfm/rtfm /bin/rtfm
