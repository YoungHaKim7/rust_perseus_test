use perseus::prelude::*;
use sycamore::prelude::*;

fn studykr_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        // div
        div(style = "display: flex; flex-direction: column; justify-content: center; align-items: center; height: 95vh;") {
            h1 { "Study! Study" }
            p { "Discord. Rust Programming"}
            a(href = "https://", id = "study-link") { "_rust_study group!"}
            p { "_______________."}
            p { "Discord Korea."}
            a(href = "https://discord.gg/GKXNDdn2CP", id = "study-link") { "Kor_rust_study group!"}
        }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("studykr").view(studykr_page).build()
}
