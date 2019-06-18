#![warn(rust_2018_idioms)]

use codespan_reporting::Diagnostic;
use ddl_core as core;
use std::io;
use std::io::prelude::*;

pub fn compile_module(
    writer: &mut impl Write,
    module: &core::Module,
) -> io::Result<Vec<Diagnostic>> {
    let diagnostics = Vec::new();

    let pkg_name = env!("CARGO_PKG_NAME");
    let pkg_version = env!("CARGO_PKG_VERSION");

    writeln!(writer, "<!--")?;
    writeln!(
        writer,
        "  This file is automatically @generated by {} {}",
        pkg_name, pkg_version,
    )?;
    writeln!(writer, "  It is not intended for manual editing.")?;
    writeln!(writer, "-->")?;

    for item in &module.items {
        match item {
            core::Item::Struct { doc, name, .. } => {
                writeln!(writer)?;
                writeln!(writer, "## {}", name)?;

                if !doc.is_empty() {
                    writeln!(writer)?;
                    writeln!(writer, "{}", doc)?;
                }
            }
        }
    }

    Ok(diagnostics)
}
