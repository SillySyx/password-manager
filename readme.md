Stores encrypted passwords on disk and copies decrypted passwords to clipboard.


## Features
* Unlock the app with an master key
* Lets you quickly copy a password into the clipboard


## Installation
Download the latest release and run the file, passwords will be stored in the same folder as the executable.

Windows requires vcruntime 140_1
https://aka.ms/vs/16/release/vc_redist.x64.exe


## Requirements
* Rust - https://rustup.rs


## Ubuntu requirements
`sudo apt install build-essential cmake`

if you have issues with x11 you may want to `sudo apt install librust-x11-dev`

if you have issues with servo-fontconfig-sys you may want to `sudo apt install libxft-dev`

if you have issues `/usr/bin/ld: cannot find -lxcb-xfixes` you may want to `sudo apt install libxcb-xfixes0-dev`


## Wayland issues
If you get an error with wayland-client when building you need to run `cargo update`