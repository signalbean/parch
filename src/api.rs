use serde::Deserialize;

const BASE_URL: &str = "https://konachan.net/post.json";

#[derive(Deserialize)]
pub struct Post {
    pub id: u64,
    pub file_url: Option<String>,
    pub large_file_url: Option<String>,
}

pub fn fetch(
    id: Option<u64>,
    verbose: bool,
) -> Result<Post, Box<dyn std::error::Error>> {
    let url = build_url(id);

    if verbose {
        match id {
            Some(i) => println!("→ Fetching post by ID {}", i),
            None => println!("→ Fetching random post"),
        }
    }

    let response = ureq::get(&url).header("User-Agent", "parch").call()?;

    let body = response.into_body().read_to_vec()?;
    let posts: Vec<Post> = serde_json::from_slice(&body)?;
    posts.into_iter().next().ok_or("No post found".into())
}

pub fn image_url(post: &Post) -> Result<String, Box<dyn std::error::Error>> {
    let url = post
        .file_url
        .as_deref()
        .or(post.large_file_url.as_deref())
        .ok_or("No image URL available")?;

    Ok(normalize_url(url))
}

fn build_url(id: Option<u64>) -> String {
    match id {
        Some(i) => format!("{}?tags=id%3A{}", BASE_URL, i),
        None => format!(
            "{}?tags=rating%3Asafe%20order%3Arandom&limit=1",
            BASE_URL
        ),
    }
}

fn normalize_url(url: &str) -> String {
    if url.starts_with("//") {
        format!("https:{}", url)
    } else if !url.starts_with("http") {
        format!("https://{}", url)
    } else {
        url.to_string()
    }
}
