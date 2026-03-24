# Rust xtask Template

This is a template wrapper that leverages Cargo's `xtask` pattern to manage powerful customized build commands.  
This template heavelly relies on the `nightly` toolchain.

## CLI Commands

You can run these custom workflows using `cargo xtask <subcommand>` (assuming you've setup a `.cargo/config.toml` alias) or `cargo run --manifest-path xtask/Cargo.toml -- <subcommand>`.

The template supports three heavily optimized profiles:

### 1. `fast-dev`

**Command:** `cargo xtask fast-dev` and `cargo xtask clippy`

**Purpose**: Maximize compilation speed during repetitive local development cycles.

- It dynamically assigns the fastest available linking backends: `rust-lld` on Windows and `wild` on Linux.
- Compiles via the cranelift codegen backend (`-Zcodegen-backend=cranelift`), enabling multithreading (`-Zthreads=32`) to achieve extremely quick iteration speeds.
- Does not care about final binary size or performance metrics; it purely cares about compile time.

**Optimization Tips:**

- **OS Constraints**: Ensure you have installed the respective linkers on your system (such as `wild` for Linux) or else the build invocation might fail. Windows will automatically use `rust-lld` which is included with nightly Cargo.
- **Advanced Windows Tips**: For even faster link times on Windows, consider using `radlinker`. Additionally, moving your project to a **Windows Dev Drive** (which builds on the ReFS file system) will significantly improve your overall disk I/O and parallel compilation times. You can also create an Antivirus exclusion for your project folder, which will significantly reduce the overhead for spawning processes.
- You may consider removing the cranelift backend flag if some specific crates you rely on do not support it properly yet (looking at you `aws-lc-rs`).
- Use `sccache` for caching.
- If you don't care about debug symbols, you can also add `debug = 0` and `strip = "debuginfo"` to your Cargo profile configuration to further speed up compilation.
- If you heavily use procedural macros in your project (e.g., if you use serde), it might be worth it to play around with opt-levels in your Cargo.toml [[1]](https://github.com/mre/endler.dev/issues/53#issuecomment-1366829563):

```toml
[profile.dev.build-override]
opt-level = 3
```

### 2. `min-size`

**Command:** `cargo xtask min-size [--target <target>] [--upx]`

**Purpose**: Achieve the smallest possible final binary size.

- Drops out the standard library panics and debugging symbols (`-Zbuild-std=std,panic_abort`, `trim-paths=true`).
- Employs size-specific compiler features (`optimize_for_size`) along with link-time optimization (`-Clto=true`) and aggressive packing (`pack-relative-relocs`).
- Optionally accepts a `--upx` flag to do an ultra-brute compression pass via the UPX compressor.

**Optimization Tips:**

- **Best Use Case**: Perfect for distributing binaries over the network, lightweight standalone tools, or Tauri wrappers where payload weight translates to consumer friction.

**Other Targets**

- I haven't yet tested these configurations for wasm.

### 3. `speed`

**Command:** `cargo xtask speed [--target <target>] [--native]`

**Purpose**: Compile for absolute maximum execution speed at runtime, ignoring binary size and compilation time.

- Same hermetic `-Zbuild-std` rebuild strategy as `min-size`, skipping pre-compiled std artifacts for potentially deeper cross-crate optimization.
- Bumps compilation bounds directly to `-Copt-level=3` and applies heavy `--lto=fat`.
- `--native`: Append this flag (`cargo xtask speed --native`) to instruct rustc to deploy cpu-specific instructions unique to your host CPU architecture (`-Ctarget-cpu=native`).

**Optimization Tips:**

- **Caveat for `--native`**: Only use `--native` if you plan to run the software on the EXACT SAME machine that compiled it! Distribution to other machines will frequently result in immediate SIGILL crashes if their processors lack the newer instruction features (e.g. AVX-512).
- Use alternative allocators like `mimalloc` for better performance.
- Use alternative hashers like `foldhash` for better performance.

## Configuration

You can easily adapt this template if you're using alternative frameworks, such as Tauri. In `xtask/src/main.rs`, modify the global constants to point to the desired workflow:

```rust
const RUN_CMD: &[&str] = &["cargo", "tauri", "dev"];
const BUILD_CMD: &[&str] = &["cargo", "tauri", "build"];
// Update `BINARY_NAME` accordingly!
// you can also specify the cargo profile if you want to use custom ones (e.g. `--profile=release-lto` or `--profile=min-size`).
```

## References

- https://nnethercote.github.io/perf-book
- https://github.com/johnthagen/min-sized-rust
- https://corrode.dev/blog/tips-for-faster-rust-compile-times/
