use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    pub search_query: Signal<String>,
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
    let mut search_query = props.search_query;
    
    rsx! {
        header {
            class: "app-header",
            div {
                class: "header-left",
                h1 { "📖 小说写作工具" }
            }
            div {
                class: "header-center",
                input {
                    r#type: "text",
                    placeholder: "搜索小说、章节或灵感...",
                    value: "{search_query}",
                    oninput: move |e| search_query.set(e.value()),
                }
            }
            div {
                class: "header-right",
                button { "🔍 搜索" }
                button { "⚙️ 设置" }
                button { "❓ 帮助" }
            }
        }
    }
}