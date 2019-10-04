## Context-Specific Features Aren't Context-Specific

Nothing in this workspace is intended to compile. Rather, the compilation errors emitted are intended to demonstrate one of the foot-guns of the Features feature.

#### Setup

`lib-a` has a number of `compile_error!` clauses [conditionally compiled][lib-a_src] based on `#[cfg(feature = "...")]` expressions.

For example, this will only compile if the `"throw_compile_error"` feature is set;

```rust
#[cfg(feature = "throw_compile_error")]
compile_error!("This feature should never be set");
```

`exe-a` ["conditionally" enables][exe-a_src] those features with `[target.<expression>.dependency]` sections, the `[dev-dependencies]` section, and the `[build-dependencies]` section.

For example, this suggests we will include `lib-a` with the `"throw_compile_error"` feature set if `cfg(false)` resolves to `true` (so, in otherwords, never);

```toml
[target.'cfg(false)'.dependencies]
lib-a = { path = "../1_lib-a", features = ["throw_compile_error"] }
```

[lib-a_src]: 1_lib-a/src/lib.rs#L1-L17
[exe-a_src]: 2_exe-a/Cargo.toml#L10-L26

###### Take a look at

```sh
cargo build -p exe-a
```

#### Explanation

This is the simplest form of the union'd Features feature gotcha. Regardless of profile, target platform, or truthiness of `target.'cfg(...)'` expressions, every `features = [...]` clause in `exe-a/Cargo.toml` will be active prior to the compilation of `lib-a`.

This is because Features exist on dependency _nodes_ in Cargo's resolution graph, rather than dependency _edges_ -- where the specifications on the current profile or target exist. So the below (in `exe-a/Cargo.toml`) describes two things,

```toml
[target.wasm32-unknown-unknown.dependencies]
lib-a = { path = "../1_lib-a", features = ["target_wasm"] }
```

1. A dependency _edge_ between `exe-a` and `lib-a` that is conditional on the target platform matching `wasm32-unknown-unknown`. If the target does not match, this edge will pruned _at compile time_.
2. The _state_ of the `lib-a` dependency _node_. In this case, `features = ["target_wasm"]` is monotonically appended to the `lib-a` node Features list _before_ the compilation phase begins.
