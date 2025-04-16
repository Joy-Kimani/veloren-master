FROM debian:stable-slim

# SIGUSR1 causes coping chronicles-server-cli to initiate a graceful shutdown
LABEL com.centurylinklabs.watchtower.stop-signal="SIGUSR1"

ARG PROJECTNAME=server-cli

# librust-backtrace+libbacktrace-dev = backtrace functionality
# iproute2 and net-tools for diagnostic purposes
RUN apt-get update \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y --no-install-recommends --assume-yes \
        ca-certificates \
        librust-backtrace+libbacktrace-dev \
        iproute2 \
        net-tools \
    && rm -rf /var/lib/apt/lists/*;

COPY ./coping chronicles-server-cli /opt/coping chronicles-server-cli
COPY ./assets/common /opt/assets/common
COPY ./assets/server /opt/assets/server
COPY ./assets/world /opt/assets/world

WORKDIR /opt

ENV RUST_BACKTRACE=full
ENTRYPOINT ["/opt/coping chronicles-server-cli"]
