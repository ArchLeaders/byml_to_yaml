use std::{env::args, path::Path};

use roead::{byml::Byml, Endian};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = args().collect();
    return convert_to_yaml(args[1].as_ref(), args[2].as_ref());
}

fn convert_to_yaml(path: &Path, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let buffer: Vec<u8> = std::fs::read(path)?;
    let byml = Byml::from_binary(&buffer)?;

    std::fs::write(
        output,
        byml.to_binary_with_version(Endian::Big, 7)
    )?;

    return Ok(());
}
