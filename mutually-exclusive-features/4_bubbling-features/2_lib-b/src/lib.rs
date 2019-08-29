// I hope you like boilerplate...

// Ensure at least one feature is set.
#[cfg(not(any(feature = "f-a", feature = "f-b", feature = "f-c")))]
compile_error!("One of f-a, f-b, or f-c must be enabled");

// Ensure only one feature is set.
#[cfg(any(
    all(feature = "f-a", any(feature = "f-b", feature = "f-c")),
    all(feature = "f-b", any(feature = "f-a", feature = "f-c")),
    all(feature = "f-c", any(feature = "f-a", feature = "f-b")),
))]
compile_error!("f-a, f-b, and f-c are mutually exclusive features");


// Pick the renamed `lib-a-f*` based on which feature is enabled, and rename it
// back to just `lib-a`.
#[cfg(feature = "f-a")]
use lib_a_fa as lib_a;

#[cfg(feature = "f-b")]
use lib_a_fb as lib_a;

#[cfg(feature = "f-c")]
use lib_a_fc as lib_a;

// Actually use `lib_a`.
pub fn foo() {
    println!("Called `lib-b::foo()`, dropping into lib-a...");
    lib_a::bar();
}
