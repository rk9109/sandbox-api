FROM ubuntu:18.04

RUN apt-get update
RUN apt-get install -y curl && \
    apt-get clean &&           \ 
    rm -rf /var/lib/apt/lists/*

# run container as unprivileged user
#
RUN groupadd --gid 5000 sandbox &&     \
    useradd  --uid 5000                \
             --gid 5000                \
             --create-home             \
             --home-dir /home/sandbox/ \
             sandbox
USER sandbox
WORKDIR /home/sandbox/

# install go toolchain
#
RUN curl -O https://dl.google.com/go/go1.14.linux-amd64.tar.gz && \
    tar -xzf go1.14.linux-amd64.tar.gz
ENV PATH="/home/sandbox/go/bin:${PATH}"
