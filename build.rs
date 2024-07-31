use chrono;
use chrono::FixedOffset;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_info.rs");
    let utc_date = chrono::Utc::now();
    let offset = FixedOffset::east_opt(8 * 3600).unwrap();
    let build_date = utc_date
        .with_timezone(&offset)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();

    fs::write(
        &dest_path,
        format!("pub const BUILD_DATE: &str = \"{}\";", build_date),
    )
    .unwrap();
}
