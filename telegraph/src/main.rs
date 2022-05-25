use telegraph::api::{self};
use telegraph::cli::{self, Args};

use hyper::{client, header::CONTENT_TYPE, Body, Client, Method, Request, Response};

use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

const BOUNDARY: &str = "b968dw21w4a509b";
const UPLOAD_URL: &str = "https://telegra.ph/upload";

// token https://telegra.ph/api
// b968da509bb76866c35425099bc0989a5ec3b32997d55286c657e6994bbb
#[tokio::main]
async fn main() {
    let mut args: Args = Vec::new();

    if cli::parse_args(&mut args).is_err() {
        println!("err");
    }

    let mut text_vec: Vec<cli::Id> = Vec::new();
    let mut img_vec: Vec<cli::Id> = Vec::new();

    for f in args.into_iter() {
        if f.ty == api::ContentType::Img {
            img_vec.push(f);
        } else if f.ty == api::ContentType::Text {
            text_vec.push(f);
        }
    }
    //println!("{:#?}", &img_vec);
    //println!("{:#?}", &text_vec);
    //println!("{}", text_vec[0]);
    //println!("{}", img_vec[0]);

    //let args = std::env::args().collect::<Vec<_>>();

    let files: Vec<cli::Id> = upload_files(img_vec).await;

    let tg = PostBody::new(
        "b968da509bb76866c35425099bc0989a5ec3b32997d55286c657e6994bbb".to_string(),
        "title".to_string(),
        "author".to_string(),
        files,
        text_vec,
    );

    //println!("{:#?}", &tg);

    let res = tg.req_once().await;
    println!("{}", res);
}

fn cut_img_res(json: &str) -> String {
    // e.g. [{"src":"/file/xxxxxxxxx.png"}]
    let c = json.chars().collect::<Vec<char>>();
    let mut s = String::new();

    for f in &c[17..c.len() - 3] {
        s.push(*f);
    }

    s
}

async fn upload_files(img_vec: Vec<cli::Id>) -> Vec<cli::Id> {
    let https = hyper_tls::HttpsConnector::new(); // support https
    let client = client::Client::builder().build::<_, hyper::Body>(https);
    let mut run = vec![];

    // join_all
    for f in img_vec.iter() {
        run.push(parse_img_res(f, &client));
    }

    futures::future::join_all(run).await
}

async fn parse_img_res(
    img: &cli::Id,
    client: &Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
) -> cli::Id {
    let data = image_data(img.body.as_str()).unwrap();
    let req = Request::post(UPLOAD_URL)
        .header(
            CONTENT_TYPE,
            &*format!(r#"multipart/form-data; boundary="{}""#, BOUNDARY),
        )
        .body(data.into())
        .unwrap();

    let r = client.request(req).await.expect("");
    if let Ok(link) = res_body_to_string(r).await {
        // todo
        // parse json to str
        // e.g. [{"src":"\/file\/2e9b50292a7e5c4c17953.png"}]
        cli::Id {
            ty: img.ty,
            id: img.id,
            body: cut_img_res(&link),
        }
    } else {
        std::process::exit(1);
    }
}

fn display_body(ty: api::ContentType, body: &str) -> String {
    if ty == api::ContentType::Img {
        format!(
            r#"{{"tag":"img","attrs":{{"src":"https://telegra.ph/file/{img}"}}}}"#,
            img = body
        )
    } else if ty == api::ContentType::Text {
        format!(r#"{{"tag":"p","children":["{text}"]}}"#, text = body)
    } else {
        std::process::exit(1);
    }
}

fn image_data(path: &str) -> io::Result<Vec<u8>> {
    let mut data = Vec::new();
    let filename = get_filename(path).expect("");

    write!(data, "--{}\r\n", BOUNDARY)?;
    write!(
        data,
        r#"Content-Disposition: form-data; name="{f}"; filename="{f}"\r\n"#,
        f = filename
    )?;
    write!(data, "Content-Type: image/jpeg\r\n\r\n")?;

    let mut f = File::open(path)?;
    f.read_to_end(&mut data)?;

    write!(data, "\r\n--{}--\r\n", BOUNDARY)?;

    Ok(data)
}

fn get_file_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

fn get_filename(path: &str) -> Option<&str> {
    std::path::Path::new(path).file_name().expect("").to_str()
}

#[derive(Debug)]
pub struct PostBody {
    pub access_token: String,
    pub title: String,
    pub author_name: String,
    pub files: Vec<cli::Id>,
    pub text: Vec<cli::Id>,
}

impl PostBody {
    pub fn new(
        access_token: String,
        title: String,
        author_name: String,
        files: Vec<cli::Id>,
        text: Vec<cli::Id>,
    ) -> Self {
        Self {
            access_token,
            title,
            author_name,
            files,
            text,
        }
    }

    pub fn create_content(&self) -> String {
        let mut res = String::new();

        let mut all: Vec<cli::Id> = Vec::new();
        all.extend_from_slice(self.files.as_slice());
        all.extend_from_slice(self.text.as_slice());

        all.sort_by(|a, b| b.id.cmp(&a.id));

        //println!("{:#?}", &all);
        //println!("{:#?}", format!("{}", vec[0].1));
        res.push('[');
        for f in all.into_iter() {
            res.push_str(display_body(f.ty, f.body.as_str()).as_str());
            res.push(',');
        }
        res.push_str(r#"{"tag":"p","children":[" "]}"#);
        res.push(']');
        //println!("{:#?}", &res);

        res
    }

    pub async fn req_once(&self) -> String {
        let url = "https://api.telegra.ph/createPage";

        let body = format!(
            r#"{{
        "access_token":"{token}",
        "title":"{title}",
        "author_name":"{author_name}",
        "content":{content},
        "return_content":"true"
        }}"#,
            token = self.access_token,
            title = self.title,
            author_name = self.author_name,
            content = self.create_content()
        );

        // println!("{:#?}", body);

        req_once(url, body).await
    }
}

pub async fn req_once(url: &str, body: String) -> String {
    let https = hyper_tls::HttpsConnector::new(); // support https

    let client = client::Client::builder().build::<_, hyper::Body>(https);

    let req: Request<Body> = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("Content-Type", "application/json")
        //.header("Accept", "application/json")
        .header(
            "USER_AGENT",
            "Mozilla/5.0 (X11; Linux x86_64; rv:65.0) Gecko/20100101 Firefox/65.0",
        )
        .body(hyper::Body::from(body))
        .expect("Error: req_once()");

    res_body_to_string(client.request(req).await.unwrap())
        .await
        .unwrap()
}

pub async fn res_body_to_string(res: Response<Body>) -> Result<String, hyper::Error> {
    let bytes = hyper::body::to_bytes(res.into_body()).await?;
    Ok(String::from_utf8(bytes.to_vec()).expect("response was not valid utf-8"))
}
