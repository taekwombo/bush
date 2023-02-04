### Requirements
- Firefox Nightly `111.0a1 (2023-02-03) (64-bit)`
- `about:config` webgpu flag set to true

### Start

```sh
# Install @webgpu/types dependency.
npm i

# Compile TypeScript files.
tsc

# Start http server (start it in the repo root directory for 07-cube to work).
python3 -m http.server
```

### Specs
- https://www.w3.org/TR/webgpu/
- https://www.w3.org/TR/WGSL/
