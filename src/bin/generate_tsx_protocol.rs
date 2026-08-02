use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use a3s_gui::{generate_tsx_typescript_protocol_v1, TSX_TYPESCRIPT_PROTOCOL_RELATIVE_PATH};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    let generated = generate_tsx_typescript_protocol_v1().map_err(|error| error.to_string())?;
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(TSX_TYPESCRIPT_PROTOCOL_RELATIVE_PATH);

    match arguments.as_slice() {
        [mode] if mode == "--write" => {
            let parent = path.parent().ok_or_else(|| {
                format!("generated protocol path has no parent: {}", path.display())
            })?;
            fs::create_dir_all(parent).map_err(|error| {
                format!("failed to create generated protocol directory: {error}")
            })?;
            fs::write(&path, generated)
                .map_err(|error| format!("failed to write {}: {error}", path.display()))?;
            println!("wrote {}", path.display());
            Ok(())
        }
        [mode] if mode == "--check" => {
            let checked_in = fs::read_to_string(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            if checked_in != generated {
                return Err(format!(
                    "{} is stale; regenerate it with --write",
                    path.display()
                ));
            }
            println!("checked {}", path.display());
            Ok(())
        }
        [mode] if mode == "--stdout" => {
            print!("{generated}");
            Ok(())
        }
        _ => Err("usage: a3s-gui-generate-tsx-protocol (--write | --check | --stdout)".to_string()),
    }
}
