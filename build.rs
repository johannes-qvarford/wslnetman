fn main() {
    slint_build::compile("src/ui/main_ui.slint").unwrap();

    // Embed icon for Windows builds
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("icon.ico");
        res.compile().unwrap();
    }
}
