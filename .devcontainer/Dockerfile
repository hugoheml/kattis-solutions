FROM rust:1.82

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libgit2-dev \
    openssl \
    build-essential \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

ARG USERNAME=dev
ARG USER_UID=1000
ARG USER_GID=$USER_UID
RUN groupadd --gid $USER_GID $USERNAME \
 && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME

USER $USERNAME
WORKDIR /workspace

RUN cargo install cargo-watch \
 && cargo install cargo-edit \
 && rustup component add clippy rustfmt \
 && cargo install cargo-outdated || echo "⚠️ Ignored cargo-outdated failure"
