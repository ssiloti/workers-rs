use crate::Error;
use crate::Request as workerRequest;
use crate::Response as workerResponse;
use edgeworker_sys::{Response as EdgeResponse, WorkerGlobalScope};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

pub async fn fetch_with_str(url: &str) -> Result<workerResponse, Error> {
    let worker: WorkerGlobalScope = js_sys::global().unchecked_into();

    let promise = worker.fetch_with_str(url);
    let result = JsFuture::from(promise).await;
    match result {
        Ok(resp) => {
            let resp: EdgeResponse = resp.dyn_into().unwrap();
            let worker_resp = workerResponse::from(resp);
            Ok(worker_resp)
        }
        Err(e) => Err(Error::from(e)),
    }
}

pub async fn fetch_with_request(request: &workerRequest) -> Result<workerResponse, Error> {
    let worker: WorkerGlobalScope = js_sys::global().unchecked_into();
    let req = &request.edge_request;
    let promise = worker.fetch_with_request(req);
    let result = JsFuture::from(promise).await;
    match result {
        Ok(resp) => {
            let resp: EdgeResponse = resp.dyn_into().unwrap();
            let worker_resp = workerResponse::from(resp);
            Ok(worker_resp)
        }
        Err(e) => Err(Error::from(e)),
    }
}
