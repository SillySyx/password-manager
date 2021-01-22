Stores encrypted passwords on disk and copies decrypted passwords to clipboard.


## Features
* Unlock your passwords with an master key
* Copies passwords to the clipboard


## Installation
Download the latest [release](https://github.com/SillySyx/password-manager/releases) and run the file.  
Passwords will be stored in the same folder as the executable.


## Build
```
docker build -t passwordmanager:builder .
docker run -it --rm -v $(pwd):/src passwordmanager:builder
```

> In windows to get the current directory you need to use `%cd%` in command prompt or `${PWD}` in powershell.