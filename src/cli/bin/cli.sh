#!/bin/sh

set -e pipefail

if command -v cargo >/dev/null 2>&1; then
 : 
else
  echo "Please install cargo (https://rust-lang.org) and try again!"
fi

if command -v git >/dev/null 2>&1; then
  :
else
  echo "Please install git (https://git.org) and try again!"
fi

echo "The dir 'Jauri' will be overwritten (if it exists in current dir) in 3s..."
sleep 3

rm -rf Jauri >/dev/null 2>&1

git clone https://github.com/CirklAI/Jauri >/dev/null 2>&1
cd Jauri
mv src/cli .
rm -rf .idea/ src/ include/ .gitignore *.txt *.md *.xml
cd cli

echo "Building Jauri CLI, this may take a while..."
cargo build --release >/dev/null 2>&1
mv target/release/cli jauri
echo "Installing Jauri CLI... (you may be asked for a password!)"
sudo mv jauri /usr/local/bin/

echo "Cleaning up..."
cd ../../
rm -rf Jauri

echo "Run 'jauri' in your terminal to init a new project!"

