// 工具栏组件
use dioxus::prelude::*;

#[component]
pub fn Toolbar() -> Element {
    rsx! {
        div {
            class: "toolbar",
            "工具栏组件"
        }
    }
}