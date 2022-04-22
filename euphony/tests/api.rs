#[test]
fn generate() {
    euphony_node::reflect::generate_api(
        concat!(env!("CARGO_MANIFEST_DIR"), "/../euphony-dsp"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/src/processors.rs"),
    );
}
