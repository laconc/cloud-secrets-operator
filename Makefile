SHELL := /bin/bash

IMAGE_NAME ?= cloud-secrets-operator
IMAGE_TAG ?= latest
VERSION := $(shell cd app; cargo metadata --no-deps --format-version=1 | jq -r '.packages[0].version')
BUILD_DATE := $(shell date -u +'%Y-%m-%dT%H:%M:%SZ')
GIT_REF := $(shell git rev-parse HEAD)

.PHONY: crdgen build-image lint

crdgen:
	cd app && \
	RUST_BACKTRACE=1 cargo run --bin crdgen -- CloudSecret > ../deploy/crds/cloud_secret.yaml && \
	RUST_BACKTRACE=1 cargo run --bin crdgen -- CloudSecretProvider > ../deploy/crds/cloud_secret_provider.yaml

build-image:
	docker build \
		--build-arg BUILD_DATE=$(BUILD_DATE) \
		--build-arg GIT_REF=$(GIT_REF) \
		--build-arg VERSION=$(VERSION)-dev \
		-t $(IMAGE_NAME):$(IMAGE_TAG) \
		.

lint:
	cd app; \
	cargo clippy --all-targets --all-features -- -D warnings; \
	cargo fmt --all -- --check
