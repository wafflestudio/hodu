#
# build hodu
#
FROM rust:1.80.0-slim AS hodu-builder

WORKDIR /usr/src/hodu
COPY . .
RUN cargo build --release

#
# build isolate
#
FROM debian:10.13-slim AS isolate-builder

# install isolate
RUN apt-get update && apt-get --no-install-recommends install -y \
  git build-essential pkg-config libcap-dev
RUN git config --global http.sslVerify false
RUN git clone --depth 1 --branch v1.10.1 https://github.com/ioi/isolate.git /usr/src/isolate
WORKDIR /usr/src/isolate
RUN make isolate

#
# runner
#
FROM debian:10.13-slim AS runner

# install language specific tools
RUN apt-get update && apt-get install -y \
  # language: c
  gcc libc6-dev \
  # language: c++
  g++ \
  # language: java
  openjdk-11-jdk \
  # language: python
  python3 \
  # language: javascript
  nodejs \
  # clean
  && rm -rf /var/lib/apt/lists/* \
  && apt-get clean

# copy bin
COPY --from=hodu-builder /usr/src/hodu/target/release/hodu-server /usr/local/bin/hodu-server
COPY --from=isolate-builder /usr/src/isolate/isolate /usr/local/bin/isolate
COPY --from=isolate-builder /usr/src/isolate/isolate-check-environment /usr/local/bin/isolate-check-environment
COPY --from=isolate-builder /usr/src/isolate/default.cf /usr/local/etc/isolate

ENTRYPOINT ["/usr/local/bin/hodu-server"]
