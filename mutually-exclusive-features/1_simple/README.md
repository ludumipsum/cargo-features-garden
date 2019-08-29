## A Simple Scenario

Say there's more than one way to skin a cat.
Say those ways are mutually exclusive.
Say you decide to use features to allow users to pick their poison.

#### Setup

```
        ┌─────┐  Exposes three features,
        │lib-a│  `f-a`, `f-b`, and `f-c`.
        └─────┘  Only one may be active.
           ▲
   ┌───────┼───────┬───────────────────────┐
   │       │       │  Each enables one of  │  Enable `f-a`+`f-b`.
   │       │       │  `f-a`, `f-b`, `f-c`. │
┌─────┐ ┌─────┐ ┌─────┐                 ┌─────┐
│exe-a│ │exe-b│ │exe-c│                 │exe-d│
└─────┘ └─────┘ └─────┘                 └─────┘
```

See `lib-a/src/lib.rs` for a the necessary boilerplate.

###### Try these!

- ```sh
  cargo build -p lib-a
  ```
  This should fail with,
  > `error: One of f-a, f-b, or f-c must be enabled`

- ```sh
  cargo run -p exe-a
  ```
  This should work.

- ```sh
  cargo run -p exe-b
  ```
  This should work.

- ```sh
  cargo run -p exe-c
  ```
  This should work.

- ```sh
  cargo run -p exe-d
  ```
  Should fail with,
  > `error: f-a, f-b, and f-c are mutually exclusive features`
