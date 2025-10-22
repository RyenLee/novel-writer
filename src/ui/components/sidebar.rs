use dioxus::prelude::*;
use crate::db::Novel;

#[derive(Props, Clone, PartialEq)]
pub struct SidebarProps {
    pub current_view: Signal<String>,
    novels: Signal<Vec<Novel>>,
    current_novel_id: Signal<Option<i64>>,
    on_select_novel: EventHandler<i64>,
    on_new_novel: EventHandler<()>,
    on_edit_novel: EventHandler<Novel>,
    on_delete_novel: EventHandler<i64>,
    on_toggle_pin: EventHandler<(i64, String)>,
    on_import_novel: Option<EventHandler<()>>,
    on_export_novel: Option<EventHandler<()>>,
}

#[component]
pub fn Sidebar(props: SidebarProps) -> Element {
    let mut current_view = props.current_view;
    let novels = props.novels;
    let current_novel_id = props.current_novel_id;
    let on_select_novel = props.on_select_novel;
    let on_new_novel = props.on_new_novel;
    let on_edit_novel = props.on_edit_novel;
    let on_delete_novel = props.on_delete_novel;
    let on_toggle_pin = props.on_toggle_pin;
    let on_import_novel = props.on_import_novel;
    let on_export_novel = props.on_export_novel;

    // 排序小说：先按置顶状态排序，然后按创建时间排序（最新的在前）
    let novels_val = novels.read();
    let mut display_novels = novels_val.clone();
    display_novels.sort_by(|a, b| {
        if a.is_pinned != b.is_pinned {
            a.is_pinned.cmp(&b.is_pinned).reverse()
        } else {
            b.created_at.cmp(&a.created_at)
        }
    });
    
    // 获取当前选中的小说ID
    let current_id = *current_novel_id.read();

    rsx! {
        div {
            class: "sidebar",
            // 侧边栏标题
            div {
                class: "sidebar-header",
                h1 {
                    class: "sidebar-title",
                    span {
                        class: "sidebar-title-icon",
                        "📝"
                    }
                    "我的小说库"
                }
            }
            
            // 导航菜单
            div {
                class: "nav-section",
                ul {
                    class: "nav-list",
                    // 小说管理导航项
                    li {
                        class: "nav-item",
                        button {
                            class: if current_view() == "novels" { "nav-link active" } else { "nav-link" },
                            onclick: move |_| current_view.set("novels".to_string()),
                            span { "📚" }
                            span { "小说管理" }
                        }
                    }
                    // 章节管理导航项
                    li {
                        class: "nav-item",
                        button {
                            class: if current_view() == "chapters" { "nav-link active" } else { "nav-link" },
                            onclick: move |_| current_view.set("chapters".to_string()),
                            span { "📑" }
                            span { "章节管理" }
                        }
                    }
                    // 灵感管理导航项
                    // li {
                    //     class: "nav-item",
                    //     button {
                    //         class: if current_view() == "inspiration" { "nav-link active" } else { "nav-link" },
                    //         onclick: move |_| current_view.set("inspiration".to_string()),
                    //         span { "💡" }
                    //         span { "灵感管理" }
                    //     }
                    // }

                    // 写作统计导航项
                    // li {
                    //     class: "nav-item",
                    //     button {
                    //         class: if current_view() == "stats" { "nav-link active" } else { "nav-link" },
                    //         onclick: move |_| current_view.set("stats".to_string()),
                    //         span { "📊" }
                    //         span { "写作统计" }
                    //     }
                    // }
                    // 系统设置导航项
                    // li {
                    //     class: "nav-item",
                    //     button {
                    //         class: if current_view() == "settings" { "nav-link active" } else { "nav-link" },
                    //         onclick: move |_| current_view.set("settings".to_string()),
                    //         span { "⚙️" }
                    //         span { "系统设置" }
                    //     }
                    // }
                }
            }
        }
    }
}