# Cursinstall
A TUI application which can run commands defined in a json file.
Maybe an alternative to your bash scripts ;)

## Disclaimer
The app is still in early stage and might have some bugs. It also needs to be run as `su` as it does not support password input yet.

## Usage
1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone the repo and go to directory:
```
$ git clone https://github.com/KuroSeongbae/cursinstall 
$ cd cursinstall
```
3. Change to `su` (Optional) and run it!
```
$ su
# cargo r
```
You can find an [example json](https://github.com/KuroSeongbae/cursinstall/blob/main/commands.json) in the project :)
