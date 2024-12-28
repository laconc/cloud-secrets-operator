FROM public.ecr.aws/docker/library/rust:1.83-alpine AS base

WORKDIR /usr/src/app

RUN adduser \
    --system \
    --disabled-password \
    --no-create-home \
    --uid 2000 \
    appuser

RUN apk update && apk add build-base

COPY ./app/Cargo.toml ./app/Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -r src

COPY ./app/src ./src

RUN cargo build --release --bin operator

# ----------------
FROM gcr.io/distroless/base AS app

ARG BUILD_DATE
ARG GIT_REF
ARG VERSION

# OCI labels for provenance
LABEL org.opencontainers.image.title="cloud-secrets-operator" \
      org.opencontainers.image.description="A Kubernetes operator for managing Kubernetes Secrets and the associated AWS Secrets Manager secrets." \
      org.opencontainers.image.authors="Dashiel Lopez Mendez <hi@64f.dev>" \
      org.opencontainers.image.url="https://github.com/laconc/cloud-secrets-operator" \
      org.opencontainers.image.source="https://github.com/laconc/cloud-secrets-operator" \
      org.opencontainers.image.documentation="https://github.com/laconc/cloud-secrets-operator/blob/${GIT_REF}/README.md" \
      org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.created="${BUILD_DATE}" \
      org.opencontainers.image.revision="${GIT_REF}" \
      org.opencontainers.image.licenses="Apache-2.0"

WORKDIR /usr/src/app

COPY --from=base /etc/passwd /etc/passwd
COPY --from=base /etc/group /etc/group
COPY --from=base /usr/src/app/target/release/operator .

USER appuser

ENV RUST_BACKTRACE=1

CMD [ "./operator" ]
