## Target Specific Diamond Dependency

This is a continuation of, and possible solution to, the [naive setup](../1_naive-setup/) describe in part one.

### Setup

Again, be sure you have the `wasm32-unknown-unknown` target installed with,

```sh
rustup target add wasm32-unknown-unknown
```

The setup this time rejects the use of target-specific dependencies entirely, and replaces them with re-exported Features that enable optional dependencies. To simplify the demonstration, `lib-d` will no longer be considered a valid build target; it's a library intended to be used by a final target, so that's how it will be used. As such, `exe-a` has been added to stand in for a native binary target, and `wlib-a` has been added to stand in for a WASM cdylib target.

The dependency graph as been modified to the below;

```
      ┌─────────┐
      │memmap-rs│
      └─────────┘
           ▲
           ¦ optional
        ┌─────┐
   ┌───▶│lib-a│◀─ ─  optional
   │    └─────┘    │ features = ["memmap"]
┌─────┐         ┌─────┐
│lib-b│         │lib-c│
└─────┘         └─────┘
   ▲    ┌─────┐    ▲ optional
   └────│lib-d│─ ─ ┘ features = ["native"]
        └─────┘
           ▲
features = │ features =
  ["wasm"] │   ["native"]
   ┌───────┴───────┐
┌──────┐        ┌─────┐
│wlib-a│        │exe-a│
└──────┘        └─────┘
```

###### Try running

```sh
cargo build -p exe-a
```

Note that memmap is built, and the build succeeds.

```sh
cargo build -p wlib-a --target wasm32-unknown-unknown
```

Note that memmap is _not_ built, and the build succeeds.

If you were to try to build `exe-a` for WASM,

```sh
cargo build -p exe-a --target wasm32-unknown-unknown
```

... you would see the same error modeled in part one. But this time, it's expected and correct.
