# cloud-secrets-operator

**_Note: This project is under active development. Features and behavior may change frequently. Not all functionality has been implemented._**

## Overview

The operator provides a declarative interface for managing secrets from external secret sources (such as AWS Secrets Manager) and syncing them into your cluster as Kubernetes Secrets, with built-in support for secret generation, rotation, and validation.

It introduces two custom resource definitions (CRDs): `CloudSecret` and `CloudSecretProvider`.

## Features

- Declarative secret syncing from external providers to Kubernetes Secrets
- Automatic periodic syncing and optional key rotation
- Fine-grained control over individual secret keys
- Support for custom generation, rotation, and validation logic via containers

## Getting Started

Will be fleshed out in a future release.

## Custom Resources

### `CloudSecretProvider`

Defines a connection to an external secrets provider.

Example:
```yaml
apiVersion: 64f.dev/v1alpha1
kind: CloudSecretProvider
metadata:
  name: aws-provider
spec:
  provider:
    awsSecretsManager:
      region: us-west-2
      auth:
        secretName: aws-credentials
```

### `CloudSecret`

Defines a Kubernetes Secret sourced from an external provider.

**Spec Highlights:**
- Sync secrets by referencing a provider (`CloudSecretProvider`) and a secret name
- Provides for global and per-key configuration for secret generation, rotation, and validation
  - Configuration can consist of secret length, regex patterns, or calling containers for custom logic

```yaml
apiVersion: 64f.dev/v1alpha1
kind: CloudSecret
metadata:
  name: my-app-secret
spec:
  targets:
    - name: my-k8s-secret
      namespace: current-namespace
  source:
    name: my-aws-secret
  providerRef:
    name: aws-provider
  strict: true # If true, only the specified keys must exist in the external source
  refreshInterval: 5m
  actions:
    create:
      minimum: 32
      maximum: 96
    rotate:
      minimum: 32
      maximum: 96
  keys:
    - name: APP_PASSWORD # Uses the global `actions` config
      rotateInterval: 7d

    - name: API_TOKEN # We define per-key config, which overrides the global config
      rotateInterval: 90d
      actions:
        create:
          pattern: "^[A-F0-9]{8}-[A-F0-9]{4}-[A-F0-9]{4}-[A-F0-9]{4}-[A-F0-9]{12}$"
        rotate:
          pattern: "^[A-F0-9]{8}-[A-F0-9]{4}-[A-F0-9]{4}-[A-F0-9]{4}-[A-F0-9]{12}$"

    - name: ENCRYPTION_KEY # Assumes the key was explicitly added in the external source. Won't be automatically rotated.
      actions:
        validate:
          container:
            image: my-validation-image:latest
            command: ["./validate-secret"]
```

## Development

### Regenerate CRDs

After making changes to the CRD specs at [app/src/crds.rs](app/src/crds.rs), you can regenerate the CRD manifests with:

```shell
make crdgen
```
