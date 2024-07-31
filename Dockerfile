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
  git build-essential pkg-config libcap-dev
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
  # language: c
  gcc libc6-dev \
  # language: java
  openjdk-17-jdk \
  # clean
  && rm -rf /var/lib/apt/lists/* \
  && apt-get clean

# copy bin
COPY --from=waffle-judge-builder /usr/src/waffle-judge/target/release/waffle-judge /usr/local/bin/waffle-judge
COPY --from=isolate-builder /usr/src/isolate/isolate /usr/local/bin/isolate
COPY --from=isolate-builder /usr/src/isolate/isolate-check-environment /usr/local/bin/isolate-check-environment
COPY --from=isolate-builder /usr/src/isolate/default.cf /usr/local/etc/isolate

RUN echo '#!/bin/sh' > /usr/local/bin/start.sh \
  && echo 'isolate --init' >> /usr/local/bin/start.sh \
  && echo '/usr/local/bin/waffle-judge' >> /usr/local/bin/start.sh \
  && chmod +x /usr/local/bin/start.sh

ENTRYPOINT ["/usr/local/bin/start.sh"]
