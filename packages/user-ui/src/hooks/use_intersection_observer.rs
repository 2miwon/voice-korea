use dioxus::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

impl EventListenerHandle {
    pub fn new<EventKind, T>(
        target_element: T,
        event_name: &'static str,
        mut callback: impl FnMut(EventKind) + 'static,
    ) -> Self
    where
        EventKind: Sized + RefFromWasmAbi + FromWasmAbi + Clone + 'static,
        T: Clone + Deref<Target = EventTarget> + std::fmt::Debug + 'static,
    {
        let closure = Closure::wrap(Box::new(move |event: EventKind| {
            callback(event);
        }) as Box<dyn FnMut(_)>);
        if let Err(e) = target_element
            .add_event_listener_with_callback(&event_name, closure.as_ref().unchecked_ref())
        {
            tracing::error!("failed to add event listener: {e:?}");
        }

        let cleanup = Rc::new(RefCell::new(Some(Box::new(move || {
            if let Err(e) = target_element
                .remove_event_listener_with_callback(&event_name, closure.as_ref().unchecked_ref())
            {
                tracing::error!("failed to remove event listener: {e:?}");
            }
        }) as Box<dyn FnOnce()>)));
        Self { cleanup }
    }

    pub fn cleanup(&self) {
        let cleanup = self.cleanup.borrow_mut().take();
        if let Some(cleanup) = cleanup {
            cleanup();
        }
    }
}

impl Drop for EventListenerHandle {
    fn drop(&mut self) {
        // Only cleanup if this is the last reference.
        if Rc::strong_count(&self.cleanup) == 1 {
            self.cleanup();
        }
    }
}

pub fn use_on_event<EventKind, T>(
    target_element: &T,
    event_name: &'static str,
    mut callback: impl FnMut(EventKind) + 'static,
) where
    EventKind: Sized + RefFromWasmAbi + FromWasmAbi + Clone + 'static,
    T: Clone + Deref<Target = EventTarget> + std::fmt::Debug + 'static,
{
    let hook = || {
        EventListenerHandle::new(target_element.clone(), event_name, move |kind| {
            debug!(%event_name, "on event");
            callback(kind)
        })
    };

    let cleanup = |f: EventListenerHandle| {
        tracing::info!("CLEANUP");
        f.cleanup();
    };

    use_hook_with_cleanup(hook, cleanup);
}

// This is modified from another user on this discord.
pub fn use_outside_click<S: ToString>(id: S, mut callback: impl FnMut(Element) + 'static) {
    let window = window().expect("");
    let document = window.document().expect("");

    let id = id.to_string();
    use_on_event(&window, "mousedown", move |ev: MouseEvent| {
        if let Some(target) = ev.target() {
            if let Some(dropdown) = document.get_element_by_id(&id) {
                let target_element: &HtmlElement = target.unchecked_ref();
                let target_node: &web_sys::Node = target_element.as_ref();
                if !dropdown.contains(Some(target_node)) {
                    callback(dropdown);
                }
            }
        }
    })
}
