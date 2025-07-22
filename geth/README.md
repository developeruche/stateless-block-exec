# Geth Stateless Block Execution

This directory contains the implementation for Ethereum stateless block execution using the Go Ethereum client.

## Quick Start

The simplest way to run stateless block execution:

```bash
cd stateless-exec
```

## Using geth_evm Tool

You can also use the `geth_evm` tool to perform transaction execution without state:

```bash
git clone https://github.com/ethereum/go-ethereum.git
cd go-ethereum/cmd/evm
GOOS=linux GOARCH=riscv64 go build -o evm
```

```bash
./evm t8n \
  --input.alloc=./assets/alloc.json \
  --input.txs=./assets/tx.json \
  --input.env=./assets/env.json \
  --state.fork=Prague
```

_you would have to modify the path to the assets directory, it is recommended to use the path from root not relative path_


## Input Files

The stateless execution requires several JSON input files:

- `assets/alloc.json` - Initial state allocation (accounts with code, storage, and balance)
- `assets/tx.json` - Transaction data to be executed
- `assets/env.json` - Block environment data (timestamp, coinbase, etc.)
- `assets/exp.json` - Expected output after execution (for verification)

## Directory Structure

- `/stateless-exec` - Go implementation of stateless execution
  - `geth_evm_arm` - EVM implementation (binary) for ARM architecture
  - `geth_evm_riscv64` - EVM implementation (binary) for RISC-V architecture
  - `assets/` - JSON input files needed for execution

## Tracing

To trace the execution and analyze system calls:

```bash
strace -o geth_trace.log ./geth_evm t8n \
  --input.alloc=./assets/alloc.json \
  --input.txs=./assets/tx.json \
  --input.env=./assets/env.json \
  --state.fork=Prague
```

## Development

When modifying the stateless execution implementation, ensure that you validate against the expected output in `exp.json`.