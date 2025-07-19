#!/bin/bash

# Create directories
mkdir -p stateless-exec/assets

# Download binary
curl -L https://raw.githubusercontent.com/developeruche/stateless-block-exec/main/geth/geth_evm_riscv64_linux -o stateless-exec/geth_evm_riscv64_linux
chmod +x stateless-exec/geth_evm_riscv64_linux

# Download asset files
curl -L https://raw.githubusercontent.com/developeruche/stateless-block-exec/main/geth/assets/alloc.json -o stateless-exec/assets/alloc.json
curl -L https://raw.githubusercontent.com/developeruche/stateless-block-exec/main/geth/assets/env.json -o stateless-exec/assets/env.json
curl -L https://raw.githubusercontent.com/developeruche/stateless-block-exec/main/geth/assets/exp.json -o stateless-exec/assets/exp.json
curl -L https://raw.githubusercontent.com/developeruche/stateless-block-exec/main/geth/assets/tx.json -o stateless-exec/assets/tx.json

echo "Download complete. Files saved to stateless-exec directory."

