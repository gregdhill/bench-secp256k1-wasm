# Secp256k1 Wasm Benchmarks

```
$ cargo bench

test bitcoin_core_libsecp256k1::bench_libsecp256k1 ... bench:   5,346,107 ns/iter (+/- 68,680)
test parity_libsecp256k1::bench_libsecp256k1       ... bench:     108,611 ns/iter (+/- 925)
test rust_crypto_libsecp256k1::bench_libsecp256k1  ... bench:      60,506 ns/iter (+/- 770)
```