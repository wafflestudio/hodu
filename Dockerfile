#
# build waffle-judge
#
FROM rust:1.80.0-slim as waffle-judge-builder

WORKDIR /usr/src/waffle-judge
COPY . .
RUN cargo build --release

#
# build isolate
#
FROM debian:12.6-slim as isolate-builder

# install isolate
RUN apt-get update && apt-get --no-install-recommends install -y \
  git build-essential pkg-config libsystemd-dev libcap-dev
RUN git config --global http.sslVerify false
RUN git clone --depth 1 --branch v2.0 https://github.com/ioi/isolate.git /usr/src/isolate
WORKDIR /usr/src/isolate
RUN make isolate

#
# runner
#
FROM debian:12.6-slim as runner

# install language specific tools
RUN apt-get update && apt-get --no-install-recommends install -y \
  gcc libc6-dev \
  openjdk-17-jdk \
  && rm -rf /var/lib/apt/lists/*

# copy bin
COPY --from=waffle-judge-builder /usr/src/waffle-judge/target/release/waffle-judge /usr/local/bin/waffle-judge
COPY --from=isolate-builder /usr/src/isolate/isolate /usr/local/bin/isolate

# run
CMD ["/usr/local/bin/waffle-judge"]
