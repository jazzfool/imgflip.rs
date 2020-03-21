use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
struct MemeTemplate {
    id: String,
    name: String,
    url: Url,
    width: u32,
    height: u32,
    box_count: u32,
}

#[derive(Debug, Deserialize)]
struct MemeTemplatesData {
    memes: Vec<MemeTemplate>,
}

#[derive(Debug, Deserialize)]
struct Response {
    success: bool,
    data: MemeTemplatesData,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let memes: Response = reqwest::Client::new()
        .get("https://api.imgflip.com/get_memes")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", memes);

    Ok(())
}
