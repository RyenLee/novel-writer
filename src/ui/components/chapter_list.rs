/// 章节列表组件 - 支持树状结构显示
use dioxus::prelude::*;
use crate::db::{Chapter, ChapterType};
use crate::core::chapter_manager::{ChapterManager, ChapterTree};

#[derive(Props, Clone, PartialEq)]
pub struct ChapterListProps {
    pub chapters: Signal<Vec<Chapter>>,
    pub current_chapter_id: Signal<Option<i64>>,
    pub on_select_chapter: EventHandler<i64>,
    pub on_edit_chapter: EventHandler<Chapter>,
    pub on_delete_chapter: EventHandler<i64>,
    pub on_add_subchapter: EventHandler<i64>,
}

#[component]
pub fn ChapterList(props: ChapterListProps) -> Element {
    let chapters = props.chapters;
    let current_chapter_id = props.current_chapter_id;
    let on_select_chapter = props.on_select_chapter;
    let on_edit_chapter = props.on_edit_chapter;
    let on_delete_chapter = props.on_delete_chapter;
    let on_add_subchapter = props.on_add_subchapter;
    
    rsx! {
        div {
            class: "chapter-list",

            if chapters().is_empty() {
                div {
                    class: "empty-state",
                    p { "暂无章节" }
                    p {
                        class: "hint",
                        "点击上方 \"新建章节\" 按钮开始创作"
                    }
                }
            } else {
                { 
                    let manager = ChapterManager::new();
                    let chapter_tree = manager.build_chapter_tree(chapters());
                    let root_nodes = chapter_tree.root_nodes.clone();
                    rsx! {
                        div {
                            class: "chapter-tree",
                            for root_id in root_nodes {
                                { render_chapter_node(
                                &chapter_tree,
                                root_id,
                                current_chapter_id,
                                on_select_chapter.clone(),
                                on_edit_chapter.clone(),
                                on_delete_chapter.clone(),
                                on_add_subchapter.clone()
                            ) }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_children(
    tree: &ChapterTree,
    children: Vec<i64>,
    current_chapter_id: Signal<Option<i64>>,
    on_select: EventHandler<i64>,
    on_edit: EventHandler<Chapter>,
    on_delete: EventHandler<i64>,
    on_add_sub: EventHandler<i64>,
) -> Element {
    if children.is_empty() {
        rsx! { div {} }
    } else {
        rsx! {
            div {
                class: "chapter-children",
                for child_id in children {
                    { render_chapter_node(
                        tree,
                        child_id,
                        current_chapter_id.clone(),
                        on_select.clone(),
                        on_edit.clone(),
                        on_delete.clone(),
                        on_add_sub.clone()
                    ) }
                }
            }
        }
    }
}

fn render_chapter_node(
    tree: &ChapterTree,
    node_id: i64,
    current_chapter_id: Signal<Option<i64>>,
    on_select: EventHandler<i64>,
    on_edit: EventHandler<Chapter>,
    on_delete: EventHandler<i64>,
    on_add_sub: EventHandler<i64>,
) -> Element {
    if let Some(node) = tree.nodes.get(&node_id) {
        let chapter = node.chapter.clone();
        let depth = node.depth;
        let has_children = !node.children.is_empty();
        let chapter_id = chapter.id;
        let chapter_title = chapter.title.clone();
        let chapter_word_count = chapter.word_count;
        let chapter_type_icon = get_chapter_type_icon(&chapter.chapter_type);
        let is_selected = current_chapter_id() == Some(chapter_id);
        let children = node.children.clone();
        
        // 改进缩进计算，提供更好的视觉层次感
        let indent_style = format!("margin-left: {}px", depth * 24);
        
        rsx! {
            div {
                key: "{chapter_id}",
                class: if is_selected { "chapter-node selected" } else { "chapter-node" },
                style: "{indent_style}",
                
                div {
                    class: "chapter-node-content",
                    onclick: {
                        let id = chapter_id;
                        move |_| on_select.call(id)
                    },
                    
                    // 章节信息区域
                    div {
                        span { 
                            class: "chapter-icon", 
                            "{chapter_type_icon}"
                        }
                        span { 
                            class: "chapter-title", 
                            "{chapter_title}"
                        }
                        
                        // 子章节数量指示器（如果有子章节）
                        {if has_children {
                            rsx! {
                                span {
                                    class: "chapter-child-count",
                                    style: "background: #e2e8f0; color: #475569; font-size: 0.75rem; padding: 0.1rem 0.4rem; border-radius: 12px;",
                                    "{children.len()}"
                                }
                            }
                        } else {
                            rsx! { div {} }
                        }}
                        
                        // 字数统计
                        span { 
                            class: "chapter-word-count", 
                            "{chapter_word_count} 字"
                        }
                    }
                    
                    // 操作按钮区域
                    div {            
                        class: "chapter-actions",
                        
                        // 常规模式下的操作按钮
                        button {
                            class: "action-btn",
                            title: "添加子章节",
                            onclick: {
                                let id = chapter_id;
                                move |e: Event<MouseData>| {
                                    e.stop_propagation();
                                    on_add_sub.call(id);
                                }
                            },
                            "➕"
                        }
                        button {
                            class: "action-btn",
                            title: "编辑",
                            onclick: {
                                let c = chapter.clone();
                                move |e: Event<MouseData>| {
                                    e.stop_propagation();
                                    on_edit.call(c.clone());
                                }
                            },
                            "✏️"
                        }
                        button {
                            class: "action-btn danger",
                            title: "删除",
                            onclick: {
                                let id = chapter_id;
                                move |e: Event<MouseData>| {
                                    e.stop_propagation();
                                    on_delete.call(id);
                                }
                            },
                            "🗑️"
                        }
                    }
                }
                
                // 递归渲染子章节
                {render_children(tree, children, current_chapter_id, on_select.clone(), 
                               on_edit.clone(), on_delete.clone(), on_add_sub.clone())}
            }
        }
    } else {
        rsx! { div {} }
    }
}

fn get_chapter_type_icon(chapter_type: &ChapterType) -> &'static str {
    match chapter_type {
        ChapterType::Volume => "📚",
        ChapterType::Chapter => "📄",
        ChapterType::Scene => "🎬",
    }
}