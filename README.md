
### Build
```bash
cargo build --release --target riscv64gc-unknown-linux-gnu
```

### Tracing
```bash
strace -o strace.log qemu-system-riscv64 -L /usr/riscv64-linux-gnu /usr/bin/strace ./exec-block
```

this trace iswrong to get the correct trace you would need to run the ubuntu distro on a riscv64 vm and then run the strace command along side the binanry. 

this docs would work you through how to dom this;
https://canonical-ubuntu-boards.readthedocs-hosted.com/en/latest/how-to/qemu-riscv/