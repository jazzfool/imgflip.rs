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
#[serde(untagged)]
enum Response<T> {
    SuccessResponse {
        success: bool,
        data: T,
    },
    FailureResponse {
        success: bool,
        error_message: String,
    },
}

struct ImageCaptionRequest {
    template_id: String,
    username: String,
    password: String,
    text0: String,
    text1: String,
}

#[derive(Debug, Deserialize)]
struct CaptionImageResponse {
	url: Url,
	page_url: Url,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let memes: Response<MemeTemplatesData> = reqwest::Client::new()
        .get("https://api.imgflip.com/get_memes")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", memes);

    let meme_caption = ImageCaptionRequest {
        template_id: "61580".into(),
        username: "freeforall6".into(),
        password: "nsfw1234".into(),
        text0: "text0".into(),
        text1: "text1".into(),
    };
    let meme: Response<CaptionImageResponse> = reqwest::Client::new()
        .post("https://api.imgflip.com/caption_image")
        .query(&[
            ("template_id", meme_caption.template_id),
            ("username", meme_caption.username),
            ("password", meme_caption.password),
            ("text0", meme_caption.text0),
            ("text1", meme_caption.text1),
        ])
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", meme);

    Ok(())
}
