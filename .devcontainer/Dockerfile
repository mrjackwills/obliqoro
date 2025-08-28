FROM mcr.microsoft.com/devcontainers/base:ubuntu-24.04

RUN printf "alias cls='clear'\nalias ll='ls -l --human-readable --color=auto --group-directories-first --classify --time-style=long-iso -all'\n" >> /etc/bash.bashrc

ENV PATH="/home/vscode/.cargo/bin:${PATH}"

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    build-essential \
    build-essential \
    curl \
    file \
    libayatana-appindicator3-dev \
    libgtk-3-dev \
    librsvg2-dev \
    libsoup-3.0-dev \
    libssl-dev \
    libssl-dev \
    libwebkit2gtk-4.1-dev \
    libxdo-dev \
    pkg-config \
    wget

USER vscode

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y