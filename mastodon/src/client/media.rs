use hyper::{client, header::CONTENT_TYPE, Body, Client, Method, Request, Response};

use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

const BOUNDARY: &str = "b968dw21w4a509b";

#[derive(Debug, Clone)]
pub struct Id {
    pub ty: ContentType,
    pub id: usize,
    pub body: String,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ContentType {
    Img,
    Text,
}

pub async fn upload_files(
    client: &Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
    files: &[&str],
    to: &str,
) -> Vec<Id> {
    let mut run = vec![];

    // join_all
    for f in files.iter() {
        run.push(parse_img_res(client, f));
    }

    futures::future::join_all(run).await
}

pub async fn parse_img_res(
    client: &Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
    path: &str,
    uri: &str,
) -> Id {
    let data = image_data(path).expect("");

    // TODO
    // upload files
    let req = Request::post(url)
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

fn get_filename(path: &str) -> Option<&str> {
    std::path::Path::new(path).file_name().expect("").to_str()
}
