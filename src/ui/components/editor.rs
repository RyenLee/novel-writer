// 编辑器组件
/// 章节编辑器组件
use dioxus::prelude::*;
use crate::db::Chapter;

#[derive(Props, Clone, PartialEq)]
pub struct EditorProps {
    pub current_chapter: Signal<Option<Chapter>>,
    pub chapter_content: Signal<String>,
    pub on_save: EventHandler<()>,
}

#[component]
pub fn Editor(props: EditorProps) -> Element {
    let current_chapter = props.current_chapter;
    let mut chapter_content = props.chapter_content;
    let on_save = props.on_save;
    
    rsx! {
        div {
            class: "editor-container",
            
            { current_chapter().map(|chapter| {
                let title = chapter.title.clone();
                let word_count = chapter.word_count;
                let chapter_type_str = chapter.chapter_type.as_str().to_string();
                let updated_at = chapter.updated_at.format("%Y-%m-%d %H:%M:%S").to_string();
                
                rsx! {
                    div {
                        class: "editor",
                        
                        div {
                            class: "editor-header",
                            h2 { "{title}" }
                            div {
                                class: "editor-stats",
                                span { "字数: {word_count}" }
                                span { "类型: {chapter_type_str}" }
                                span { "最后更新: {updated_at}" }
                            }
                        }
                        
                        div {
                            class: "editor-toolbar",
                            button {
                                class: "toolbar-btn",
                                onclick: move |_| on_save.call(()),
                                title: "保存 (Ctrl+S)",
                                "💾 保存"
                            }
                            button {
                                class: "toolbar-btn",
                                title: "撤销",
                                "↶ 撤销"
                            }
                            button {
                                class: "toolbar-btn",
                                title: "重做",
                                "↷ 重做"
                            }
                        }
                        
                        div {
                            class: "editor-content",
                            textarea {
                                class: "editor-textarea",
                                value: "{chapter_content}",
                                oninput: move |e| chapter_content.set(e.value()),
                                placeholder: "开始写作...",
                                spellcheck: true,
                            }
                        }
                        
                        div {
                            class: "editor-footer",
                            span { "当前字数: {chapter_content().chars().filter(|c| !c.is_whitespace()).count()}" }
                        }
                    }
                }
            }).unwrap_or_else(|| rsx! {
                div {
                    class: "editor-placeholder",
                    p { "📝 请选择一个章节开始编辑" }
                    p {
                        class: "hint",
                        "在左侧章节列表中选择或创建一个章节"
                    }
                }
            }) }
        }
    }
}