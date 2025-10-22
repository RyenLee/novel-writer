/// 章节表单组件 - 用于创建和编辑章节
use dioxus::prelude::*;
use crate::db::{Chapter, ChapterType};

#[derive(Props, Clone, PartialEq)]
pub struct ChapterFormProps {
    pub editing_chapter: Signal<Option<Chapter>>,
    pub parent_chapter_id: Signal<Option<i64>>,
    pub chapter_title: Signal<String>,
    pub chapter_type: Signal<ChapterType>,
    pub on_submit: EventHandler<()>,
    pub on_cancel: EventHandler<()>,
}

#[component]
pub fn ChapterForm(props: ChapterFormProps) -> Element {
    let editing_chapter = props.editing_chapter;
    let parent_chapter_id = props.parent_chapter_id;
    let mut chapter_title = props.chapter_title;
    let mut chapter_type = props.chapter_type;
    let on_submit = props.on_submit;
    let on_cancel = props.on_cancel;
    
    rsx! {
        div {
            class: "chapter-form-overlay",
            onclick: move |_| on_cancel.call(()),
            
            div {
                class: "chapter-form",
                onclick: move |e: Event<MouseData>| e.stop_propagation(),
                
                h3 {
                    if editing_chapter().is_some() {
                        "编辑章节"
                    } else if parent_chapter_id().is_some() {
                        "新建子章节"
                    } else {
                        "新建章节"
                    }
                }
                
                div {
                    class: "form-group",
                    label { "章节标题" }
                    input {
                        r#type: "text",
                        class: "form-input",
                        placeholder: "请输入章节标题",
                        value: "{chapter_title}",
                        oninput: move |e| chapter_title.set(e.value()),
                        autofocus: true,
                    }
                }
                
                div {
                    class: "form-group",
                    label { "章节类型" }
                    div {
                        class: "chapter-type-selector",
                        button {
                            class: if chapter_type() == ChapterType::Volume { "type-btn active" } else { "type-btn" },
                            onclick: move |_| chapter_type.set(ChapterType::Volume),
                            span { "📚 卷" }
                        }
                        button {
                            class: if chapter_type() == ChapterType::Chapter { "type-btn active" } else { "type-btn" },
                            onclick: move |_| chapter_type.set(ChapterType::Chapter),
                            span { "📄 章节" }
                        }
                        button {
                            class: if chapter_type() == ChapterType::Scene { "type-btn active" } else { "type-btn" },
                            onclick: move |_| chapter_type.set(ChapterType::Scene),
                            span { "🎬 场景" }
                        }
                    }
                }
                
                if parent_chapter_id().is_some() {
                    div {
                        class: "form-hint",
                        "💡 此章节将作为子章节创建"
                    }
                }
                
                div {
                    class: "form-actions",
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| on_submit.call(()),
                        disabled: chapter_title().trim().is_empty(),
                        if editing_chapter().is_some() {
                            "更新章节"
                        } else {
                            "创建章节"
                        }
                    }
                    button {
                        class: "btn btn-secondary",
                        onclick: move |_| on_cancel.call(()),
                        "取消"
                    }
                }
            }
        }
    }
}
