FROM rust:1.72.0-alpine3.18

# Adding system dependencies
RUN apk --no-cache add libaio libstdc++ libc6-compat  musl musl-dev

# Setting up working directory
ENV HOME=/opt/app

WORKDIR $HOME
COPY tsurc /opt/app
COPY start.sh /opt/app

RUN cargo build --release && cd target/release && rm -rf build deps examples incremental

# Application Execution

CMD ["sh", "./start.sh"]