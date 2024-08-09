use std::ffi::CString;

use serde::Serialize;
use serde_json::Value;
use tokio_postgres::{Error, NoTls};

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

fn get_url() -> String {
    if let Ok(url) = std::env::var("DATABASE_URL") {
        url
    } else {
        "postgres://postgres:postgres@localhost/postgres".into()
    }
}

fn get_user_authorization() -> Result<String, Error> {
    let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let authorization = runtime.block_on(async {
        let (client, connection) = tokio_postgres::connect(&*get_url(), NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let rows = client.query("SELECT * FROM users;", &[]).await?;
        let authorization = rows.iter().next().unwrap().get::<usize, String>(1);

        Ok(authorization)
    })?;

    Ok(authorization)
}

#[no_mangle]
pub extern "C" fn plugin_name(input: *const u8, len: usize) -> *const u32 {
    if input.is_null() || len == 0 {
        return std::ptr::null_mut();
    }

    let slice = unsafe { std::slice::from_raw_parts(input, len) };

    let string = match std::str::from_utf8(slice) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let authorization = match get_user_authorization() {
        Ok(auth) => auth,
        Err(_) => "error".to_owned(),
    };

    let res_json = match serde_json::from_str::<Value>(string) {
        Ok(v) => {
            let request_authorization = v["headers"].as_object().unwrap()["authorization"]
                .as_str()
                .unwrap();
            if request_authorization == authorization {
                let body = Body {
                    status: "success".to_string(),
                    message: "authorization success".to_string(),
                    data: "Hello world!".to_string(),
                };
                let res = ResContext {
                    headers_authorization: "authorization success".to_string(),
                    body: serde_json::to_string(&body).unwrap(),
                };
                serde_json::to_string(&res).unwrap()
            } else {
                let body = Body {
                    status: "fail".to_string(),
                    message: format!(
                        "authorization fail, authorization value: {}",
                        request_authorization
                    ),
                    data: "".to_string(),
                };
                let res = ResContext {
                    headers_authorization: "authorization fail".to_string(),
                    body: serde_json::to_string(&body).unwrap(),
                };
                serde_json::to_string(&res).unwrap()
            }
        }
        Err(_) => "not a valid json string".to_owned(),
    };

    let cstring = match CString::new(res_json) {
        Ok(cstr) => cstr,
        Err(_) => return std::ptr::null_mut(),
    };

    let s_len = cstring.to_bytes().len() as u32;
    let s_ptr = cstring.into_raw() as u32;

    let data: [u32; 2] = [s_ptr as u32, s_len as u32];

    Box::into_raw(Box::new(data)) as *const u32
}
