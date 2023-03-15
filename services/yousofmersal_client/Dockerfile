# stage 1
FROM rust as chef

RUN rustup toolchain install nightly
RUN rustup default nightly

USER root
RUN cargo +nightly install -Z sparse-registry --debug cargo-chef
WORKDIR /sources

# stage 2
FROM chef AS planner
COPY . .
RUN cargo +nightly chef prepare --recipe-path recipe.json

# stage 3
FROM chef AS builder 
COPY --from=planner /sources/recipe.json recipe.json
RUN rustup target add wasm32-unknown-unknown
RUN cargo +nightly chef cook --release -Z sparse-registry --recipe-path recipe.json
RUN wget -qO- https://github.com/YousofMersal/cli/releases/download/0.2/dioxus-cli-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
RUN ./dioxus-cli build --release
COPY . .

# stage 4
FROM nginx as runtime

COPY nginx.conf /etc/nginx/conf.d/default.conf

WORKDIR /usr/share/nginx/html

COPY --from=builder /sources/dist/ ./
