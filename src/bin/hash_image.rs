//! Hash an image and print the Base64 value

use std::env;
use std::fmt::Write;

use image_hasher::HasherConfig;

fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 2);

    let image = image::open(&args[1]).map_err(|e| format!("failed to open {}: {}", &args[1], e))?;

    let hash = HasherConfig::new()
        .hash_size(8, 8)
        .to_hasher()
        .hash_image(&image);

    let hash_str = hash.as_bytes().iter().fold(String::new(), |mut acc, &b| {
        write!(acc, "{b:02x}").expect("Failed to write to String");
        acc
    });

    println!("{}: {}", &args[1], hash_str);

    Ok(())
}
