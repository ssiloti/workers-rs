use std::collections::HashMap;

use js_sys::{JsString, Promise};
use wasm_bindgen::prelude::*;

use crate::Request;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=::js_sys::Object, js_name=IncomingRequestCfProperties)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Cf;

    #[wasm_bindgen(structural, method, getter, js_class=IncomingRequestCfProperties, js_name=colo)]
    pub fn colo(this: &Cf) -> String;

    #[wasm_bindgen(structural, method, getter, js_class=IncomingRequestCfProperties, js_name=asn)]
    pub fn asn(this: &Cf) -> u32;

    #[wasm_bindgen(structural, method, getter, js_class=IncomingRequestCfProperties, js_name=country)]
    pub fn country(this: &Cf) -> Option<String>;

    #[wasm_bindgen(structural, method, getter, js_class=IncomingRequestCfProperties, js_name=httpProtocol)]
    pub fn http_protocol(this: &Cf) -> String;

    #[wasm_bindgen(structural, method, getter, js_class=IncomingRequestCfProperties, js_name=requestPriority)]
    pub fn request_priority(this: &Cf) -> Option<String>;

    #[wasm_bindgen(structural, method, getter, js_class=IncomingRequestCfProperties, js_name=tlsClientAuth)]
    pub fn tls_client_auth(this: &Cf) -> TlsClientAuth;

    #[wasm_bindgen(structural, method, getter, js_class=IncomingRequestCfProperties, js_name=tlsCipher)]
    pub fn tls_cipher(this: &Cf) -> String;

    #[wasm_bindgen(structural, method, getter, js_class=IncomingRequestCfProperties, js_name=tlsVersion)]
    pub fn tls_version(this: &Cf) -> String;

    #[wasm_bindgen(structural, method, getter, js_class=IncomingRequestCfProperties, js_name=city)]
    pub fn city(this: &Cf) -> Option<String>;

    #[wasm_bindgen(structural, method, getter, js_class=IncomingRequestCfProperties, js_name=continent)]
    pub fn continent(this: &Cf) -> Option<String>;

    #[wasm_bindgen(structural, method, getter, js_class=IncomingRequestCfProperties, js_name=latitude)]
    pub fn latitude(this: &Cf) -> Option<String>;

    #[wasm_bindgen(structural, method, getter, js_class=IncomingRequestCfProperties, js_name=longitude)]
    pub fn longitude(this: &Cf) -> Option<String>;

    #[wasm_bindgen(structural, method, getter, js_class=IncomingRequestCfProperties, js_name=postalCode)]
    pub fn postal_code(this: &Cf) -> Option<String>;

    #[wasm_bindgen(structural, method, getter, js_class=IncomingRequestCfProperties, js_name=metroCode)]
    pub fn metro_code(this: &Cf) -> Option<String>;

    #[wasm_bindgen(structural, method, getter, js_class=IncomingRequestCfProperties, js_name=region)]
    pub fn region(this: &Cf) -> Option<String>;

    #[wasm_bindgen(structural, method, getter, js_class=IncomingRequestCfProperties, js_name=regionCode)]
    pub fn region_code(this: &Cf) -> Option<String>;

    #[wasm_bindgen(structural, method, getter, js_class=IncomingRequestCfProperties, js_name=timezone)]
    pub fn timezone(this: &Cf) -> String;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=::js_sys::Object, js_name=tlsClientAuth)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type TlsClientAuth;

    #[wasm_bindgen(structural, method, getter, js_name=certIssuerDNLegacy, js_class = "tlsClientAuth")]
    pub fn cert_issuer_dn_legacy(this: &TlsClientAuth) -> String;

    #[wasm_bindgen(structural, method, getter, js_name=certIssuerDN, js_class = "tlsClientAuth")]
    pub fn cert_issuer_dn(this: &TlsClientAuth) -> String;

    #[wasm_bindgen(structural, method, getter, js_name=certIssuerDNRFC2253, js_class = "tlsClientAuth")]
    pub fn cert_issuer_dn_rcf2253(this: &TlsClientAuth) -> String;

    #[wasm_bindgen(structural, method, getter, js_name=certSubjectDNLegacy, js_class = "tlsClientAuth")]
    pub fn cert_subject_dn_legacy(this: &TlsClientAuth) -> String;

    #[wasm_bindgen(structural, method, getter, js_name=certVerified, js_class = "tlsClientAuth")]
    pub fn cert_verified(this: &TlsClientAuth) -> String;

    #[wasm_bindgen(structural, method, getter, js_name=certNotAfter, js_class = "tlsClientAuth")]
    pub fn cert_not_after(this: &TlsClientAuth) -> String;

    #[wasm_bindgen(structural, method, getter, js_name=certSubjectDN, js_class = "tlsClientAuth")]
    pub fn cert_subject_dn(this: &TlsClientAuth) -> String;

    #[wasm_bindgen(structural, method, getter, js_name=certFingerprintSHA1, js_class = "tlsClientAuth")]
    pub fn cert_fingerprint_sha1(this: &TlsClientAuth) -> String;

    #[wasm_bindgen(structural, method, getter, js_name=certNotBefore, js_class = "tlsClientAuth")]
    pub fn cert_not_before(this: &TlsClientAuth) -> String;

    #[wasm_bindgen(structural, method, getter, js_name=certSerial, js_class = "tlsClientAuth")]
    pub fn cert_serial(this: &TlsClientAuth) -> String;

    #[wasm_bindgen(structural, method, getter, js_name=certPresented, js_class = "tlsClientAuth")]
    pub fn cert_presented(this: &TlsClientAuth) -> String;

    #[wasm_bindgen(structural, method, getter, js_name=certSubjectDNRFC225, js_class = "tlsClientAuth")]
    pub fn cert_subject_dn_rfc225(this: &TlsClientAuth) -> String;
}

#[wasm_bindgen]
extern "C" {
    pub type DurableObjectState;
    pub type DurableObjectStorage;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &DurableObjectState) -> DurableObjectId;

    #[wasm_bindgen(method, getter)]
    pub fn storage(this: &DurableObjectState) -> DurableObjectStorage;
}

#[wasm_bindgen]
extern "C" {
    pub type DurableObjectId;

    #[wasm_bindgen(method, js_name=toString)]
    pub fn to_string(this: &DurableObjectId) -> JsString;
}

#[wasm_bindgen]
extern "C" {
    pub type Env;

    #[wasm_bindgen(method, getter, js_name=durableObject)]
    pub fn durable_object(this: &Env) -> DurableObjectNamespace;
}

#[wasm_bindgen]
extern "C" {
    pub type DurableObjectNamespace;

    #[wasm_bindgen(method)]
    pub fn get(this: &DurableObjectNamespace, id: &DurableObjectId) -> DurableObjectStub;

    #[wasm_bindgen(method, js_name=newUniqueId)]
    pub fn new_unique_id(this: &DurableObjectNamespace) -> DurableObjectId;

    #[wasm_bindgen(method, js_name=idFromName)]
    pub fn id_from_name(this: &DurableObjectNamespace, name: &str) -> DurableObjectId;

    #[wasm_bindgen(method, js_name=idFromString)]
    pub fn id_from_string(this: &DurableObjectNamespace, name: &str) -> DurableObjectId;
}

#[wasm_bindgen]
extern "C" {
    pub type DurableObjectStub;

    #[wasm_bindgen(method)]
    pub fn fetch(this: &DurableObjectStub, request: &Request) -> Promise;
}
