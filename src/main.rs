use dioxus::prelude::*;

fn main() {
    launch_desktop(app);
}

fn app() -> Element {
    rsx!(
        style { {include_str!("../out.css")} },
        div {
            class: "w-full h-screen bg-[#152D35]",
            "Dioxus Application"
        },
    )
}
