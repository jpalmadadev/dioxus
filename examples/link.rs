#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            p {
                a { href: "http://dioxuslabs.com/", "Default link - links outside of your app" }
            }
            p {
                a {
                    href: "http://dioxuslabs.com/",
                    prevent_default: "onclick",
                    onclick: |_| println!("Hello Dioxus"),
                    "Custom event link - links inside of your app",
                }
            }
        }
        div {
            Router::<Route> {}
        }
    ))
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(Header)]
        #[route("/")]
        Home {},
        #[route("/settings")]
        Settings {},
}

#[inline_props]
fn Header(cx: Scope) -> Element {
    render! {
        h1 { "Your app here" }
        ul {
            li { Link { to: Route::Home {}, "home" } }
            li { Link { to: Route::Settings {}, "settings" } }
        }
        Outlet::<Route> {}
    }
}

#[inline_props]
fn Home(cx: Scope) -> Element {
    render!(h1 { "Home" })
}

#[inline_props]
fn Settings(cx: Scope) -> Element {
    render!(h1 { "Settings" })
}
