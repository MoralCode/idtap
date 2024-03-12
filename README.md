# idtap
A framework for using the serial number of nearly any NFC card in projects




## Testing

```
cargo build
socat -d -d pty,raw,echo=0 pty,raw,echo=0
cargo run --bin serialreader
cargo run --bin serialemitter
```

