## Intro

The *Cargo Features Garden* is a collection of examples demonstrating the sometimes surprising behavior of [Cargo Features][]. The intent is to take a clear but critical look at how Cargo Features work under the hood and how the Cargo.toml manifest may contribute to misunderstandings.

[Cargo Features]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section

## Structure

The directories in this repository aim to be self-descriptive, and contain Cargo Workspaces or contain a series of directories that contain workspaces. Many of the crates in these repository will not compile and this is as intended. If running `cargo build` -- without specifying a `--package` -- in any of these workspaces _does not_ yield an error, that is purely by coincidence. Each of these directories contains a README.md that will discuss the contents, the crates intended to be built, and the expected results.

It will be common for directories in a workspace to be named something like `1_lib-a`, `1_lib-b`, `2_exe-a`. In this case, the crates will be named `lib-a`, `lib-b`, and `exe-a`, and the numerical prefix will suggest the crate's level in the implied dependency tree. `lib-a` and `lib-b` are at the lowest level, so neither depend on any crate in the workspace. `exe-a` is at the second level, probably because it depends on one or both of the libraries.

## Table of Contents

This might also be a suggested reading order. If so, the suggestion is very loose.

###### [Context Specific Features Aren't][]
A relatively straight-forward demonstration of the possibly surprising nature of feature unification.

###### [Common Disjoint Dependency][]
An exploration of union'd feature flags focusing on `no_std`. (Well, it builds into a discussion of `no_std`. Spoilers, I guess.)

###### [Target Specific Diamond][]
An exploration of union'd feature flags focusing on deeply nested, target-specific features.

###### [Mutually Exclusive Features][]
An exploration of possible methods to encode mutually exclusive features into crates.

[Context Specific Features Aren't]: context-specific-features-arent/
[Common Disjoint Dependency]: common-disjoint-dependency.pt1/
[Target Specific Diamond]: target-specific-diamond/
[Mutually Exclusive Features]: mutually-exclusive-features/

## Ongoing Work

Below is a list of active, interrelated issues relating to Cargo Features, brief descriptions of them, and links to salient GitHub issues (mostly in the [`rust-lang/cargo` Features issue list][features-issues].

[features-issues]: https://github.com/rust-lang/cargo/issues?q=is%3Aopen+is%3Aissue+label%3AA-features

-----

### Cargo Features are Always Unified

When you include a crate in more than one context -- in both `[dev-dependencies]` and `[build-dependencies]`, behind `[target.'cfg(...)'.dependencies]` expressions, transitively from other dependencies, etc. -- all of the features activated across all instances of the dependency are union'd prior to a build. That means you could write something like,

```toml
[dependencies]
some-lib = *

[target.'cfg(false)'.dev-dependencies]
some-lib = { version = "*", features = ["broken-experimental"] }
```

expecting `some-lib` to never have the `broken-experimental` feature activated. In reality, no mater what target or profile this crate is built with, the `broken-experimental` feature will always be active on `some-lib`.

#### Examples

 - [Context Specific Features Aren't][]
 - [Common Disjoint Dependency][]
 - [Target Specific Diamond][]

#### Issues

The rust-lang issues and discussions surrounding this behavior can be broken down into three rough categories;

- **Target Specific Features**
  - [rust-lang/cargo#1197][]
  - [rust-lang/cargo#2524][]
  - [rust-lang/cargo#6571][]

  _Additional duplicates w/o meaningful conversation; [rust-lang/cargo#3741][], [rust-lang/cargo#6870][], [rust-lang/cargo#7292][]_

- **`[build-dependencies]` interacting with `[dependencies]`**
  - [rust-lang/cargo#2589][]
  - [rust-lang/cargo#4361][]
  - [rust-lang/cargo#4866][]
  - [rust-lang/cargo#5237][]
  - [rust-lang/cargo#5730][] _(The comments in this one are important)_

  _There are a couple issues that are more correctly dependency interference issues, but may be correctly handled in the conversation regarding `[build-dependencies]`._
  - [rust-lang/cargo#5304][]
  - [rust-lang/cargo#5760][]

- **`[dev-dependencies]` interacting with `[dependencies]`**
  - [rust-lang/cargo#1596][]
  - [rust-lang/cargo#1796][]
  - [rust-lang/cargo#4664][]

[rust-lang/cargo#1197]: https://github.com/rust-lang/cargo/issues/1197
[rust-lang/cargo#2524]: https://github.com/rust-lang/cargo/issues/2524
[rust-lang/cargo#6571]: https://github.com/rust-lang/cargo/issues/6571
[rust-lang/cargo#3741]: https://github.com/rust-lang/cargo/issues/3741
[rust-lang/cargo#6870]: https://github.com/rust-lang/cargo/issues/6870
[rust-lang/cargo#7292]: https://github.com/rust-lang/cargo/issues/7292
[rust-lang/cargo#2589]: https://github.com/rust-lang/cargo/issues/2589
[rust-lang/cargo#4361]: https://github.com/rust-lang/cargo/issues/4361
[rust-lang/cargo#4866]: https://github.com/rust-lang/cargo/issues/4866
[rust-lang/cargo#5237]: https://github.com/rust-lang/cargo/issues/5237
[rust-lang/cargo#5730]: https://github.com/rust-lang/cargo/issues/5730
[rust-lang/cargo#5304]: https://github.com/rust-lang/cargo/issues/5304
[rust-lang/cargo#5760]: https://github.com/rust-lang/cargo/issues/5760
[rust-lang/cargo#1596]: https://github.com/rust-lang/cargo/issues/1596
[rust-lang/cargo#1796]: https://github.com/rust-lang/cargo/issues/1796
[rust-lang/cargo#4664]: https://github.com/rust-lang/cargo/issues/4664

-----

### Activating a Feature that Activates a Feature on an Optional Dependency also Activates the Optional Dependency

This one is somewhat self-explanatory, if you can parse the above title. If you have an optional dependency,

```toml
optional-lib = {version = "*", optional = true }
```

... and a feature that enables a feature on that optional dependency,

```toml
some-feature = ["optional-lib/foo"]
```

... activating that feature also activates the optional dependency.
Which means that this,

```toml
# Activate the dependency on `optional-lib`, and the feature `optional-lib/foo`.
my-lib = { features = ["optional-lib", "some-feature"] }
```

... is the same as this,

```toml
# Activate `optional-lib/foo` and imply the dependency on `optional-lib`.
my-lib = { features = ["some-feature"] }
```


#### Issues
- [rust-lang/cargo#3494][]

_Additional duplicates w/o meaningful conversation; [rust-lang/cargo#5023][], [rust-lang/cargo#6658][], [rust-lang/cargo#7259][]_

[rust-lang/cargo#3494]: https://github.com/rust-lang/cargo/issues/3494
[rust-lang/cargo#5023]: https://github.com/rust-lang/cargo/issues/5023
[rust-lang/cargo#6658]: https://github.com/rust-lang/cargo/issues/6658
[rust-lang/cargo#7259]: https://github.com/rust-lang/cargo/issues/7259

-----

### Mutually Exclusive Features

When authoring code that needs to be platform-aware, it seems to be a common pattern to homogenize system-level operations by implementing similarly named modules, and `use`ing them like,

```rust
#[cfg(unix)]
pub use crate::sys::unix as sys;
#[cfg(windows)]
pub use crate::sys::windows as sys;
#[cfg(target_arch = "wasm32")]
pub use crate::sys::wasm as sys;
```

This can also be a useful pattern for selecting a specific implementation of a more general operation. For example [`flate2`][] defaults to using miniz as a compression/decompression library, but can optionally be configured to use zlib, or miniz_oxide. Building `flate2` with `cargo build --features "zlib rust_backend"` throws no errors, but it's unclear (to me, at least) which backend will be used in that case.

It seems a reasonable ask to for Cargo to provide a mechanism for enforcing mutually exclusive features.

[`flate2`]: https://github.com/alexcrichton/flate2-rs

#### Examples

 - [Mutually Exclusive Features][]

#### Issues & Discussions

 - [rust-lang/cargo#2980][]
 - [internals.rust-lang #8601](https://internals.rust-lang.org/t/mutually-exclusive-feature-flags/8601)

[rust-lang/cargo#2980]: https://github.com/rust-lang/cargo/issues/2980

-----

### Workspaces do _not_ Play Well with Features

I need to do more research here to dig into the specifics. Generally, Features aren't great at multiple contexts, and Workspaces aren't well polished. The interaction between these two features results in a less than stellar user experience.

#### Issues
  - [rust-lang/cargo#4463][]
  - [rust-lang/cargo#5015][]
  - [rust-lang/cargo#5364][]
  - [rust-lang/cargo#6458][]
  - [rust-lang/cargo#7218][]

  _Additional duplicates w/o meaningful conversation; [rust-lang/cargo#5251][]_

[rust-lang/cargo#4463]: https://github.com/rust-lang/cargo/issues/4463
[rust-lang/cargo#5015]: https://github.com/rust-lang/cargo/issues/5015
[rust-lang/cargo#5364]: https://github.com/rust-lang/cargo/issues/5364
[rust-lang/cargo#6458]: https://github.com/rust-lang/cargo/issues/6458
[rust-lang/cargo#7218]: https://github.com/rust-lang/cargo/issues/7218
[rust-lang/cargo#5251]: https://github.com/rust-lang/cargo/issues/5251
