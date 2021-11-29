#!/bin/bash

cargo build --release

sudo mv target/release/rsh /opt/rsh/rsh
sudo ln -s /opt/rsh/rsh /bin/rsh
