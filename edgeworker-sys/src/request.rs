use wasm_bindgen::prelude::*;

use crate::Cf;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=::js_sys::Object, js_name=Request)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Request;

    #[wasm_bindgen(structural, method, getter, js_class=Request, js_name=method)]
    pub fn method(this: &Request) -> String;

    #[wasm_bindgen(structural, method, getter, js_class=Request, js_name=url)]
    pub fn url(this: &Request) -> String;

    #[cfg(feature = "Headers")]
    #[wasm_bindgen(structural, method, getter, js_class=Request, js_name=headers)]
    // #[doc = "*This API requires the following crate features to be activated: `Headers`, `Request`*"]
    pub fn headers(this: &Request) -> web_sys::Headers;

    #[cfg(feature = "RequestRedirect")]
    #[wasm_bindgen(structural, method, getter, js_class=Request, js_name=redirect)]
    // #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestRedirect`*"]
    pub fn redirect(this: &Request) -> web_sys::RequestRedirect;

    #[wasm_bindgen(structural, method, getter, js_class=Request, js_name=bodyUsed)]
    // #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    pub fn body_used(this: &Request) -> bool;

    #[cfg(feature = "ReadableStream")]
    #[wasm_bindgen(structural, method, getter, js_class=Request, js_name=body)]
    // #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `Request`*"]
    pub fn body(this: &Request) -> Option<web_sys::ReadableStream>;

    #[cfg(feature = "Request")]
    #[wasm_bindgen(catch, constructor, js_class=Request)]
    // #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    pub fn new_with_request(input: &Request) -> Result<Request, JsValue>;

    #[cfg(feature = "Request")]
    #[wasm_bindgen(catch, constructor, js_class=Request)]
    // #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    pub fn new_with_str(input: &str) -> Result<Request, JsValue>;

    #[cfg(feature = "RequestInit")]
    #[wasm_bindgen(catch, constructor, js_class=Request)]
    // #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestInit`*"]
    pub fn new_with_request_and_init(
        input: &Request,
        init: &RequestInit,
    ) -> Result<Request, JsValue>;

    #[cfg(feature = "RequestInit")]
    #[wasm_bindgen(catch, constructor, js_class=Request)]
    // #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestInit`*"]
    pub fn new_with_str_and_init(input: &str, init: &RequestInit) -> Result<Request, JsValue>;

    #[wasm_bindgen(catch, method, structural, js_class=Request, js_name=clone)]
    // #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    pub fn clone(this: &Request) -> Result<Request, JsValue>;

    #[wasm_bindgen(catch, method, structural, js_class=Request, js_name=arrayBuffer)]
    // #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    pub fn array_buffer(this: &Request) -> Result<::js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, structural, js_class=Request, js_name=formData)]
    // #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    pub fn form_data(this: &Request) -> Result<::js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, structural, js_class=Request, js_name=json)]
    // #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    pub fn json(this: &Request) -> Result<::js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, structural, js_class=Request, js_name=text)]
    // #[doc = "*This API requires the following crate features to be activated: `Request`*"]
    pub fn text(this: &Request) -> Result<::js_sys::Promise, JsValue>;

    #[wasm_bindgen(structural, method, getter, js_class=Request, js_name=cf)]
    // #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestCache`*"]
    pub fn cf(this: &Request) -> Cf;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (extends = :: js_sys :: Object , js_name = RequestInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RequestInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    pub type RequestInit;
}

#[wasm_bindgen]
#[doc = "The `RequestRedirect` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RequestRedirect`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestRedirect {
    Follow = "follow",
    Error = "error",
    Manual = "manual",
}

impl RequestInit {
    #[doc = "Construct a new `RequestInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `body` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    pub fn body(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("body"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "RequestCache")]
    #[doc = "Change the `cache` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCache`, `RequestInit`*"]
    pub fn cache(&mut self, val: RequestCache) -> &mut Self {
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("cache"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Change the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `RequestInit`*"]
    pub fn credentials(&mut self, val: RequestCredentials) -> &mut Self {
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("credentials"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    pub fn headers(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("headers"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `integrity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    pub fn integrity(&mut self, val: &str) -> &mut Self {
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("integrity"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `method` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    pub fn method(&mut self, val: &str) -> &mut Self {
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("method"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "RequestMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`, `RequestMode`*"]
    pub fn mode(&mut self, val: RequestMode) -> &mut Self {
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("mode"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "ObserverCallback")]
    #[doc = "Change the `observe` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ObserverCallback`, `RequestInit`*"]
    pub fn observe(&mut self, val: &ObserverCallback) -> &mut Self {
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("observe"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "RequestRedirect")]
    #[doc = "Change the `redirect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`, `RequestRedirect`*"]
    pub fn redirect(&mut self, val: RequestRedirect) -> &mut Self {
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("redirect"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    pub fn referrer(&mut self, val: &str) -> &mut Self {
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("referrer"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "ReferrerPolicy")]
    #[doc = "Change the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReferrerPolicy`, `RequestInit`*"]
    pub fn referrer_policy(&mut self, val: ReferrerPolicy) -> &mut Self {
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("referrerPolicy"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `RequestInit`*"]
    pub fn signal(&mut self, val: Option<&AbortSignal>) -> &mut Self {
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("signal"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
