use dioxus::prelude::*;
use crate::db::Novel;

#[derive(Props, Clone, PartialEq)]
pub struct NovelDetailProps {
    pub current_novel_id: Signal<Option<i64>>,
    pub novels: Signal<Vec<Novel>>,
    pub on_edit_novel: EventHandler<Novel>,
    pub on_delete_novel: EventHandler<i64>,
}

#[component]
pub fn NovelDetail(props: NovelDetailProps) -> Element {
    let current_novel_id = props.current_novel_id;
    let novels = props.novels;
    let on_edit_novel = props.on_edit_novel;
    let on_delete_novel = props.on_delete_novel;
    
    rsx! {
        div {
            class: "novel-management",
            h2 { "📚 小说管理" }
            if let Some(current_id) = current_novel_id() {
                // 从列表中克隆找到的小说以避免借用问题
                { novels().iter().find(|n| n.id == current_id).cloned().map(|novel| {
                    let novel_for_edit = novel.clone();
                    let novel_id = novel.id;
                                
                    rsx! {
                        div {
                            class: "novel-detail",
                            h3 { "{novel.title}" }
                            p { "作者: {novel.author}" }
                            p { "字数: {novel.word_count}" }
                            p { "状态: {novel.status.as_str()}" }
                            p { "创建时间: {novel.created_at.to_string()}" }
                            p { "最后更新: {novel.updated_at.to_string()}" }
                            div {
                                class: "novel-description",
                                h4 { "简介" }
                                p { "{novel.description}" }
                            }
                            div {
                                class: "novel-actions-detail",
                                button {
                                    class: "primary-btn",
                                    onclick: move |_| on_edit_novel.call(novel_for_edit.clone()),
                                    "编辑小说"
                                }
                                button {
                                    class: "danger-btn",
                                    onclick: move |_| on_delete_novel.call(novel_id),
                                    "删除小说"
                                }
                            }
                        }
                    }
                }) }
            } else {
                div {
                    class: "novel-placeholder",
                    p { "请从左侧选择一部小说,或创建新小说" }
                }
            }
        }
    }
}