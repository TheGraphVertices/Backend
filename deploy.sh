#!/bin/sh

if [ "$USER" != "root" ];
then
  echo "This script must be run as root; run `sudo ./deploy.sh`."
  exit 1
fi

echo "=== Installing rust. Follow Instructions on screen. ==="
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

echo "=== Installing diesel CLI and running migrations ==="
cargo install diesel_cli --no-default-features --features sqlite
diesel migration run

echo "
Installation finished. Edit the .env file to the port and host you want, then run 
`cargo r --release & disown` to compile and run the project."
