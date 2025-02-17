use std::env;
use std::fs;
use std::path::Path;

fn embed_roms_from_dir(dir_path: &Path) -> String {
    let mut rom_code = String::new();
    
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension() == Some("xz".as_ref()) {
                    let name = path.file_name().unwrap().to_str().unwrap();
                    rom_code.push_str(&format!(
r#"    RomFile {{
        name: "{}",
        data: include_bytes!({:?}),
    }},
"#,
                        &name[..(name.len() - ".sfc.xz".len())],
                        path
                    ));
                }
            }
        }
    }
    rom_code
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("rom_files.rs");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let roms_dir = Path::new(&manifest_dir).join("roms");
    let games_dir = roms_dir.join("games");
    let programs_dir = roms_dir.join("programs");
    
    // Generate the ROM embedding code
    let mut rom_code = String::new();
    
    // Define the RomFile struct
    rom_code.push_str("pub struct RomFile {\n");
    rom_code.push_str("    pub name: &'static str,\n");
    rom_code.push_str("    pub data: &'static [u8],\n");
    rom_code.push_str("}\n\n");
    
    // Embed games
    rom_code.push_str("pub const EMBEDDED_GAMES: &[RomFile] = &[\n");
    rom_code.push_str(&embed_roms_from_dir(&games_dir));
    rom_code.push_str("];\n\n");
    
    // Embed programs
    rom_code.push_str("pub const EMBEDDED_PROGRAMS: &[RomFile] = &[\n");
    rom_code.push_str(&embed_roms_from_dir(&programs_dir));
    rom_code.push_str("];\n");

    fs::write(&dest_path, rom_code).unwrap();
    
    // Make sure to rebuild if any ROM files change
    println!("cargo:rerun-if-changed=roms/games");
    println!("cargo:rerun-if-changed=roms/programs");
}