# Builder phase

FROM rust:latest AS builder
RUN USER= root cargo new --bin person_api
WORKDIR ./person_api
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/person_api*
RUN cargo build --release

# Release phase

FROM debian:buster-slim

ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 3000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from =builder /person_api/target/release/person_api ${APP}/person_api

RUN chown -R $APP_USER:$APP_USER ${APP}

USER ${APP_USER}
WORKDIR ${APP}

CMD ["./person_api"]