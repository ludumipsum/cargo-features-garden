## Context-Specific Mutually Exclusive Features

Let's say `lib-a`s implementations have some target-specific implications.
It'd be easy to make your binary enable features based on the target platform, right?

#### Setup

```
┌─────┐  Exposes three features,
│lib-a│  `f-a`, `f-b`, and `f-c`.
└─────┘  Only one may be active.
▲  ▲  ▲
└─┐│┌─┘  These are `target.'cfg(..)'`-gated, and
  │││    each enables one of `f-a`, `f-b`, or `f-c`.
┌─────┐
│exe-a│
└─────┘
```

From `2_exe-a/Cargo.toml`,
```
[target.'cfg(unix)'.dependencies]
lib-a = { path = "../1_lib-a", features = ["f-a"] }

[target.'cfg(windows)'.dependencies]
lib-a = { path = "../1_lib-a", features = ["f-b"] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
lib-a = { path = "../1_lib-a", features = ["f-c"] }
```

#### Explanation

No matter where you build this, no matter what your `--target` is, `cargo build -p exe-a` will fail with,

> `error: f-a, f-b, and f-c are mutually exclusive features`

This is the same issue described in `context-specific-features-arent/` and `target-specific-diamond`; features are calculated before target-specific dependencies are pruned, so no matter what you're actually targeting, `lib-a` will _always_ have `features = ["f-a", "f-b", "f-c"]`.
