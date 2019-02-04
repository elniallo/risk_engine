FROM rust:1-stretch as builder
# Choose a workdir
WORKDIR /usr/src/app
# Create blank project
# Copy Cargo.toml to get dependencies
COPY Cargo.toml .
# This is a dummy build to get the dependencies cached
COPY src src
# Build app (bin will be in /usr/src/app/target/release/rust-lang-docker-multistage-build)
RUN cargo build --release

FROM ubuntu:xenial
# Copy bin from builder to this new image
RUN apt-get update
RUN apt-get install -y wget
RUN apt-get install -y build-essential
RUN apt-get install -y zlib1g-dev
# Setup OpenSSL
ARG OPENSSL_VERSION=1.1.0g
RUN wget https://www.openssl.org/source/openssl-${OPENSSL_VERSION}.tar.gz
RUN tar xvfz openssl-${OPENSSL_VERSION}.tar.gz
# Fix error `openssl: error while loading shared libraries: libssl.so.1.1: cannot open shared object file: No such file or directory`. Reference: https://github.com/openssl/openssl/issues/3993
RUN cd openssl-${OPENSSL_VERSION} && \
    ./config \
    --debug \
    --prefix=/usr/local \
    --libdir=/lib \
    --openssldir=/usr/local/ssl && \
    make && make install
# Add /usr/local/openssl/lib to /etc/ld.so.conf and then run the command `ldconfig`
RUN echo '/usr/local/ssl/lib' >> /etc/ld.so.conf
RUN cat /etc/ld.so.conf
RUN ldconfig
RUN echo 'export LD_LIBRARY_PATH=/usr/local/ssl/lib' >> ~/.bash_profile && . ~/.bash_profile
RUN openssl version
COPY assets /assets/
COPY --from=builder /usr/src/app/target/release/risk-engine /bin/
# Default command, run app
CMD ["risk-engine"]

EXPOSE 7878
EXPOSE 9092