use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, Url};

#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    Home,
    Archives,
    Post(String),
    ArchivePost(String),
    NotFound,
}

impl Route {
    fn from_path(path: &str) -> Self {
        if path == "/" || path == "/index.html" {
            Route::Home
        } else if path == "/archives" || path == "/archives/" {
            Route::Archives
        } else if let Some(slug) = path.strip_prefix("/posts/") {
            let slug = slug.trim_matches('/');
            if slug.is_empty() {
                Route::NotFound
            } else {
                Route::Post(slug.to_string())
            }
        } else if let Some(slug) = path.strip_prefix("/archives/posts/") {
            let slug = slug.trim_matches('/');
            if slug.is_empty() {
                Route::NotFound
            } else {
                Route::ArchivePost(slug.to_string())
            }
        } else {
            Route::NotFound
        }
    }
}

pub fn use_router() -> (Signal<Route>, Signal<usize>) {
    let (path, set_path) = signal(
        web_sys::window()
            .and_then(|w| w.location().pathname().ok())
            .unwrap_or_else(|| "/".to_string()),
    );

    let (search, set_search) = signal(
        web_sys::window()
            .and_then(|w| w.location().search().ok())
            .unwrap_or_default(),
    );

    // Sync UI on history change (PopState)
    Effect::new(move |_| {
        let handle = window_event_listener(ev::popstate, move |_| {
            if let Some(w) = web_sys::window() {
                let loc = w.location();
                set_path.set(loc.pathname().unwrap_or_default());
                set_search.set(loc.search().unwrap_or_default());
            }
        });
        on_cleanup(move || handle.remove());
    });

    // Intercept <a> clicks for client-side routing
    Effect::new(move |_| {
        let handle = window_event_listener(ev::click, move |ev| {
            let target = ev.target().unwrap();
            let anchor = if let Some(a) = target.dyn_ref::<HtmlAnchorElement>() {
                Some(a.clone())
            } else {
                target
                    .unchecked_ref::<web_sys::Element>()
                    .closest("a")
                    .ok()
                    .flatten()
                    .and_then(|el| el.dyn_into::<HtmlAnchorElement>().ok())
            };

            if let Some(a) = anchor {
                let href = a.href();
                if let Ok(url) = Url::new(&href) {
                    if let Ok(origin) = web_sys::window().unwrap().location().origin() {
                        if url.origin() == origin {
                            ev.prevent_default();
                            let pathname = url.pathname();
                            let search_str = url.search();

                            if let Ok(history) = web_sys::window().unwrap().history() {
                                let _ = history.push_state_with_url(
                                    &wasm_bindgen::JsValue::NULL,
                                    "",
                                    Some(&href),
                                );
                            }

                            set_path.set(pathname);
                            set_search.set(search_str);
                            web_sys::window().unwrap().scroll_to_with_x_and_y(0.0, 0.0);
                        }
                    }
                }
            }
        });
        on_cleanup(move || handle.remove());
    });

    let current_route = Signal::derive(move || Route::from_path(&path.get()));

    let current_page = Signal::derive(move || {
        let s = search.get();
        web_sys::UrlSearchParams::new_with_str(&s)
            .ok()
            .and_then(|p| p.get("page"))
            .and_then(|p_str| p_str.parse::<usize>().ok())
            .unwrap_or(1)
    });

    (current_route, current_page)
}
