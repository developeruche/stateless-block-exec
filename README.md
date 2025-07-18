# Ethereum Stateless Block Execution

This repository contains implementations for comparing Ethereum stateless block execution across different clients:
- Geth (Go implementation)
- Reth (Rust implementation)

## Geth Implementation

### Stateless Block Execution

Run the stateless block execution with:

```bash
cd geth/stateless-exec
```

#### Using geth_evm

Alternatively, you can use the geth_evm tool:

```bash
./geth_evm t8n \
  --input.alloc=./geth/assets/alloc.json \
  --input.txs=./geth/assets/tx.json \
  --input.env=./geth/assets/env.json \
  --state.fork=Prague
```

### Tracing Geth Execution

```bash
strace -o geth_trace.log ./geth_evm t8n \
  --input.alloc=./geth/assets/alloc.json \
  --input.txs=./geth/assets/tx.json \
  --input.env=./geth/assets/env.json \
  --state.fork=Prague
```

## Reth Implementation

### Building Reth for RISC-V

```bash
cargo build --release --target riscv64gc-unknown-linux-gnu
```

### Tracing Reth Execution

```bash
strace -o reth_trace.log qemu-system-riscv64 -L /usr/riscv64-linux-gnu /usr/bin/strace ./exec-block
```

## RISC-V Tracing Notes

To properly trace the RISC-V binary execution, you need to run it in a RISC-V VM:

1. Set up a RISC-V VM using Ubuntu following the guide at [Ubuntu RISC-V Boards Documentation](https://canonical-ubuntu-boards.readthedocs-hosted.com/en/latest/how-to/qemu-riscv/)
2. Transfer the binary to the VM
3. Run the tracing command within the VM environment

## Project Structure

- `/geth` - Go Ethereum implementation files
  - `/stateless-exec` - Stateless execution implementation
  - `/assets` - JSON input files for block execution
- `/reth` - Rust Ethereum implementation files

## License

MIT