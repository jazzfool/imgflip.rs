use serde::{Deserialize, Serialize};
use url::Url;
use reqwest::header::{HeaderValue, CONTENT_TYPE};

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

#[derive(Debug, Serialize)]
enum CaptionFont {
    Impact,
    Arial,
}

#[derive(Debug, Serialize)]
struct TopBottomCaptionRequest {
    template_id: String,
    username: String,
    password: String,
    text_top: String,
    text_bottom: String,
    font: Option<CaptionFont>,
    max_font_size: u32,
}

#[derive(Debug, Serialize)]
struct CaptionBox {
    text: String,
    x: Option<u32>,
    y: Option<u32>,
    width: Option<u32>,
    height: Option<u32>,
    color: Option<String>,
    outline_color: Option<String>,
}

#[derive(Debug, Serialize)]
struct CaptionBoxesRequest {
    template_id: String,
    username: String,
    password: String,
    font: Option<CaptionFont>,
    max_font_size: Option<u32>,
    boxes: Vec<CaptionBox>,
}

enum ImageCaptionRequest {
    TopBottomCaptionRequest(TopBottomCaptionRequest),
    CaptionBoxesRequest(CaptionBoxesRequest),
}

#[derive(Debug, Deserialize)]
struct CaptionImageResponse {
    url: Url,
    page_url: Url,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    /*
        let memes: Response<MemeTemplatesData> = reqwest::Client::new()
            .get("https://api.imgflip.com/get_memes")
            .send()
            .await?
            .json()
            .await?;

        println!("{:#?}", memes);
    */
    let meme_caption = CaptionBoxesRequest {
        template_id: "61580".into(),
        username: "freeforall6".into(),
        password: "nsfw1234".into(),
        font: None,
        max_font_size: None,
        boxes: vec![
            CaptionBox {
                text: "text0".into(),
                x: None,
                y: None,
                width: None,
                height: None,
                color: None,
                outline_color: None,
            },
            CaptionBox {
                text: "text1".into(),
                x: None,
                y: None,
                width: None,
                height: None,
                color: None,
                outline_color: None,
            },
            CaptionBox {
                text: "text2".into(),
                x: None,
                y: None,
                width: None,
                height: None,
                color: None,
                outline_color: None,
            },
        ],
    };
    let meme: Response<CaptionImageResponse> = reqwest::Client::new()
        .post("https://api.imgflip.com/caption_image")
		.header(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"))
		.body(serde_qs::to_string(&meme_caption).unwrap())
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", meme);

    Ok(())
}
