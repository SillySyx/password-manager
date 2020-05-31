# Building

Linux
```
docker build -f Dockerfile-linux -t passwordmanager-builder:linux .

docker run \
    --rm \
    -v ${PWD}:/usr/src/app \
    -e APP_VERSION='1.0' \
    passwordmanager-builder:linux
```


# Requirements
http://gtk-rs.org/docs/requirements.html

```
sudo apt-get install libgtk-3-dev
```