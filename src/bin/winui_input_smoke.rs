#[cfg(target_os = "windows")]
#[path = "winui_input_smoke/mod.rs"]
mod windows_runner;

#[cfg(target_os = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    windows_runner::main()
}

#[cfg(not(target_os = "windows"))]
fn main() {
    eprintln!("a3s-gui-winui-input-smoke requires Windows.");
    std::process::exit(2);
}
