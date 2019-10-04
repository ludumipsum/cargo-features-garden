pub fn check_compatibility() {
    #[cfg(feature = "incompatible")]
    let msg = "\n\n\tlib-a is not compatible with exe-a!\n\n";
    #[cfg(not(feature = "incompatible"))]
    let msg = "\n\n\tGood news! lib-a is compatible with exe-a!\n\n";

    panic!(msg);
}
