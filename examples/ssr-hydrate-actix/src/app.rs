use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::{expect_i18n, leptos_fluent, move_tr, tr, Language};
use leptos_meta::*;
use leptos_router::*;

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    leptos_fluent! {{
        translations: TRANSLATIONS,
        locales: "./locales",
        url_param: "lang",
        initial_language_from_url_param: true,
        initial_language_from_url_param_to_localstorage: true,
        set_language_to_url_param: true,
        localstorage_key: "language",
        initial_language_from_localstorage: true,
        set_language_to_localstorage: true,
        initial_language_from_navigator: true,
        initial_language_from_accept_language_header: true,
    }};

    view! {
        <Title text=move || tr!("welcome-to-leptos")/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let i18n = expect_i18n();

    view! {
        <h1>{move_tr!("welcome-to-leptos")}</h1>
        <fieldset>
            <For
                each=move || i18n.languages
                key=move |lang| i18n.language_key(lang)
                children=move |lang: &&Language| {
                    view! {
                        <div>
                            <input
                                type="radio"
                                id=lang.id.to_string()
                                name="language"
                                value=lang.id.to_string()
                                checked=i18n.is_active_language(lang)
                                on:click=move |_| i18n.set_language(lang)
                            />
                            <label for=lang.id.to_string()>{lang.name}</label>
                        </div>
                    }
                }
            />

        </fieldset>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>{move_tr!("not-found")}</h1> }
}
