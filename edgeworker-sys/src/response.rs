use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Body {
    BufferSource,
    FormData,
    ReadableStream,
    UrlSearchParams,
    UsvString,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=::js_sys::Object, js_name=Response)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    // #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub type Response;

    #[cfg(feature = "ResponseType")]
    #[wasm_bindgen(structural, method, getter, js_class=Response, js_name=type)]
    // #[doc = "*This API requires the following crate features to be activated: `Response`, `ResponseType`*"]
    pub fn type_(this: &Response) -> ResponseType;

    #[wasm_bindgen(structural, method, getter, js_class=Response, js_name=url)]
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn url(this: &Response) -> String;

    #[wasm_bindgen(structural, method, getter, js_class=Response, js_name=redirected)]
    #[doc = "Getter for the `redirected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn redirected(this: &Response) -> bool;

    #[wasm_bindgen(structural, method, getter, js_class=Response, js_name=status)]
    #[doc = "Getter for the `status` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/status)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn status(this: &Response) -> u16;

    #[wasm_bindgen(structural, method, getter, js_class=Response, js_name=ok)]
    #[doc = "Getter for the `ok` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/ok)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn ok(this: &Response) -> bool;

    #[wasm_bindgen(structural, method, getter, js_class=Response, js_name=statusText)]
    #[doc = "Getter for the `statusText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/statusText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn status_text(this: &Response) -> String;

    #[cfg(feature = "Headers")]
    #[wasm_bindgen(structural, method, getter, js_class=Response, js_name=headers)]
    #[doc = "Getter for the `headers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/headers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`, `Response`*"]
    pub fn headers(this: &Response) -> web_sys::Headers;

    #[wasm_bindgen(structural, method, getter, js_class=Response, js_name=bodyUsed)]
    #[doc = "Getter for the `bodyUsed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/bodyUsed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn body_used(this: &Response) -> bool;

    #[cfg(feature = "ReadableStream")]
    #[wasm_bindgen(structural, method, getter, js_class=Response, js_name=body)]
    #[doc = "Getter for the `body` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/body)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `Response`*"]
    pub fn body(this: &Response) -> Option<web_sys::ReadableStream>;

    #[wasm_bindgen(catch, constructor, js_class=Response)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn new() -> Result<Response, JsValue>;

    #[cfg(feature = "Blob")]
    #[wasm_bindgen(catch, constructor, js_class=Response)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `Response`*"]
    pub fn new_with_opt_blob(body: Option<&Blob>) -> Result<Response, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class=Response)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn new_with_opt_buffer_source(body: Option<&::js_sys::Object>)
        -> Result<Response, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class=Response)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn new_with_opt_u8_array(body: Option<&mut [u8]>) -> Result<Response, JsValue>;

    #[cfg(feature = "FormData")]
    #[wasm_bindgen(catch, constructor, js_class=Response)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`, `Response`*"]
    pub fn new_with_opt_form_data(body: Option<&FormData>) -> Result<Response, JsValue>;

    #[cfg(feature = "UrlSearchParams")]
    #[wasm_bindgen(catch, constructor, js_class=Response)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `UrlSearchParams`*"]
    pub fn new_with_opt_url_search_params(
        body: Option<&UrlSearchParams>,
    ) -> Result<Response, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class=Response)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn new_with_opt_str(body: Option<&str>) -> Result<Response, JsValue>;

    #[cfg(feature = "ReadableStream")]
    #[wasm_bindgen(catch, constructor, js_class=Response)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `Response`*"]
    pub fn new_with_opt_readable_stream(
        body: Option<&web_sys::ReadableStream>,
    ) -> Result<Response, JsValue>;
    #[cfg(all(feature = "Blob", feature = "ResponseInit",))]
    #[wasm_bindgen(catch, constructor, js_class=Response)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `Response`, `ResponseInit`*"]
    pub fn new_with_opt_blob_and_init(
        body: Option<&Blob>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(feature = "ResponseInit")]
    #[wasm_bindgen(catch, constructor, js_class=Response)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `ResponseInit`*"]
    pub fn new_with_opt_buffer_source_and_init(
        body: Option<&::js_sys::Object>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(feature = "ResponseInit")]
    #[wasm_bindgen(catch, constructor, js_class=Response)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `ResponseInit`*"]
    pub fn new_with_opt_u8_array_and_init(
        body: Option<&mut [u8]>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(all(feature = "FormData", feature = "ResponseInit",))]
    #[wasm_bindgen(catch, constructor, js_class=Response)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`, `Response`, `ResponseInit`*"]
    pub fn new_with_opt_form_data_and_init(
        body: Option<&FormData>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(all(feature = "ResponseInit", feature = "UrlSearchParams",))]
    #[wasm_bindgen(catch, constructor, js_class=Response)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `ResponseInit`, `UrlSearchParams`*"]
    pub fn new_with_opt_url_search_params_and_init(
        body: Option<&UrlSearchParams>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(feature = "ResponseInit")]
    #[wasm_bindgen(catch, constructor, js_class=Response)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `ResponseInit`*"]
    pub fn new_with_opt_str_and_init(
        body: Option<&str>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(all(feature = "ReadableStream", feature = "ResponseInit",))]
    #[wasm_bindgen(catch, constructor, js_class=Response)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `Response`, `ResponseInit`*"]
    pub fn new_with_opt_readable_stream_and_init(
        body: Option<&web_sys::ReadableStream>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[wasm_bindgen(catch, method, structural, js_class=Response, js_name=clone)]
    #[doc = "The `clone()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/clone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn clone(this: &Response) -> Result<Response, JsValue>;

    #[wasm_bindgen(static_method_of=Response, js_class=Response, js_name=error)]
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn error() -> Response;

    #[wasm_bindgen(catch, static_method_of=Response, js_class=Response, js_name=redirect)]
    #[doc = "The `redirect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn redirect(url: &str) -> Result<Response, JsValue>;

    #[wasm_bindgen(catch, static_method_of=Response, js_class=Response, js_name=redirect)]
    #[doc = "The `redirect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn redirect_with_status(url: &str, status: u16) -> Result<Response, JsValue>;

    #[wasm_bindgen(catch, method, structural, js_class=Response, js_name=arrayBuffer)]
    #[doc = "The `arrayBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/arrayBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn array_buffer(this: &Response) -> Result<::js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, structural, js_class=Response, js_name=blob)]
    #[doc = "The `blob()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn blob(this: &Response) -> Result<::js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, structural, js_class=Response, js_name=formData)]
    #[doc = "The `formData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/formData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn form_data(this: &Response) -> Result<::js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, structural, js_class=Response, js_name=json)]
    #[doc = "The `json()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/json)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn json(this: &Response) -> Result<::js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, method, structural, js_class=Response, js_name=text)]
    #[doc = "The `text()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn text(this: &Response) -> Result<::js_sys::Promise, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends =::js_sys::Object, js_name=ResponseInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ResponseInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub type ResponseInit;
}

impl ResponseInit {
    #[doc = "Construct a new `ResponseInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
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
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub fn status(&mut self, val: u16) -> &mut Self {
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("status"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `statusText` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub fn status_text(&mut self, val: &str) -> &mut Self {
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("statusText"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for ResponseInit {
    fn default() -> Self {
        ResponseInit::new()
    }
}
