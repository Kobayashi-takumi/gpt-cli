# ベースイメージを指定
FROM --platform=linux/x86_64 rust:1.68-alpine as builder

# 環境変数の設定
ENV TARGET_ARCH=x86_64-unknown-linux-musl

# ターゲットアーキテクチャを追加
RUN rustup target add $TARGET_ARCH
RUN rustup component add rustfmt

# 必要なパッケージのインストール
RUN apk update && apk --no-cache add \
    libpq \
    musl-dev \
    pkgconfig \
    openssl-dev \
    gcc \
    postgresql-dev
    
# 作業ディレクトリを設定
WORKDIR /usr/src

# ビルド時に自動的にクリーンアップされるように環境変数を設定
ENV CARGO_TARGET_DIR=target

# クロスコンパイル用の環境変数を設定
COPY . .
RUN ls -a
RUN echo $TARGET_ARCH
RUN cargo clean && cargo build --release --target $TARGET_ARCH --bin app

FROM --platform=linux/x86_64 alpine:latest as runtime
# 必要なパッケージのインストール
RUN apk update && apk --no-cache add ca-certificates
WORKDIR /bin
COPY --from=builder /usr/src/target/x86_64-unknown-linux-musl/release/app .
CMD app