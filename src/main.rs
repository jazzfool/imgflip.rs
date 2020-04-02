use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
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

#[derive(Debug)]
pub enum ErrorKind {
    Reqwest(reqwest::Error),
    ApiError(String),
}

#[derive(Debug)]
pub struct Error(ErrorKind);

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "todo")
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self(ErrorKind::Reqwest(e))
    }
}

pub type Result<T> = std::result::Result<T, Error>;

fn err_for_failure<T>(response: Response<T>) -> Result<T> {
    match response {
        Response::SuccessResponse { data, .. } => Ok(data),
        Response::FailureResponse { error_message, .. } => {
            Err(Error(ErrorKind::ApiError(error_message)))
        }
    }
}

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Client {
            client: reqwest::Client::new(),
        }
    }

    pub async fn memes(&self) -> Result<Vec<MemeTemplate>> {
        let result = self
            .client
            .get("https://api.imgflip.com/get_memes")
            .send()
            .await?
            .json::<Response<MemeTemplatesData>>()
            .await?;
        err_for_failure(result).map(|r| r.memes)
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let memes = Client::new().memes().await?;
    println!("{:#?}", memes);

    /*
        let memes: Response<MemeTemplatesData> = reqwest::Client::new()
            .get("https://api.imgflip.com/get_memes")
            .send()
            .await?
            .json()
            .await?;

        println!("{:#?}", memes);
    */
    /*
    let meme_caption = CaptionBoxesRequest {
        template_id: "61580".into(),
        username: "freeforall6".into(),
        password: "nsfw1234".into(),
        font: Some(CaptionFont::Arial),
        max_font_size: Some(42),
        boxes: vec![
            CaptionBox {
                text: "".into(),
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
        ],
    };
    println!("{}", serde_qs::to_string(&meme_caption).unwrap());
    let meme: Response<CaptionImageResponse> = reqwest::Client::new()
        .post("https://api.imgflip.com/caption_image")
        .header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        )
        .body(serde_qs::to_string(&meme_caption).unwrap())
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", meme);
    */
    Ok(())
}
