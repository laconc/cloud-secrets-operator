crdgen:
	cd app && \
	RUST_BACKTRACE=1 cargo run --bin crdgen -- CloudSecret > ../deploy/crds/cloud_secret.yaml && \
	RUST_BACKTRACE=1 cargo run --bin crdgen -- CloudSecretProvider > ../deploy/crds/cloud_secret_provider.yaml

docker-build:
	docker build -t cloud-secrets-operator:latest .

lint:
	cd app; \
	cargo clippy --all-targets --all-features -- -D warnings; \
	cargo fmt --all -- --check
