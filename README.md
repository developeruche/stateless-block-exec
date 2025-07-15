
### Build
```bash
cargo build --release --target riscv64gc-unknown-linux-gnu
```

### Tracing
```bash
strace -o strace.log qemu-system-riscv64 -L /usr/riscv64-linux-gnu /usr/bin/strace ./exec-block
```