FROM rust:1.79.0-alpine

RUN apk update && apk add --no-cache clang git curl htop tmux nushell
