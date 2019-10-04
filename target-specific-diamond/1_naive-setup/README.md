## Target Specific Diamond Dependency

### Setup

First off, be sure you have the `wasm32-unknown-unknown` target installed with,

```sh
rustup target add wasm32-unknown-unknown
```

For the actual setup, this is, again, a modified diamond problem, but the failure isn't to do with version mismatches. The dependency graph is as below;

```
      ┌─────────┐
      │memmap-rs│
      └─────────┘
           ▲
           ¦ optional
        ┌─────┐
   ┌───▶│lib-a│◀─ ─  not(target_arch = "wasm32)
   │    └─────┘    │ features = ["memmap"]
┌─────┐         ┌─────┐
│lib-b│         │lib-c│
└─────┘         └─────┘
   ▲    ┌─────┐    ▲
   └────│lib-d│─ ─ ┘ not(target_arch = "wasm32)
        └─────┘
```

Solid lines are non-optional, non-target-dependent dependencies. Dashed lines are either optional or target-specific.

`lib-a`'s dependency on `memmap` is `optional = true`.

`lib-c`'s dependency on `lib-a` and `lib-d`'s dependency on `lib-c` are both under `[target.'cfg(not(target_arch = "wasm32))'.dependencies]` sections.

It's important for this demonstration to know that `lib-b`'s dependency on `lib-a` does not active the optional dependency on `memmap-rs`, but `lib-c`'s target-specific dependency on `lib-a` does. The _intention_ being that when `lib-c` is built for a non-WASM target it includes `lib-a` with the `memmap` feature active; when built for WASM it is not dependent on `lib-a` at all.

###### Try running

```
cargo build -p lib-a
cargo build -p lib-b
cargo build -p lib-c
cargo build -p lib-d

cargo build -p lib-a --target wasm32-unknown-unknown
cargo build -p lib-b --target wasm32-unknown-unknown
cargo build -p lib-c --target wasm32-unknown-unknown
```

The above commands should yield nothing but green.

#### What Works

I'd like to highlight that when considering just `lib-c`, it seems that we've correctly encoded the `lib-a`-when-not-`wasm32` dependency. If you build `lib-c` with a cold cache, you can see that `lib-a` is not built for the WASM target;

*Building `lib-c` for Not-WASM*
```sh
$ rm -rf target; cargo build -p lib-c
   Compiling libc v0.2.62
   Compiling memmap v0.7.0
   Compiling lib-a v0.1.0 (/path/to/.../1_lib-a)
   Compiling lib-c v0.1.0 (/path/to/.../2_lib-c)
    Finished dev [unoptimized + debuginfo] target(s) in 1.79s
```


*Building `lib-c` for WASM*
```sh
$ rm -rf target; cargo build -p lib-c --target wasm32-unknown-unknown
   Compiling lib-c v0.1.0 (/path/to/.../2_lib-c)
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
```

With the setup as obvious as I can make it, you can probably guess what you're going to see next.

###### Now try running

```
cargo build -p lib-d --target wasm32-unknown-unknown
```

#### What's Not Working

On my system, the first compilation error printed is from `memmap-rs` v0.7.0,

```
error[E0433]: failed to resolve: use of undeclared type or module `MmapInner`
   --> /Users/drew/.cargo/registry/src/github.com-1ecc6299db9ec823/memmap-0.7.0/src/lib.rs:183:9
```

This is an entirely accurate error; `memmap-rs` does not -- and should not -- build when the target is `wasm32-unknown-unknown`. WASM backends don't expose a memory model that can be mmap'd, so it makes no semantic sense to ask for such an interface.

This exact error is thrown because `memmap-rs/src/lib.rs` brings the `MmapInner` struct into scope with the below,

```
#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows::MmapInner;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
use unix::MmapInner;
```

None of the above will be compiled for a WASM target because they are neither `windows` or `unix`. Again, I consider this to be correct.

##### But How Did We Get Here?

As a quick demonstration of the core issue, the below commands will yield identical errors,

```
(cd 3_lib-d; cargo build --target wasm32-unknown-unknown;)
(cd 1_lib-a; cargo build --target wasm32-unknown-unknown --features="memmap";)
```

_NB. We have to `cd` into `1_lib-a/` for the `--features` flag to correctly resolve; cargo workspaces currently don't have a mechanism to pass feature flags to member packages via the CLI._

Having shown that building `lib-a` with both `--target wasm32-unknown-unknown` and `--features="memmap"` triggers the error we're modeling, the question becomes one of feature resolution; how does building `lib-d` with `--target wasm32-unknown-unknown` result in `lib-a` building with the `memmap` feature active?

The pithy answer is "cargo feature resolution."

The longer answer lies in three crucial behaviors of cargo's dependency resolution;
1. A crate's dependency graph is finalized before any pruning occurs.
   For this demonstration, that means cargo will walk and record `lib-d`'s dependency on `lib-c`, and `lib-c`'s dependency on `lib-a` regardless of the stated target.
2. Target specifications are recorded, and dependency pruning occurs on edges.
   `lib-d` and `lib-c` are nodes on the dependency graph. `lib-d`'s direct dependency on `lib-c` is an edge in this graph. That edge stores the `not(target_arch = "wasm32")` specification, and is what will be pruned when `lib-d` is built for a WASM target. _After_ that edge is pruned, `lib-c` will be removed from the build set because it is no longer a dependent of any other crate. The `lib-c` -> `lib-a` edge will also be pruned, but `lib-a` will remain in the build set, because it remains a dependent of `lib-b`.
3. Features are stored on nodes.
   The `memmap` feature activated in `lib-c`'s dependency on `lib-a` is stored in the `lib-a` node, _not_ on the `lib-c` -> `lib-a` edge. When that edge is pruned, there is no record that the `memmap` feature came from the `lib-c` node, and remains in the graph.

Therefore building `lib-d` with `--target wasm32-unknown-unknown` will result in `lib-a` building with the `memmap` feature flag active.

Therefore `memmap-rs` will (not) be compiled on a platform that does not expose the `mmap` interface.

#### Final Thoughts

This class of error -- and, in fact, this _exact_ error -- already exists the wild. The assumption that target-specific dependencies provide target-specific features is an easy one to make, especially when considering something in the below form,

```toml
[target.'cfg(not(unix))'.dependencies]
some-lib = { version = "0.1.0" }

[target.'cfg(unix)'.dependencies]
some-lib = { version = "0.1.0", features = ["optimize-for-unix"] }
```

It's easy to read this and think `some-lib` will always be a dependency of this crate, and that it will only be optimized for unix when `cfg(unix)` resolves to true. That the `optimize-for-unix` feature flag will always be active on `some-lib` -- and that there is currently no way to encode this kind of conditional feature activation -- was surprising, and remains frustrating, for me.

##### There is a Work-Around

Not for a system that operates automatically, but conditional features can be hand-encoded. See [part two](../2_solved-with-features/) for an exploration of one such work-around.
