pub fn donwload_file(url: &str, name: &str) {
    #[cfg(feature = "web")]
    {
        use wasm_bindgen::JsCast;

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let a = document.create_element("a").unwrap();
        a.set_attribute("href", url).unwrap();
        a.set_attribute("download", name).unwrap();

        document.body().unwrap().append_child(&a).unwrap();
        let a: web_sys::HtmlElement = a.unchecked_into();
        a.click();
        a.remove();
    }
    #[cfg(not(feature = "web"))]
    {
        use dioxus_logger::tracing;

        tracing::warn!(
            "Download is not supported on this platform {} {}",
            url,
            name
        );
    }
}
