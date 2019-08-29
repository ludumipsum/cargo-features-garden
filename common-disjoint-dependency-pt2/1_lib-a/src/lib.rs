// Apply the `#![no_std]` attribute if `not(feature = "std")` resolves to true.
#![cfg_attr(not(feature = "std"), no_std)]

pub fn check_compatibility() {
    #[cfg(feature = "std")]
    let msg = "\n\n\tlib-a was *not* built with `no_std`\n\n";
    #[cfg(not(feature = "std"))]
    let msg = "\n\n\tGood news! lib-a *was* built with `no_std`!\n\n";

    panic!(msg);
}
