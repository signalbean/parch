use std::path::PathBuf;

pub fn parch_dir() -> Result<PathBuf, String> {
    let home = if cfg!(windows) {
        std::env::var("USERPROFILE")
            .or_else(|_| {
                std::env::var("HOMEDRIVE")
                    .and_then(|d| std::env::var("HOMEPATH").map(|p| format!("{}{}", d, p)))
            })
            .map_err(|_| "Home directory not found")?
    } else {
        std::env::var("HOME").map_err(|_| "HOME not set")?
    };

    let mut path = PathBuf::from(home);
    path.push("Pictures");
    path.push("Parch");

    Ok(path)
}
