use serde::{Deserialize, Serialize};
use worker::fetch_with_request;
use worker::fetch_with_str;
use worker::stub_fetch_with_req;
use worker::CfEnv;
use worker::{kv::KvStore, prelude::*};

mod utils;

#[derive(Deserialize, Serialize)]
struct MyData {
    message: String,
    #[serde(default)]
    is: bool,
    #[serde(default)]
    data: Vec<u8>,
}

#[cf::worker(fetch)]
pub async fn main(mut req: Request, env: CfEnv) -> Result<Response> {
    utils::set_panic_hook();

    match (req.method(), req.path().as_str()) {
        (Method::Get, "/") => {
            let msg = format!(
                "[rustwasm] event type: {}, colo: {}, asn: {}",
                req.event_type(),
                req.cf().colo(),
                req.cf().asn(),
            );
            Response::ok(Some(msg))
        }
        (Method::Post, "/") => {
            let data: MyData = req.json().await?;
            Response::ok(Some(format!("[POST /] message = {}", data.message)))
        }
        (Method::Post, "/read-text") => Response::ok(Some(format!(
            "[POST /read-text] text = {}",
            req.text().await?
        ))),
        (_, "/json") => Response::json(&MyData {
            message: "hello!".into(),
            is: true,
            data: vec![1, 2, 3, 4, 5],
        }),
        (Method::Post, "/job") => {
            let kv = KvStore::create("JOB_LOG").expect("no binding for JOB_LOG");
            if kv
                .put("manual entry", 123)
                .expect("fail to build KV put operation")
                .execute()
                .await
                .is_err()
            {
                Response::error("Failed to put into KV".into(), 500)
            } else {
                Response::empty()
            }
        }
        (_, "/jobs") => {
            if let Ok(kv) = KvStore::create("JOB_LOG") {
                return match kv.list().execute().await {
                    Ok(jobs) => Response::json(&jobs),
                    Err(e) => Response::error(format!("KV list error: {:?}", e), 500),
                };
            }
            Response::error("Failed to access KV binding".into(), 500)
        }
        (_, "/fetch") => {
            let resp = fetch_with_str("https://example.com/test.txt").await;
            match resp {
                Ok(r) => {
                    if r.status_code() == 200 {
                        Response::ok(Some("test".into()))
                    } else {
                        Response::error("failed".into(), r.status_code())
                    }
                }
                Err(_e) => Response::error("failed".into(), 500),
            }
        }
        (_, "/fetch_with_req") => {
            let worker_req = Request::new("https://example.com/test.txt");
            let resp = fetch_with_request(&worker_req).await;
            match resp {
                Ok(r) => {
                    if r.status_code() == 200 && r.edge_response().is_some() {
                        Response::ok(Some("fetch_with_req success!".into()))
                    } else {
                        Response::error("fetch_with_req failed!".into(), r.status_code())
                    }
                }
                Err(_e) => Response::error("failed".into(), 500),
            }
        }
        (_, "/fetch_with_durable_object") => {
            let counter = env.counter();
            let id = counter.id_from_name("A");
            //let str_id: String = id.to_string().into();
            let stub = counter.get(&id);
            let worker_req = Request::new("https://example.com/increment");
            let resp = stub_fetch_with_req(&stub, &worker_req).await;
            match resp {
                Ok(r) => {
                    if r.status_code() == 200 {
                        Response::ok(r.body().map(String::from))
                    } else {
                        Response::error(
                            format!("Request failed. Response: {:?}", r),
                            r.status_code(),
                        )
                    }
                }
                Err(e) => Response::error(format!("Durable Object error: {:?}", e), 500),
            }
            //Response::ok(Some(format!("{:?} {}", req.method(), req.path())))
        }
        (_, "/404") => Response::error("Not Found".to_string(), 404),
        _ => Response::ok(Some(format!("{:?} {}", req.method(), req.path()))),
    }
}

#[cf::worker(scheduled)]
pub async fn job(s: Schedule) -> Result<()> {
    utils::set_panic_hook();

    let kv = KvStore::create("JOB_LOG").expect("no binding for JOB_LOG");
    kv.put(&format!("{}", s.time()), s)
        .expect("fail to build KV put operation")
        .execute()
        .await
        .map_err(worker::Error::from)

    // s.time() = 1621579157181, s.cron() = "15 * * * *", s.event_type() == "scheduled";
}
