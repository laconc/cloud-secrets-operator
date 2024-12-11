FROM public.ecr.aws/docker/library/rust:1.83-alpine AS base

WORKDIR /usr/src/app

ENV USER=appuser

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    "$USER"

COPY ./app/Cargo.toml ./app/Cargo.lock ./

RUN apk update && apk add build-base

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -r src

COPY ./app/src ./src

RUN cargo build --release

# ----------------
FROM gcr.io/distroless/base AS app

WORKDIR /usr/src/app

COPY --from=base /etc/passwd /etc/passwd
COPY --from=base /etc/group /etc/group
COPY --from=base /usr/src/app/target/release/operator .

USER appuser:appuser

ENV RUST_BACKTRACE=1

CMD [ "./operator" ]
