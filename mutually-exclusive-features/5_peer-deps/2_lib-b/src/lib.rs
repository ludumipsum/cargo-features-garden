pub fn foo() {
    println!("Called `lib-b::foo()`, dropping into lib-a...");
    lib_a::bar();
}
