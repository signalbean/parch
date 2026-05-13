use crate::paths::parch_dir;
use std::fs::{File, create_dir_all};
use std::io::copy;
use std::path::PathBuf;

pub fn save(id: u64, url: &str, verbose: bool) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let dir = parch_dir()?;
    create_dir_all(&dir)?;

    let ext = extract_extension(url);
    let dest = dir.join(format!("{}.{}", id, ext));

    if verbose {
        println!(
            "→ Downloading from: {}\n→ Saving to: {}",
            url,
            dest.display()
        );
    }

    let response = ureq::get(url).header("User-Agent", "parch").call()?;

    let mut file = File::create(&dest)?;
    let bytes = copy(&mut response.into_body().into_reader(), &mut file)?;

    if bytes == 0 {
        return Err("Downloaded file is empty".into());
    }

    if verbose {
        println!("→ Downloaded {} bytes", bytes);
    }

    dest.canonicalize().or_else(|_| Ok(dest))
}

fn extract_extension(url: &str) -> &str {
    url.rsplit('/')
        .next()
        .and_then(|filename| filename.split('?').next())
        .and_then(|filename| filename.rsplit('.').next())
        .filter(|ext| !ext.is_empty())
        .unwrap_or("jpg")
}
