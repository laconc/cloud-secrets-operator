# cloud-secrets-operator

**_Note: This project is under active development. Interfaces, features, and behavior may change frequently. Not all functionality is currently implemented._**

## ⚙️ Overview

The operator provides a declarative interface for managing secrets from external secret sources (such as AWS Secrets Manager) and syncing them into your cluster as Kubernetes Secrets, with built-in support for secret generation, rotation, and validation.

It introduces two custom resource definitions (CRDs): `CloudSecret` and `CloudSecretProvider`.

---

## ✨ Features

- ✅ Declarative secret syncing from external providers to Kubernetes Secrets
- 🔄 Automatic periodic syncing and optional key rotation
- ✅ Fine-grained control over individual secret keys
- 🔒 Support for custom generation, rotation, and validation logic via containers

---

## 📦 Getting Started

---

## 🧩 Custom Resources

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
  secretName: my-k8s-secret
  source:
    name: my-aws-secret
  providerRef:
    name: aws-provider
  strict: true
  refreshInterval: 5m
  keys:
    - name: APP_PASSWORD
      rotateInterval: 90d
      actions:
        create:
          pattern: "^[a-zA-Z0-9]{16,}$"
        validate:
          pattern: "^[a-zA-Z0-9]{16,}$"
```

---

## Development

### Build CRDs

After making changes to the CRD specs at [app/src/crds.rs](app/src/crds.rs), you can regenerate the CRD manifests with:

```shell
make crdgen
```
