use leptos::prelude::*;
use sinter_core::Post;
use sinter_theme_sdk::{PageDataContext, fetch_archive_page_data, fetch_json, fetch_page_data};

#[component]
pub fn Home(#[prop(into)] page: Signal<usize>) -> impl IntoView {
    let state = use_context::<sinter_theme_sdk::GlobalState>().expect("缺少 GlobalState");

    // 创建页面数据资源
    let page_data_resource = LocalResource::new(move || {
        let page_num = page.get();
        async move { fetch_page_data(page_num).await }
    });

    // 提供 PageDataContext 供主题使用
    provide_context(PageDataContext(page_data_resource));

    view! {
        {state.theme.get_untracked().render_home()}
    }
}

#[component]
pub fn Archives(#[prop(into)] page: Signal<usize>) -> impl IntoView {
    let state = use_context::<sinter_theme_sdk::GlobalState>().expect("缺少 GlobalState");

    // 创建页面数据资源 (Archives)
    let page_data_resource = LocalResource::new(move || {
        let page_num = page.get();
        async move { fetch_archive_page_data(page_num).await }
    });

    // 提供 PageDataContext 供主题使用
    provide_context(PageDataContext(page_data_resource));

    view! {
        {state.theme.get_untracked().render_archive()}
    }
}

#[component]
pub fn PostView(#[prop(into)] slug: Signal<String>) -> impl IntoView {
    let state = use_context::<sinter_theme_sdk::GlobalState>().expect("缺少 GlobalState");

    // 捕获 state 以便在闭包中使用
    let theme_signal = state.theme;

    // 根据 slug 获取文章详情
    let post_resource = LocalResource::new(move || {
        let current_slug = slug.get();

        async move {
            // 直接构建 URL 请求文章数据
            let url = format!("/sinter_data/posts/{}.json", current_slug);

            match fetch_json::<Post>(&url).await {
                Ok(post) => Some(post),
                Err(_) => None,
            }
        }
    });

    view! {
        <Suspense fallback=move || theme_signal.get_untracked().render_post_loading()>
            {move || {
                let theme = theme_signal.get_untracked();
                match post_resource.get() {
                    Some(Some(post)) => theme.render_post(post),
                    Some(None) => theme.render_post_not_found(),
                    None => theme.render_post_loading(),
                }
            }}
        </Suspense>
    }
}

#[component]
pub fn ArchivePostView(#[prop(into)] slug: Signal<String>) -> impl IntoView {
    let state = use_context::<sinter_theme_sdk::GlobalState>().expect("缺少 GlobalState");

    let theme_signal = state.theme;

    let post_resource = LocalResource::new(move || {
        let current_slug = slug.get();

        async move {
            let url = format!("/sinter_data/archives/{}.json", current_slug);

            match fetch_json::<Post>(&url).await {
                Ok(post) => Some(post),
                Err(_) => None,
            }
        }
    });

    view! {
        <Suspense fallback=move || theme_signal.get_untracked().render_post_loading()>
            {move || {
                let theme = theme_signal.get_untracked();
                match post_resource.get() {
                    Some(Some(post)) => theme.render_post(post),
                    Some(None) => theme.render_post_not_found(),
                    None => theme.render_post_loading(),
                }
            }}
        </Suspense>
    }
}
