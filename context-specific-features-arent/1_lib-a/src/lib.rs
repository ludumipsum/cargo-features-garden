#[cfg(feature = "throw_compile_error")]
compile_error!("This feature should never be set");

#[cfg(feature = "target_macos")]
compile_error!("The feature \"target_macos\" is set");

#[cfg(feature = "target_wasm")]
compile_error!("The feature \"target_wasm\" is set");

#[cfg(feature = "target_windows")]
compile_error!("The feature \"target_windows\" is set");

#[cfg(feature = "profile_dev")]
compile_error!("The feature \"profile_dev\" is set");

#[cfg(feature = "profile_build")]
compile_error!("The feature \"profile_build\" is set");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
