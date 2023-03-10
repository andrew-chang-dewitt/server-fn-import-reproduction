// glob import has no issues w/ the server macro
// use leptos::*;
// explicit imports runs into a series of weird errors
use leptos::{
    component, create_resource, create_server_action,
    server_fn::{self, server, ServerFn, ServerFnError},
    view, IntoView, Scope, SignalGet, Suspense, SuspenseProps,
};

/// An example server function
#[server(Example, "/api")]
async fn example(_: Scope) -> Result<String, ServerFnError> {
    Ok(String::from("From Server Function"))
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <HomePage/>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // use the server function
    let action = create_server_action::<Example>(cx);
    let on_click = move |_| action.dispatch(Example {});
    let value = create_resource(
        cx,
        move || action.version().get(),
        move |_| {
            log::info!("refetching resource");
            example(cx)
        },
    );

    view! { cx,
        <button on:click=on_click>"Dispatch Server Function"</button>
        <Suspense fallback={move || view!{cx,"Loading..."}}>
            {value.read(cx)}
        </Suspense>
    }
}
