pub mod attribute;
pub mod element;
pub mod suspense;
pub mod view;

pub use attribute::*;
pub use element::*;
pub use suspense::*;
pub use view::*;
use web_sys::Document;

thread_local! {
    static DOCUMENT: Document = {
        let window = web_sys::window().expect("No global window");
        window.document().expect("No document")
    };
}

pub(crate) fn document() -> Document {
    DOCUMENT.with(|d| d.clone())
}
