SHELL := /bin/bash

IMAGE_NAME ?= cloud-secrets-operator
IMAGE_TAG ?= latest
BUILD_DATE := $(shell date -u +'%Y-%m-%dT%H:%M:%SZ')
GIT_REF := $(shell git rev-parse HEAD)
VERSION := $(shell cd app; cargo metadata --no-deps --format-version=1 | jq -r '.packages[0].version')

.PHONY: build-image crdgen verify-crds lint

build-image:
	docker build \
		--build-arg BUILD_DATE=$(BUILD_DATE) \
		--build-arg GIT_REF=$(GIT_REF) \
		--build-arg VERSION=$(VERSION) \
		-t $(IMAGE_NAME):$(IMAGE_TAG) \
		.

crdgen:
	cd app && \
	RUST_BACKTRACE=1 cargo run --bin crdgen -- CloudSecret > ../deploy/crds/cloud_secret.yaml && \
	RUST_BACKTRACE=1 cargo run --bin crdgen -- CloudSecretProvider > ../deploy/crds/cloud_secret_provider.yaml

verify-crds:
	@cd app && RUST_BACKTRACE=1 cargo run --bin crdgen -- CloudSecret \
		| diff -u ../deploy/crds/cloud_secret.yaml - \
		|| (echo "ERROR: The CloudSecret CRD is out of date, please regenerate the CRDs locally with 'make crdgen'."; exit 1)

	@cd app && RUST_BACKTRACE=1 cargo run --bin crdgen -- CloudSecretProvider \
		| diff -u ../deploy/crds/cloud_secret_provider.yaml - \
		|| (echo "ERROR: The CloudSecretProvider CRD is out of date, please regenerate the CRDs locally with 'make crdgen'."; exit 1)

lint:
	cd app; \
	cargo clippy --all-targets --all-features -- -D warnings; \
	cargo fmt --all -- --check
