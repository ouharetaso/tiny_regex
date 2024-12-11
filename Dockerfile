FROM rust:1.82

ENV TZ=Asia/Tokyo
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && \
    echo $TZ > /etc/timezone
RUN apt update && apt upgrade -y
RUN apt install -y locales
RUN locale-gen ja_JP.UTF-8
ENV LANG=ja_JP.UTF-8
ENV LANGUAGE=ja_JP.UTF-8
ENV LC_ALL=ja_JP.UTF-8


WORKDIR /artifact
# 必要なAPTパッケージを適当にインストール
# RUN apt install -y ${apt-package} 

# Gitリポジトリを展開しても良い
#RUN git clone https://github.com/oss-experiment-uec/2024-s2210298-tiny_regex .

# Dockerfileを実行する場所からファイルをコピーする場合
COPY . /artifact

RUN cargo build --bin tiny_grep
RUN cargo build --bin re_place
RUN cargo build --features on_the_fly --bin tiny_grep
RUN cargo build --features on_the_fly --bin re_place

