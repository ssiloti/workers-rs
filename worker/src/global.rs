use crate::Error;
use crate::Request as workerRequest;
use crate::Response as workerResponse;
use edgeworker_sys::DurableObjectStub;
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

pub async fn stub_fetch_with_str(
    stub: &DurableObjectStub,
    url: &str,
) -> Result<workerResponse, Error> {
    let promise = stub.fetch_with_str(url);
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

pub async fn stub_fetch_with_req(
    stub: &DurableObjectStub,
    request: &workerRequest,
) -> Result<workerResponse, Error> {
    let req = &request.edge_request;
    let promise = stub.fetch_with_request(req);
    let result = JsFuture::from(promise)
        .await
        .expect("what error would it be?");
    let worker_resp = workerResponse::ok(Some("hello world".to_string()))?; //workerResponse::from(resp);
    Ok(worker_resp)
    /*match result {
        Ok(resp) => {
            //let resp: EdgeResponse = resp.dyn_into().unwrap();
            let worker_resp = workerResponse::ok(Some("hello world".to_string()))?; //workerResponse::from(resp);
            Ok(worker_resp)
        }
        Err(e) => Err(Error::from(e)),
    }*/

    /*let resp: workerResponse = result?.dyn_into().expect_throw(message);
    //let resp = EdgeResponse::new_with_opt_str(Some(format!("Count: {}", 5).as_str())).unwrap();

    //et status: u16 = 5;
    let resp: EdgeResponse =
        EdgeResponse::new_with_opt_str(Some(format!("Count: {}", "zaidoon").as_str())).unwrap();

    let worker_resp = workerResponse::from(resp);
    Ok(worker_resp)*/
    //let resp = workerResponse::ok(Some(status.to_string()));
    //resp
    /* match result {
        Ok(resp) => {
            let resp: EdgeResponse = resp.dyn_into().unwrap();
            let resp: EdgeResponse =
                EdgeResponse::new_with_opt_str(Some(format!("Count: {}", "zaidoon").as_str()))
                    .unwrap();

            let worker_resp = workerResponse::from(resp);
            Ok(worker_resp)
        }
        Err(e) => Err(Error::from(e)),
    }*/
}
