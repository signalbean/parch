mod api;
mod cli;
mod download;
mod local;
mod paths;
mod wallpaper;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = cli::parse()?;

    let path = if args.local {
        local::get_random(args.verbose)?
    } else {
        let post = api::fetch(args.id, args.verbose)?;
        let url = api::image_url(&post)?;
        download::save(post.id, &url, args.verbose)?
    };

    wallpaper::set(&path, args.verbose)?;
    println!("✓ Applied");
    Ok(())
}
