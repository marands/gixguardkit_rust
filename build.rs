fn main() {
    uniffi_build::generate_scaffolding("./src/gix_guard.udl")
        .unwrap();
}