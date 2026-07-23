use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use a3s_gui::{
    NativeBackendKind, NativeCapabilities, NativeInputConformanceManifestV1,
    NativeInputConformanceRunV1,
};

fn main() -> ExitCode {
    match execute() {
        Ok(code) => code,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}

fn execute() -> Result<ExitCode, String> {
    let mut arguments = env::args().skip(1);
    let Some(command) = arguments.next() else {
        return Err(usage());
    };
    match command.as_str() {
        "manifest" => {
            let backend = arguments
                .next()
                .ok_or_else(usage)
                .and_then(|value| parse_backend(&value))?;
            reject_extra_arguments(arguments)?;
            let manifest = NativeInputConformanceManifestV1::from_capabilities(
                &NativeCapabilities::for_backend(backend),
            );
            print_json(&manifest)?;
            Ok(ExitCode::SUCCESS)
        }
        "verify" => {
            let evidence_path = arguments.next().ok_or_else(usage)?;
            reject_extra_arguments(arguments)?;
            let run = read_run(Path::new(&evidence_path))?;
            let manifest = NativeInputConformanceManifestV1::from_capabilities(
                &NativeCapabilities::for_backend(run.backend),
            );
            let report = manifest.verify(&run);
            let conformant = report.is_conformant();
            print_json(&report)?;
            Ok(if conformant {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            })
        }
        _ => Err(usage()),
    }
}

fn parse_backend(value: &str) -> Result<NativeBackendKind, String> {
    match value.to_ascii_lowercase().as_str() {
        "appkit" => Ok(NativeBackendKind::AppKit),
        "gtk4" => Ok(NativeBackendKind::Gtk4),
        "winui" => Ok(NativeBackendKind::WinUI),
        _ => Err(format!(
            "unsupported native backend {value:?}; expected appkit, gtk4, or winui"
        )),
    }
}

fn reject_extra_arguments(mut arguments: impl Iterator<Item = String>) -> Result<(), String> {
    if arguments.next().is_some() {
        Err(usage())
    } else {
        Ok(())
    }
}

fn read_run(path: &Path) -> Result<NativeInputConformanceRunV1, String> {
    let contents = fs::read_to_string(path).map_err(|error| {
        format!(
            "failed to read native input evidence {}: {error}",
            path.display()
        )
    })?;
    serde_json::from_str(&contents).map_err(|error| {
        format!(
            "failed to parse native input evidence {}: {error}",
            path.display()
        )
    })
}

fn print_json(value: &impl serde::Serialize) -> Result<(), String> {
    let json = serde_json::to_string_pretty(value)
        .map_err(|error| format!("failed to serialize conformance artifact: {error}"))?;
    println!("{json}");
    Ok(())
}

fn usage() -> String {
    "usage: a3s-gui-native-input-conformance manifest <appkit|gtk4|winui> | verify <evidence.json>"
        .to_string()
}
