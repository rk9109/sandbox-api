FROM ubuntu:18.04

RUN apt-get update
RUN apt-get install -y gcc curl && \
    apt-get clean &&               \
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

# install rust toolchain
#
RUN curl https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/sandbox/.cargo/bin:${PATH}"
