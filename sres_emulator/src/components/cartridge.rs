//! Implementation of SFC file parsing.
use std::path::Path;

use anyhow::bail;
use anyhow::ensure;
use anyhow::Context;
use anyhow::Result;
use intbits::Bits;
use packed_struct::prelude::*;

#[derive(Clone, Default)]
pub struct Cartridge {
    pub header: SnesHeader,
    pub rom: Vec<u8>,
    pub sram: Vec<u8>,
}

impl Cartridge {
    pub fn with_sfc_data(data: &[u8], srm_data: Option<&[u8]>) -> Result<Cartridge> {
        let header = SnesHeader::find_header_in_rom(data)?;
        let sram = match srm_data {
            Some(srm_data) => {
                if srm_data.len() != header.sram_size {
                    bail!(
                        "Warning: SRAM size mismatch. Expected {} bytes, got {} bytes",
                        header.sram_size,
                        srm_data.len()
                    );
                }
                srm_data.to_vec()
            }
            None => vec![0; header.sram_size],
        };

        Ok(Cartridge {
            header,
            rom: data.to_vec(),
            sram,
        })
    }

    pub fn with_sfc_file(path: &Path) -> Result<Cartridge> {
        let srm_path = path.with_extension("srm");
        let srm_data: Option<Vec<u8>> = if srm_path.exists() {
            Some(std::fs::read(srm_path)?)
        } else {
            None
        };
        let sfc_data = std::fs::read(path)?;
        Self::with_sfc_data(&sfc_data, srm_data.as_deref())
    }

    pub fn with_program(program: &[u8]) -> Cartridge {
        Cartridge {
            header: SnesHeader::default(),
            rom: program.to_vec(),
            sram: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MappingMode {
    LoRom,
    HiRom,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnesHeader {
    pub name: String,
    pub fast_rom: bool,
    pub mapping_mode: MappingMode,
    pub rom_size: usize,
    pub sram_size: usize,
}

impl Default for SnesHeader {
    fn default() -> Self {
        Self {
            name: String::new(),
            fast_rom: false,
            mapping_mode: MappingMode::LoRom,
            rom_size: 0,
            sram_size: 0,
        }
    }
}

impl SnesHeader {
    fn find_header_in_rom(rom: &[u8]) -> Result<Self> {
        let lorom_header = Self::try_header(rom, MappingMode::LoRom);
        let hirom_header = Self::try_header(rom, MappingMode::HiRom);

        match (lorom_header, hirom_header) {
            (Ok(lorom_header), Err(_)) => Ok(lorom_header),
            (Err(_), Ok(hirom_header)) => Ok(hirom_header),
            (Ok(lorom_header), Ok(hirom_header)) => {
                bail!(
                    "Failed to pick header. Both look ok.\n LoRom: {:?}\n HiRom: {:?}",
                    lorom_header,
                    hirom_header
                )
            }
            (Err(lorom_err), Err(hirom_err)) => {
                bail!(
                    "Failed to find header.\n LoRom: {:?}\n HiRom: {:?}",
                    lorom_err,
                    hirom_err
                )
            }
        }
    }

    fn try_header(rom: &[u8], mapping_mode: MappingMode) -> Result<Self> {
        let location = match mapping_mode {
            MappingMode::LoRom => 0x7FC0,
            MappingMode::HiRom => 0xFFC0,
        };
        if location + 0x20 > rom.len() {
            bail!("Header location out of bounds")
        }
        let header = Self::parse_header(&rom[location..(location + 0x20)]);
        if let Ok(header) = header {
            if header.name.trim_matches('\0').trim().is_empty() {
                bail!("Header in ${location:06X} has empty name")
            }
            if header.mapping_mode != mapping_mode {
                bail!("Header in ${location:06X} does not match mapping mode {mapping_mode:?}")
            }
            Ok(header)
        } else {
            header
        }
    }

    fn parse_header(data: &[u8]) -> Result<Self> {
        ensure!(data.len() >= 32, "Header too short");
        let raw = RawSnesHeader::unpack_from_slice(&data[0..32]).unwrap();
        Ok(SnesHeader {
            name: String::from_utf8(raw.name.to_vec())
                .with_context(|| "ROM name is not ASCII")?
                .trim()
                .to_string(),
            fast_rom: raw.mapping.bit(5),
            mapping_mode: match raw.mapping.bits(0..4) {
                0 => MappingMode::LoRom,
                1 => MappingMode::HiRom,
                other => bail!("Invalid mapping mode: {other}"),
            },
            rom_size: if raw.rom_size < 5 {
                32 * 1024 // Use 32kB as a minimum. The test roms specify 2kB but provide 32kB
            } else {
                (1 << raw.rom_size) * 1024
            },
            sram_size: if raw.sram_size == 0 {
                0
            } else {
                (1 << raw.sram_size) * 1024
            },
        })
    }
}

#[derive(PackedStruct, Clone, Debug, Default, PartialEq, Eq)]
#[packed_struct(bit_numbering = "msb0", endian = "lsb")]
struct RawSnesHeader {
    name: [u8; 21],
    mapping: u8,
    chipset: u8,
    rom_size: u8,
    sram_size: u8,
    country: u8,
    developer_id: u8,
    version: u8,
    checksum_complement: u16,
    checksum: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example header of Final Fantasy 4 from
    // https://en.wikibooks.org/wiki/Super_NES_Programming/SNES_memory_map#Final_Fantasy_4
    static EXAMPLE_HEADER: &[u8] = &[
        0x46, 0x49, 0x4E, 0x41, 0x4C, 0x20, 0x46, 0x41, 0x4E, 0x54, 0x41, 0x53, 0x59, 0x20, 0x49,
        0x49, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x02, 0x0A, 0x03, 0x01, 0xC3, 0x00, 0x0F, 0x7A,
        0xF0, 0x85, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x02, 0xFF,
        0xFF, 0x04, 0x02, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0x00, 0x80, 0xFF, 0xFF,
    ];

    #[test]
    fn test_parse_header() {
        let header = SnesHeader::parse_header(EXAMPLE_HEADER).unwrap();
        assert_eq!(
            header,
            SnesHeader {
                name: "FINAL FANTASY II".to_string(),
                fast_rom: true,
                mapping_mode: MappingMode::LoRom,
                rom_size: 1024 * 1024,
                sram_size: 8 * 1024
            }
        )
    }
}
