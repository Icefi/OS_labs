#!/bin/bash

echo "INSTALLATION Rust SHell[2]"

echo "INSTALLING RSH[1/2]"
cd rsh
./install.sh

echo "DONE[1/2]"
echo "INSTALLING RTFM[2/2]"

cd ../rtfm
./install.sh

echo "DONE[2/2]"
echo "INSTALLATION FINISHED"
