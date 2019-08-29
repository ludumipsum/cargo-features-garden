## A Diamond Problem

A simple, but entirely intractable failure state for mutually exclusive libraries.

#### Setup

```
      ┌─────┐  Exposes three features,
      │lib-a│  `f-a`, `f-b`, and `f-c`.
      └─────┘  Only one may be active.
     f-a ▲ f-b
   ┌─────┴────┐
┌─────┐    ┌─────┐
│lib-b│    │lib-c│
└─────┘    └─────┘
   ▲          ▲
   └─────┬────┘
      ┌─────┐
      │exe-a│
      └─────┘
```

If your dependencies make Strong Choices about implementation details and disagree with one another, the best you're going to get is a decent error message.
