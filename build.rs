
fn main() {
    tonic_build::configure()
        .out_dir("src/generated")
        .compile(&["proto-definitions/tenantm.proto"], &["proto-definitions"])
        .expect("Failed to compile proto files");
}
