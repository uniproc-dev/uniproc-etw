use std::path::Path;

fn text(file: &Path) -> String {
    std::fs::read_to_string(file).unwrap_or_default().replace("\r\n", "\n")
}

#[test]
fn the_bindings_are_what_their_filter_generates() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let committed = root.join("src").join("bindings.rs");
    let generated = std::env::temp_dir().join(format!("uniproc-etw-bindings-{}.rs", std::process::id()));
    windows_bindgen::builder()
        .input_default()
        .flat()
        .filter_file(root.join("src").join("bindings.txt"))
        .output(&generated)
        .write();
    let fresh = text(&generated);
    let _ = std::fs::remove_file(&generated);
    if std::env::var_os("UNIPROC_ETW_REGENERATE").is_some() {
        std::fs::write(&committed, &fresh).unwrap();
    }
    assert!(
        text(&committed) == fresh,
        "src/bindings.rs is not what src/bindings.txt generates: run UNIPROC_ETW_REGENERATE=1 cargo test --test bindings"
    );
}
