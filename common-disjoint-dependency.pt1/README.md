## Shared but Disjoint Dependencies

### Setup
This is going to be a bit of a long walk because the problem being modeled is primarily a linker-time concern. As such we're going to be running some code to demonstrate the issue, and looking at the emitted messages to determine how our crates were compiled.

We're pretending to be the authors of `exe-a` for this exercise, with the _conceptual_ dependency graph as below.

```
┌─────┐ ┌─────┐
│lib-a│ │lib-a│
└─────┘ └─────┘
   ▲       ▲
   │       │ features = ["incompatible"]
   │    ┌─────┐
   │    │lib-b│
   │    └─────┘
   │       ▲
   ├───────┘ [build-dependencies]
┌─────┐
│exe-a│
└─────┘
```

`exe-a` relies on `lib-a` as a primary dependency for the generated executable, and we're using `lib-b` as a build dependency for some undefined convenience. Maybe we're cross-compiling or doing some code-gen, and want the host to generate something that's baked into the target binary or consumed by some monitoring system.

`lib-a` exposes an optional feature, `incompatible`, that is (as the name implies) incompatible with our final binary. We don't activate it in the `lib-a` dependency in `3_exe-a/Cargo.toml`.

###### Take a look at

```sh
cargo run -p exe-a
```

#### Explanation

For whatever reason, the authors of `lib-b` have chosen to activate the `incompatible` feature flag. There are any number of reasons why this might have been done, but (to prime ourselves for an upcoming discussion) let's pretend it's because `exe-a` is being written for an uncommon usecase that the authors of `lib-b` simply haven't considered.

The dependency graph above is, of course, inaccurate. Only one instance of `lib-a` exists in the final build set, and that version -- that both `exe-a` and `lib-b` link against -- will be built with the `incompatible` flag active.

###### Try

Commenting out `exe-a`s dependency on `lib-b` (in 3_exe-a/Cargo.toml), and re-running `cargo run -p exe-a`.

#### The Actual Setup

The initial setup was somewhat disingenuous; the problem being modeled isn't to do with active-positives (actively turning a feature flag on), rather it's to do with the absence of active-negatives (failing to actively turning a feature flag off).

The Rust `std` library is the only opt-out dependency in the Rust ecosystem. The default, no-op, do-this-unless-told-otherwise behavior for crates is to implicitly include a dependency on `std` and link against that library. This informs the typical structure of crates that _might be_ `no_std`, and the way dependent crates need to craft themselves to correctly expose this optional-active-negative.

#### This is Getting a Bit Theoretical...

Let's keep things concrete and continue this discussion in [Common Disjoint Dependencies Part 2][].

[Common Disjoint Dependencies Part 2]: ../common-disjoint-dependency.pt2/
