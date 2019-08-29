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


// Pick exactly one implementation of `foo()` based on which feature is set.
#[cfg(all(feature = "f-a", not(any(feature = "f-b", feature = "f-c"))))]
pub fn foo() {
    println!("Called `lib-a::foo()` in lib-a with feature \"f-a\" set.");
}

#[cfg(all(feature = "f-b", not(any(feature = "f-a", feature = "f-c"))))]
pub fn foo() {
    println!("Called `lib-a::foo()` in lib-a with feature \"f-b\" set.");
}

#[cfg(all(feature = "f-c", not(any(feature = "f-a", feature = "f-b"))))]
pub fn foo() {
    println!("Called `lib-a::foo()` in lib-a with feature \"f-c\" set.");
}

// NB. We use the fully qualified `all(.., not(any(..)))` to avoid redefinition
//     errors. Comment out the triplet above, uncomment the below, and build
//     exe-d for an example of what I mean.

// #[cfg(feature = "f-a")]
// pub fn foo() {
//     println!("Called `lib-a::foo()` in lib-a with feature \"f-a\" set.");
// }

// #[cfg(feature = "f-b")]
// pub fn foo() {
//     println!("Called `lib-a::foo()` in lib-a with feature \"f-b\" set.");
// }

// #[cfg(feature = "f-c")]
// pub fn foo() {
//     println!("Called `lib-a::foo()` in lib-a with feature \"f-c\" set.");
// }
