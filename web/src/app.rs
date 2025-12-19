use leptos::prelude::*;

use crate::components::Layout;
use crate::pages::{ArchivePostView, Archives, Home, PostView};
use crate::router::{Route, use_router};
use sinter_theme_sdk::GlobalState;
use std::sync::Arc;

#[component]
pub fn App() -> impl IntoView {
    // 0. Initialize themes registry
    let manager = sinter_themes::init_manager();
    let manager = Arc::new(manager);

    // 1. Create the GlobalState which includes data fetching resources and theme
    // 2. Provide the state as global context
    provide_context(GlobalState::new(manager, "default"));

    // 3. Use Simple Router
    let (route, page) = use_router();

    view! {
        <Layout>
            {move || {
                match route.get() {
                    Route::Home => view! { <Home page=page /> }.into_any(),
                    Route::Archives => view! { <Archives page=page /> }.into_any(),
                    Route::Post(slug) => view! {
                        <PostView slug=Signal::derive(move || slug.clone()) />
                    }.into_any(),
                    Route::ArchivePost(slug) => view! {
                        <ArchivePostView slug=Signal::derive(move || slug.clone()) />
                    }.into_any(),
                    Route::NotFound => view! { "404 - Not Found" }.into_any(),
                }
            }}
        </Layout>
    }
}
