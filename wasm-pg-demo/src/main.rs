use serde::Serialize;
use serde_json::Value;
use std::env;
use tokio_postgres::{NoTls, Error};

fn get_url() -> String {
    if let Ok(url) = std::env::var("DATABASE_URL") {
        url
    } else {
        "postgres://postgres:postgres@localhost/postgres".into()
    }
}

#[derive(Serialize)]
struct ResContext {
    headers_authorization: String,
    body: String,
}

#[derive(Serialize)]
struct Body {
    status: String,
    message: String,
    data: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(&*get_url(), NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let rows = client.query("SELECT * FROM users;", &[]).await?;
    // println!("authorization {}", rows.iter().next().unwrap().get::<usize, String>(1));
    let authorization_lyy = rows.iter().next().unwrap().get::<usize, String>(1);
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let json_str = &args[1];
    match serde_json::from_str::<Value>(json_str) {
        Ok(v) => {
            let authorization = v["headers"].as_object().unwrap()["authorization"].as_str().unwrap();
            if authorization == authorization_lyy {
                let body = Body {
                    status: "success".to_string(),
                    message: "authorization success".to_string(),
                    data: "Hello world!".to_string(),
                };
                let res = ResContext {
                    headers_authorization: "authorization success".to_string(),
                    body: serde_json::to_string(&body).unwrap(),
                };
                let res_json = serde_json::to_string(&res).unwrap();
                println!("{}", res_json);
            } else {
                let body = Body {
                    status: "fail".to_string(),
                    message: format!("authorization fail, authorization value: {}", authorization),
                    data: "".to_string(),
                };
                let res = ResContext {
                    headers_authorization: "authorization fail".to_string(),
                    body: serde_json::to_string(&body).unwrap(),
                };
                let res_json = serde_json::to_string(&res).unwrap();
                println!("{}", res_json);
            }
        }
        Err(e) => {
            eprintln!("Failed to parse json: {}", e);
        }
    }
    Ok(())
}