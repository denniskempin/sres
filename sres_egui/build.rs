use std::env;
use std::fs;
use std::path::Path;

/// Extract rom name: path/to/_FooBar.sfc -> FooBar
fn rom_name(path: &Path) -> &str {
    let name = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .strip_suffix(".sfc")
        .unwrap();
    if let Some(stripped) = name.strip_prefix("_") {
        stripped
    } else {
        name
    }
}

fn rom_image(path: &Path) -> String {
    let image_path = path.with_extension("png");
    if image_path.exists() {
        format!("include_bytes!({:?})", image_path)
    } else {
        "DEFAULT_IMAGE".to_string()
    }
}

fn rom_attribution(path: &Path) -> String {
    let attribution_path = path.with_extension("txt");
    if attribution_path.exists() {
        let content = fs::read_to_string(&attribution_path).unwrap();
        let mut lines = content.lines();
        let author = lines.next().unwrap_or("").trim();
        let url = lines.next().unwrap_or("").trim();
        format!("Some(({:?}, {:?}))", author, url)
    } else {
        "None".to_string()
    }
}

fn create_rom_file_info(dir_path: &Path) -> String {
    let mut rom_code = String::new();
    for entry in fs::read_dir(dir_path).unwrap() {
        let path = entry.unwrap().path();
        if path.extension() == Some("sfc".as_ref()) {
            rom_code.push_str(&format!(
                r#"
                RomFileInfo {{
                    name: "{}",
                    image: {},
                    attribution: {},
                    path: {:?},

                }},"#,
                &rom_name(&path),
                &rom_image(&path),
                &rom_attribution(&path),
                path.strip_prefix(dir_path.parent().unwrap()).unwrap(),
            ));
        }
    }
    rom_code
}

fn create_category_info(dir_path: &Path) -> String {
    format!(
        r#"
    CategoryInfo {{
        name: "{}",
        roms: &[{}]
    }},"#,
        dir_path.file_name().unwrap().to_str().unwrap(),
        create_rom_file_info(dir_path)
    )
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("embedded_roms_generated.rs");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let roms_dir = Path::new(&manifest_dir).join("roms");

    // Generate the ROM embedding code. Each directory is a category with multiple roms inside.
    let mut rom_code = String::new();
    rom_code.push_str("&[");
    for entry in fs::read_dir(roms_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rom_code.push_str(&create_category_info(&path));
        }
    }
    rom_code.push_str("\n]");
    fs::write(&dest_path, rom_code).unwrap();

    // Make sure to rebuild if any ROM files change
    println!("cargo:rerun-if-changed=roms");
}
