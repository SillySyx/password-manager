FROM rust:latest as base
    RUN apt update
    RUN apt install -y build-essential libxcb-xfixes0-dev zip

FROM base as build
    WORKDIR /src