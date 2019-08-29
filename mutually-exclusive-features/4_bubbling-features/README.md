## Bubbling a Dependency's Feature

Say you're building on top of a library that exposes mutually exclusive features. You don't want to leak your dependency on that library, but you do want to allow users to pick which mutually exclusive feature to enable.

#### Setup

```
        ┌─────┐  Exposes three features,
        │lib-a│  `f-a`, `f-b`, and `f-c`.
        └─────┘  Only one may be active.
        ▲  ▲  ▲
        └─┐│┌─┘  Each of these is `optional = true`,
          │││    each enables one of `f-a`, `f-b`, or `f-c`.
        ┌─────┐
        │lib-b│
        └─────┘
           ▲
   ┌───────┼───────┐  Each enables one of
   │       │       │  `f-a`, `f-b`, `f-c`.
┌─────┐ ┌─────┐ ┌─────┐
│exe-a│ │exe-b│ │exe-c│
└─────┘ └─────┘ └─────┘
```

This -- somewhat surprisingly, to me -- works.
There are some... tricks that need to be pulled though.

The short short story is "Rename your dependencies, then rename them back."

The long story starts with `2_lib-b/Cargo.toml`, and goes from there.
