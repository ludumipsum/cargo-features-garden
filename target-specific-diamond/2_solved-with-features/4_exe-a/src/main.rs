#[cfg(target_arch = "wasm32")]
compile_error!("\n\
    exe-a is intended to only build for native targets\n\
    Please rerun this build without `--target wasm32-unknown-unknown`."
);

fn main() {
    println!("Hello, world!");
}
