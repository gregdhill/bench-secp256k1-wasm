# Secp256k1 Wasm Benchmarks

```
$ cargo bench

test bitcoin_core_libsecp256k1::bench_libsecp256k1 ... bench:      35,490 ns/iter (+/- 1,263)
test parity_libsecp256k1::bench_libsecp256k1       ... bench:     105,328 ns/iter (+/- 14,306)
test rust_crypto_libsecp256k1::bench_libsecp256k1  ... bench:      58,396 ns/iter (+/- 4,284)
```