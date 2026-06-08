pub mod emit_rust;
pub mod model;
pub mod scan;

pub fn compile_to_rust(src: &str) -> Result<String, String> {
    let app = scan::scan_app(src)?;
    Ok(emit_rust::emit_rust(&app))
}
