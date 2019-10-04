#[cfg(not(target_arch = "wasm32"))]
compile_error!("\n\
    wlib-a is intended to only build for wasm32 targets\n\
    Please rerun this build with `--target wasm32-unknown-unknown`."
);

fn main() {
    println!("Hello, world!");
}
