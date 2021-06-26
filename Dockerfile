FROM rust:1.53.0

# デフォルトのディクトリを指定
WORKDIR /app

RUN apt-get update -qq && \
    rm -rf /var/lib/apt/lists/* && \
    cargo install cargo-watch && \
    cargo install diesel_cli --no-default-features --features postgres

# ローカルのファイルの全てをコンテナのappディレクトリの中にコピーする
COPY . .