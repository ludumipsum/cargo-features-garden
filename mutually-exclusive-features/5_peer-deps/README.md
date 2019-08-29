Same setup as bubbled features, but this time you don't mind letting the dependency on `lib-a` leak. `lib-b` no longer exposes any features, and includes nothing but an unadorned dependency on `lib-a`.

Note that `lib-b` will fail to compile because `lib-a` requires exactly one feature to be enabled, and `lib-b` enables exactly zero. It's kind of an incomplete dependency. The edge from `lib-b' to `lib-a` is there, but the `lib-a` node is hollow.

It's now up to users of `lib-b` to also include `lib-a` and specify the feature they want to use, even -- and especially -- if they don't want to directly use `lib-a`.

```
              ┌─────┐ Exposes three features,
Specifies  ┌─▶│lib-a│ `f-a`, `f-b`, and `f-c`.
no feature │  └─────┘ Only one may be active.
        ┌─────┐  ▲
        │lib-b│  |                 Never directly
        └─────┘  ├ ─ ─ ─ ┬ ─ ─ ─ ┐ `use`d. Only
           ▲     |       |       | specifies a feature.
           │  ┌─────┐ ┌─────┐ ┌─────┐
           └──┤exe-a├─┤exe-b├─│exe-c│
              └─────┘ └─────┘ └─────┘
```

I call this a "Peer Dependency" -- even though it's not quite the same concept as NPM's peer deps -- because `lib-b` requires its parents to also depend on `lib-a`. In that way, for `lib-b` to build, it needs `lib-a` to be it's sibling---it's peer. Event though `lib-b` does depend on (an incomplete?) `lib-a`.

I actually think this is strictly inferior to the bubbled features. I thought that pattern simply wasn't going to work, so I decided to prove this one out. But I was wrong -- bubbling mutually exclusive features does work. This works too, but it's... a bit harry, conceptually, and there's no way to encode the peer dependency in Cargo. (Aside from compiler errors, and good comments in multiple `Cargo.toml`s. Which is, at best, a half-measure.)
