FROM rust:1.82

ENV TZ=Asia/Tokyo
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && \
    echo $TZ > /etc/timezone
RUN apt update && apt upgrade -y

WORKDIR /artifact
# 必要なAPTパッケージを適当にインストール
# RUN apt install -y ${apt-package} 

# Gitリポジトリを展開しても良い
RUN git clone https://github.com/oss-experiment-uec/2024-s2210298-tiny_regex .

RUN cargo build
# Dockerfileを実行する場所からファイルをコピーする場合
# COPY . /artifact