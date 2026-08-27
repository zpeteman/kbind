#!/usr/bin/env bash

echo "Uninstalling kbind (kb)..."

# Remove binaries
rm -f ~/.cargo/bin/kb
sudo rm -f /usr/bin/kb 2>/dev/null
sudo rm -f /usr/local/bin/kb 2>/dev/null

# Remove configs and keys
rm -rf ~/.config/kbind

# Remove shell hooks from rc files
if [ -f ~/.zshrc ]; then
    sed -i.bak '/kbind/d' ~/.zshrc
    echo "Removed kbind hook from ~/.zshrc"
fi

if [ -f ~/.bashrc ]; then
    sed -i.bak '/kbind/d' ~/.bashrc
    echo "Removed kbind hook from ~/.bashrc"
fi

if [ -f ~/.config/fish/config.fish ]; then
    sed -i.bak '/kbind/d' ~/.config/fish/config.fish
    echo "Removed kbind hook from config.fish"
fi

echo "Successfully uninstalled kbind!"
