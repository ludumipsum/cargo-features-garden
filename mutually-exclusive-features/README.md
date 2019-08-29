## Mutually Exclusive Features

This is less an exploration of the behaviors of Cargo -- Cargo doesn't provide any mechanism fo Mutually Exclusive Features -- and more an exploration of possible work-arounds.

#### Setup

Say you're writing a library and you've found more than one solution to the problem you're solving. Each has its own tradeoffs; maybe the most space-efficient isn't available on all platforms, or the fastest has potential security concerns. Point is, you don't want to enforce a choice on your users.

One way to handle this would be to provide multiple implementations, and only expose (or build) one at a time. Your forward-facing API doesn't need to change, so long as the backend operations do. You can let users pick an implementation by enabling one of -- and only one of -- the related Geature flags at compilation time.

Let's take a look at how that might be handled.
