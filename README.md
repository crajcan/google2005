### Build wasm binary for Fastly Compute

```
cd wasm32-wasi_executable/
fastly compute build
```


### Run wasm binary locally

```
cd wasm32-wasi_executable/
fastly compute serve
```

### Deploy wasm binary to Fastly Compute

```
cd wasm32-wasi_executable/
fastly compute deploy
```

### build and run locally

```
cd x86_64-apple-darwin_executable
cargo run
```