use dioxus::prelude::*;
use crate::db::Novel;

#[derive(Props, Clone, PartialEq)]
pub struct NovelFormProps {
    pub novel_title: Signal<String>,
    pub novel_author: Signal<String>,
    pub novel_description: Signal<String>,
    pub editing_novel: Signal<Option<Novel>>,
    pub on_submit: EventHandler<()>,
    pub on_cancel: EventHandler<()>,
}

#[component]
pub fn NovelForm(props: NovelFormProps) -> Element {
    let mut novel_title = props.novel_title;
    let mut novel_author = props.novel_author;
    let mut novel_description = props.novel_description;
    let editing_novel = props.editing_novel;
    let on_submit = props.on_submit;
    let on_cancel = props.on_cancel;
    
    rsx! {
        div {
            class: "novel-form",
            h2 {
                if editing_novel().is_some() { "编辑小说" } else { "新建小说" }
            }
            div {
                class: "form-group",
                label { "小说标题" }
                input {
                    r#type: "text",
                    placeholder: "请输入小说标题",
                    value: "{novel_title}",
                    oninput: move |e| novel_title.set(e.value()),
                }
            }
            div {
                class: "form-group",
                label { "作者" }
                input {
                    r#type: "text",
                    placeholder: "请输入作者名",
                    value: "{novel_author}",
                    oninput: move |e| novel_author.set(e.value()),
                }
            }
            div {
                class: "form-group",
                label { "简介" }
                textarea {
                    placeholder: "请输入小说简介",
                    value: "{novel_description}",
                    oninput: move |e| novel_description.set(e.value()),
                    rows: "4",
                }
            }
            div {
                class: "form-actions",
                button {
                    class: "primary-btn",
                    onclick: move |_| on_submit.call(()),
                    if editing_novel().is_some() { "更新小说" } else { "创建小说" }
                }
                button {
                    class: "secondary-btn",
                    onclick: move |_| on_cancel.call(()),
                    "取消"
                }
            }
        }
    }
}