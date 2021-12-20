#!/bin/bash

echo "INSTALLATION My SHell[2]"

echo "INSTALLING MSH[1/2]"
cd msh
./install.sh

echo "DONE[1/2]"
echo "INSTALLING RTFM[2/2]"

cd ../rtfm
./install.sh

echo "DONE[2/2]"
echo "INSTALLATION FINISHED"
