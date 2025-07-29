fn main() {
    slint_build::compile_with_config(
        "slint_ui/application.slint",
        slint_build::CompilerConfiguration::new()
            .with_style("cupertino-light".to_string())
    ).unwrap();
    // compile("slint_ui/application.slint").unwrap();
}