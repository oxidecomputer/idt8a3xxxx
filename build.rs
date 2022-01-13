use anyhow::{bail, Result};
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::fmt::Write;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
enum Contents {
    Byte,
    Word,
    Word24,
    Word32,
    Word40,
    Word48,
    Frequency,
    TimeOfDay,
}

#[derive(Debug, Deserialize)]
struct Register(u16, String, Contents);

#[derive(Debug, Deserialize)]
struct Module {
    base: Vec<u16>,
    registers: Vec<Register>,
}

type Modules = HashMap<String, Module>;

fn output_modules(modules: &Modules) -> Result<String> {
    let mut s = String::new();
    let mut sorted = BTreeMap::new();

    for (name, module) in modules {
        sorted.insert(module.base[0], name.to_string());
    }

    writeln!(
        &mut s,
        r##"
pub fn modules() -> &'static [Module<'static>] {{
    &["##
    )?;

    for (_, name) in &sorted {
        let module = modules.get(name).unwrap();

        write!(
            &mut s,
            r##"
        Module {{
            name: "{}",
            base: &["##,
            name,
        )?;

        for b in &module.base {
            write!(&mut s, "0x{:x}, ", b)?;
        }

        writeln!(
            &mut s,
            r##"],
            registers: &["##,
        )?;

        for r in &module.registers {
            writeln!(
                &mut s,
                r##"
                Register {{
                    name: "{}",
                    offset: 0x{:x},
                    contents: Contents::{:?},
                }},"##,
                r.1, r.0, r.2
            )?;
        }

        writeln!(&mut s, "            ]\n        }},")?;
    }

    writeln!(&mut s, "    ]\n}}")?;

    Ok(s)
}

fn codegen() -> Result<()> {
    use std::io::Write;

    let mut dir = PathBuf::from(&env::var("CARGO_MANIFEST_DIR")?);
    dir.push("src");
    dir.push("registers.ron");

    let f = match File::open(dir) {
        Ok(f) => f,
        Err(e) => {
            bail!("failed to open registers.ron: {}", e);
        }
    };

    let modules: Modules = match from_reader(f) {
        Ok(modules) => modules,
        Err(e) => {
            bail!("failed to parse register registers.ron: {}", e);
        }
    };

    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("modules.rs");
    let mut file = File::create(&dest_path)?;
    let out = output_modules(&modules)?;

    file.write_all(out.as_bytes())?;

    Ok(())
}

fn main() {
    if let Err(e) = codegen() {
        println!("code generation failed: {}", e);
        std::process::exit(1);
    }

    println!("cargo:rerun-if-changed=src/registers.ron");
    println!("cargo:rerun-if-changed=build.rs");
}
