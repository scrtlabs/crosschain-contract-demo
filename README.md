# Crosschain contract demo

This repo is a demo of writing a contract that is intended to be deployed on multiple chains with multiple flavours of CosmWasm in a **single codebase**.

This demo supports two flavours of CosmWasm: `vanilla` (i.e. original, like Terra or Juno supported) and `secret` (i.e. SecretNetwork supported).

## Building

```bash
# Building for Secret
make secret

# Building for Juno
make vanilla
```
