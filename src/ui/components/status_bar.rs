use dioxus::prelude::*;
use crate::db::Novel;

#[derive(Props, Clone, PartialEq)]
pub struct StatusBarProps {
    pub current_novel_id: Signal<Option<i64>>,
    pub novels: Signal<Vec<Novel>>,
}

#[component]
pub fn StatusBar(props: StatusBarProps) -> Element {
    let current_novel_id = props.current_novel_id;
    let novels = props.novels;
    
    rsx! {
        footer {
            class: "status-bar",
            div {
                class: "status-left",
                "就绪"
            }
            div {
                class: "status-center",
                if let Some(novel_id) = current_novel_id() {
                    if let Some(novel) = novels().iter().find(|n| n.id == novel_id) {
                        "当前小说: {novel.title}"
                    } else {
                        "未选择小说"
                    }
                } else {
                    "未选择小说"
                }
            }
            div {
                class: "status-right",
                "小说数量: {novels().len()} | 自动保存: 开启"
            }
        }
    }
}