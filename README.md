# runwasi-kind-workloads

[runwasi](https://github.com/containerd/runwasi) の containerd-shim-wasmtime を使った kind で動かした

```sh
cargo component new sleep-wasm
```

---

sleep-wasm/ で実行

```sh
cargo component build --target wasm32-wasip2
```

```sh
wasmtime target/wasm32-wasip2/debug/sleep-wasm.wasm
```

```
docker build --platform linux/amd64 -t ghcr.io/z63d/sleep-wasm-ociimage .
```

```sh
docker run --rm --runtime=io.containerd.wasmtime.v1 --platform=linux/amd64 ghcr.io/z63d/sleep-wasm-ociimage
```

```sh
docker push ghcr.io/z63d/sleep-wasm-ociimage
```

```sh
wkg oci push ghcr.io/z63d/sleep-wasm-wasmociartifact target/wasm32-wasip2/debug/sleep-wasm.wasm
```

---

```sh
kubectl apply -f deployment.yaml
```
