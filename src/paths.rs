use std::path::PathBuf;

pub fn parch_dir() -> Result<PathBuf, String> {
    let mut path = if cfg!(windows) {
        PathBuf::from(
            std::env::var("USERPROFILE")
                .or_else(|_| {
                    std::env::var("HOMEDRIVE")
                        .and_then(|d| std::env::var("HOMEPATH").map(|p| format!("{}{}", d, p)))
                })
                .map_err(|_| "Directory not found")?,
        )
    } else {
        PathBuf::from(std::env::var("HOME").map_err(|_| "HOME not set")?)
    };

    path.push("Pictures");
    path.push("Parch");

    Ok(path)
}
