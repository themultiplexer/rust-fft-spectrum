#!/bin/sh

# Register SIGINT Signal to kill process via SSH
trap "echo CTRL-C was pressed && ssh root@$1 'killall -9 fft' > 
/dev/null" 2

if [ $# -eq 1 ]
  then
    echo "Cross-Compiling and Installing to " $1
    cargo build --release --target=armv7-unknown-linux-gnueabihf
    echo "Killing running process"
    ssh root@$1 'killall -9 fft' > /dev/null
    echo "Copying executable"
    scp target/armv7-unknown-linux-gnueabihf/release/fft root@$1:/root/ 
> /dev/null
    echo "Running..."
    ssh root@$1 '/root/fft'
  else
    echo "./build.sh <IP_OF_RASPBERRY>"
fi


