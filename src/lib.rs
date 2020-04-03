use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize)]
pub struct MemeTemplate {
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

impl<T> Response<T> {
    fn convert(self) -> Result<T> {
        match self {
            Response::SuccessResponse { data, .. } => Ok(data),
            Response::FailureResponse { error_message, .. } => {
                Err(Error(ErrorKind::ApiError(error_message)))
            }
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CaptionFont {
    Impact,
    Arial,
}

#[derive(Debug, Serialize)]
pub struct CommonCaptionRequest {
    pub template_id: String,
    pub font: Option<CaptionFont>,
    pub max_font_size: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct TopBottomCaptionRequest {
    #[serde(flatten)]
    common: CommonCaptionRequest,
    text_top: String,
    text_bottom: String,
}

#[derive(Debug, Serialize)]
pub struct CaptionBox {
    pub text: String,
    pub x: Option<u32>,
    pub y: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub color: Option<String>,
    pub outline_color: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CaptionBoxesRequest {
    #[serde(flatten)]
    pub common: CommonCaptionRequest,
    pub boxes: Vec<CaptionBox>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ImageCaptionRequest {
    TopBottomCaptionRequest(TopBottomCaptionRequest),
    CaptionBoxesRequest(CaptionBoxesRequest),
}

#[derive(Debug, Deserialize)]
pub struct CaptionImageResponse {
    url: Url,
    page_url: Url,
}

#[derive(Debug)]
pub enum ErrorKind {
    Reqwest(reqwest::Error),
    SerdeQs(serde_qs::Error),
    ApiError(String),
}

#[derive(Debug, Serialize)]
pub struct RequestAuthWrapper<T> {
    #[serde(flatten)]
    request: T,
    username: String,
    password: String,
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

impl From<serde_qs::Error> for Error {
    fn from(e: serde_qs::Error) -> Self {
        Self(ErrorKind::SerdeQs(e))
    }
}

pub type Result<T> = std::result::Result<T, Error>;

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
        self.client
            .get("https://api.imgflip.com/get_memes")
            .send()
            .await?
            .error_for_status()?
            .json::<Response<MemeTemplatesData>>()
            .await?
            .convert()
            .map(|r| r.memes)
    }
}

pub struct AccountClient {
    username: String,
    password: String,
    client: reqwest::Client,
}

impl AccountClient {
    pub fn new(username: String, password: String) -> Self {
        AccountClient {
            client: reqwest::Client::new(),
            username,
            password,
        }
    }

    pub async fn caption_image(
        &self,
        image_caption: ImageCaptionRequest,
    ) -> Result<CaptionImageResponse> {
        self.client
            .post("https://api.imgflip.com/caption_image")
            .header(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            )
            .body(serde_qs::to_string(&RequestAuthWrapper {
                request: image_caption,
                username: self.username.clone(),
                password: self.password.clone(),
            })?)
            .send()
            .await?
            .error_for_status()?
            .json::<Response<CaptionImageResponse>>()
            .await?
            .convert()
    }
}
