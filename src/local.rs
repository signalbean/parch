use crate::paths::parch_dir;
use std::fs;
use std::path::{Path, PathBuf};

const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png"];

pub fn get_random(verbose: bool) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let dir = parch_dir()?;

    if verbose {
        println!("→ Scanning directory: {}", dir.display());
    }

    if !dir.exists() {
        return Err(format!("Directory not found: {}", dir.display()).into());
    }

    let images = collect_images(&dir)?;

    if images.is_empty() {
        return Err(format!(
            "No wallpapers found in {}. Download some first with 'parch id <id>'",
            dir.display()
        )
        .into());
    }

    if verbose {
        println!("→ Found {} wallpaper(s)", images.len());
    }

    let selected = select_random_image(images);

    if verbose {
        println!("→ Selected: {}", selected.display());
    }

    Ok(selected)
}

fn collect_images(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut images = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file()
            && let Some(extension) = path.extension()
            && let Some(ext_str) = extension.to_str()
            && IMAGE_EXTENSIONS.contains(&ext_str.to_lowercase().as_str())
        {
            images.push(path);
        }
    }

    Ok(images)
}

fn select_random_image(images: Vec<PathBuf>) -> PathBuf {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    // Simple pseudo-random selection using system time
    let mut hasher = DefaultHasher::new();
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
        .hash(&mut hasher);

    let index = (hasher.finish() as usize) % images.len();
    images[index].clone()
}
