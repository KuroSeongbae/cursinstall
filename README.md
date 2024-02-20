# Cursinstall
A TUI application which can add repos, sync, update and install packages defined in a json file.
No bash scripts for installing packages after setting up your new minimal Distro ;)

## Disclaimer
The app is still in early stage and might have some bugs. It also needs to be run as su as it does not support password input yet. I also want to refactor it to be a general command executor instead of just being able to install packages and so.

## Usage
1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone the repo and go to directory:
```
$ git clone https://github.com/KuroSeongbae/blahbla
$ cd cursinstall
```
3. run it!
```
cargo r
```
You can find an example json in the project :)
