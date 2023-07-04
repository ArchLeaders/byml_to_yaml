use anyhow_ext::Result;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Endianness {
    Big,
    Little,
}

impl std::str::FromStr for Endianness {
    type Err = anyhow_ext::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "be" | "big" | "big endian" | "bigendian" => Ok(Self::Big),
            "le" | "little" | "little endian" | "littleendian" => Ok(Self::Little),
            _ => anyhow_ext::bail!("Invalid endianness"),
        }
    }
}
