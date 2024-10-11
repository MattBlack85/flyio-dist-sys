use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Deserialize, Serialize)]
struct Body {
    r#type: String,
    msg_id: i32,
    echo: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Request {
    src: Option<String>,
    dest: Option<String>,
    body: Body,
}

#[derive(Debug, Deserialize, Serialize)]
struct ResponseBody {
    r#type: String,
    msg_id: i32,
    in_reply_to: i32,
    echo: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    src: Option<String>,
    dest: Option<String>,
    body: ResponseBody,
}

impl From<Request> for Response {
    fn from(r: Request) -> Self {
        Self {
            src: if r.src.is_some() {
                Some(r.src.clone().unwrap())
            } else {
                None
            },
            dest: if r.src.is_some() {
                Some(r.src.clone().unwrap())
            } else {
                None
            },
            body: ResponseBody {
                r#type: format!("{}_ok", r.body.r#type.clone()),
                msg_id: r.body.msg_id,
                in_reply_to: r.body.msg_id,
                echo: if r.body.echo.is_some() {
                    Some(r.body.echo.clone().unwrap())
                } else {
                    None
                },
            },
        }
    }
}

fn main() -> io::Result<()> {
    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer)?;
        let r = serde_json::from_str::<Request>(&buffer)?;
        let resp: Response = Response::from(r);
        let resp_str = serde_json::to_string(&resp)?;
        println!("{}", format_args!("{}", &resp_str));
    }
}
