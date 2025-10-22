/// 章节管理主组件
use dioxus::prelude::*;
use crate::db::{self, Chapter, ChapterType};
use crate::core::chapter_manager::ChapterManager;
use super::{ChapterList, ChapterForm, Editor};
use log::{info, warn, error};
use tokio::task::spawn_local;

#[derive(Props, Clone, PartialEq)]
pub struct ChapterManagementProps {
    pub current_novel_id: Signal<Option<i64>>,
    pub novels: Signal<Vec<db::Novel>>,
}

#[component]
pub fn ChapterManagement(props: ChapterManagementProps) -> Element {
    let current_novel_id = props.current_novel_id;
    let novels = props.novels;
    
    // 章节相关状态
    let mut chapters = use_signal(|| Vec::<Chapter>::new());
    let mut current_selected_chapter_id = use_signal(|| None::<i64>);
    let mut current_chapter = use_signal(|| None::<Chapter>);
    let mut chapter_content = use_signal(|| String::new());
    
    // 表单状态
    let mut show_chapter_form = use_signal(|| false);
    let mut editing_chapter = use_signal(|| None::<Chapter>);
    let mut parent_chapter_id = use_signal(|| None::<i64>);
    let mut chapter_title = use_signal(|| String::new());
    let mut chapter_type = use_signal(|| ChapterType::Chapter);
    
    // 删除确认状态
    let mut show_delete_confirm = use_signal(|| false);
    let mut chapter_to_delete = use_signal(|| None::<i64>);
    
    // 加载章节列表
    use_effect(move || {
        if let Some(novel_id) = current_novel_id() {
            if let Ok(db) = db::get_database() {
                if let Ok(chapters_list) = db.get_chapters_by_novel(novel_id) {
                    chapters.set(chapters_list);
                }
            }
        } else {
            chapters.set(Vec::new());
            current_selected_chapter_id.set(None);
            current_chapter.set(None);
        }
    });
    
    // 选择章节
    let select_chapter = move |chapter_id: i64| {
        current_selected_chapter_id.set(Some(chapter_id));
        if let Some(chapter) = chapters().iter().find(|c| c.id == chapter_id) {
            current_chapter.set(Some(chapter.clone()));
            chapter_content.set(chapter.content.clone());
        }
    };
    
    // 新建章节
    let show_new_chapter_form = move |_| {
        editing_chapter.set(None);
        parent_chapter_id.set(None);
        chapter_title.set(String::new());
        chapter_type.set(ChapterType::Chapter);
        show_chapter_form.set(true);
    };
    
    // 添加子章节
    let add_subchapter = move |parent_id: i64| {
        editing_chapter.set(None);
        parent_chapter_id.set(Some(parent_id));
        chapter_title.set(String::new());
        chapter_type.set(ChapterType::Chapter);
        show_chapter_form.set(true);
    };
    
    // 编辑章节
    let edit_chapter = move |chapter: Chapter| {
        editing_chapter.set(Some(chapter.clone()));
        parent_chapter_id.set(chapter.parent_id);
        chapter_title.set(chapter.title.clone());
        chapter_type.set(chapter.chapter_type.clone());
        show_chapter_form.set(true);
    };
    
    // 显示删除确认
    let confirm_delete = move |chapter_id: i64| {
        chapter_to_delete.set(Some(chapter_id));
        show_delete_confirm.set(true);
    };
    
    // 执行删除
    let execute_delete = move |_| {
        if let Some(chapter_id) = chapter_to_delete() {
            info!("执行章节删除: id={}", chapter_id);
            if let Ok(db) = db::get_database() {
                if let Ok(()) = db.delete_chapter(chapter_id) {
                    info!("章节删除成功: id={}", chapter_id);
                    // 重新加载章节列表
                    if let Some(novel_id) = current_novel_id() {
                        if let Ok(chapters_list) = db.get_chapters_by_novel(novel_id) {
                            chapters.set(chapters_list);
                        }
                    }
                    
                    // 如果删除的是当前章节,清空选择
                    if current_selected_chapter_id() == Some(chapter_id) {
                        current_selected_chapter_id.set(None);
                        current_chapter.set(None);
                        chapter_content.set(String::new());
                    }
                } else {
                    error!("章节删除失败: id={}", chapter_id);
                }
            }
        }
        show_delete_confirm.set(false);
        chapter_to_delete.set(None);
    };
    
    // 移动章节功能已移除
    
    // 取消删除
    let cancel_delete = move |_| {
        show_delete_confirm.set(false);
        chapter_to_delete.set(None);
    };
    
    // 提交章节表单
    let handle_chapter_submit = move |_| {
        if let Ok(db) = db::get_database() {
            if !chapter_title().trim().is_empty() {
                if let Some(novel_id) = current_novel_id() {
                    if let Some(editing) = editing_chapter() {
                        // 更新章节
                        let mut updated_chapter = editing.clone();
                        updated_chapter.title = chapter_title();
                        updated_chapter.chapter_type = chapter_type();
                        
                        if let Ok(()) = db.update_chapter(&updated_chapter) {
                            // 重新加载列表
                            if let Ok(chapters_list) = db.get_chapters_by_novel(novel_id) {
                                chapters.set(chapters_list);
                            }
                            
                            // 更新当前章节
                            if current_selected_chapter_id() == Some(updated_chapter.id) {
                                current_chapter.set(Some(updated_chapter));
                            }
                            
                            show_chapter_form.set(false);
                        }
                    } else {
                        // 创建新章节
                        if let Ok(mut chapter) = db.create_chapter(
                            novel_id,
                            &chapter_title(),
                            parent_chapter_id()
                        ) {
                            chapter.chapter_type = chapter_type();
                            let _ = db.update_chapter(&chapter);
                            
                            // 重新加载列表
                            if let Ok(chapters_list) = db.get_chapters_by_novel(novel_id) {
                                chapters.set(chapters_list);
                            }
                            
                            show_chapter_form.set(false);
                        }
                    }
                }
            }
        }
    };
    
    // 取消表单
    let cancel_form = move |_| {
        show_chapter_form.set(false);
        editing_chapter.set(None);
        parent_chapter_id.set(None);
        chapter_title.set(String::new());
        chapter_type.set(ChapterType::Chapter);
    };
    
    // 保存章节内容
    let save_chapter = move |_| {
        if let Some(chapter) = current_chapter() {
            if let Ok(db) = db::get_database() {
                if let Ok(()) = db.update_chapter_content(chapter.id, &chapter_content()) {
                    // 更新章节信息
                    if let Ok(updated_chapter) = db.get_chapter(chapter.id) {
                        current_chapter.set(Some(updated_chapter.clone()));
                        
                        // 更新列表中的章节
                        if let Some(novel_id) = current_novel_id() {
                            if let Ok(chapters_list) = db.get_chapters_by_novel(novel_id) {
                                chapters.set(chapters_list);
                            }
                        }
                    }
                }
            }
        }
    };
    
    rsx! {
        div {
            class: "chapter-management",
            
            if let Some(novel_id) = current_novel_id() {
                if let Some(novel) = novels().iter().find(|n| n.id == novel_id) {
                    div {
                        class: "chapter-management-content",
                        
                        // 左侧章节列表
                        div {
                            class: "chapter-sidebar",
                            div {
                                class: "chapter-sidebar-header",
                                h3 {
                                    span { class: "material-icons", "article" }
                                    "📑 {novel.title}"
                                }
                                button {
                                    class: "btn btn-primary",
                                    onclick: show_new_chapter_form,
                                    span { class: "material-icons", "add" }
                                    "新建章节"
                                }
                            }
                            div {
                                class: "chapter-list",

                                ChapterList {
                                    chapters: chapters,
                                    current_chapter_id: current_selected_chapter_id,
                                    on_select_chapter: select_chapter,
                                    on_edit_chapter: edit_chapter,
                                    on_delete_chapter: confirm_delete,
                                    on_add_subchapter: add_subchapter,
                                }
                            }
                        }
                        
                        // 右侧编辑器
                        div {
                            class: "chapter-main",
                            Editor {
                                current_chapter: current_chapter,
                                chapter_content: chapter_content,
                                on_save: save_chapter,
                            }
                        }
                        
                        // 章节表单弹窗
                        if show_chapter_form() {
                            div {
                                class: "chapter-form-overlay",
                                onclick: cancel_form,
                                
                                div {
                                    class: "chapter-form",
                                    onclick: move |e: Event<MouseData>| e.stop_propagation(),
                                    
                                    h3 { if editing_chapter().is_some() { "编辑章节" } else { "新建章节" } }
                                    div {
                                        class: "form-group",
                                        label { "章节标题: " }
                                        input {
                                            placeholder: "输入章节标题",
                                            value: chapter_title(),
                                            oninput: move |evt| chapter_title.set(evt.value().clone()),
                                        }
                                    }
                                    div {
                                        class: "form-group",
                                        label { "章节类型: " }
                                        select {
                                            value: format!("{:?}", chapter_type()),
                                            onchange: move |evt| {
                                                match evt.value().as_str() {
                                                    "Volume" => chapter_type.set(ChapterType::Volume),
                                                    "Chapter" => chapter_type.set(ChapterType::Chapter),
                                                    "Scene" => chapter_type.set(ChapterType::Scene),
                                                    _ => chapter_type.set(ChapterType::Chapter),
                                                }
                                            },
                                            option { value: "Volume", "卷" }
                                            option { value: "Chapter", "章节" }
                                            option { value: "Scene", "场景" }
                                        }
                                    }
                                    div {
                                        class: "form-actions",
                                        button {
                                            class: "btn btn-secondary",
                                            onclick: cancel_form,
                                            "取消"
                                        }
                                        button {
                                            class: "btn btn-primary",
                                            onclick: handle_chapter_submit,
                                            if editing_chapter().is_some() { "保存" } else { "创建" }
                                        }
                                    }
                                }
                            }
                        }
                        
                        // 删除确认对话框
                        if show_delete_confirm() {
                            div {
                                class: "chapter-form-overlay",
                                onclick: cancel_delete,
                                
                                div {
                                    class: "chapter-form",
                                    onclick: move |e: Event<MouseData>| e.stop_propagation(),
                                    
                                    h3 { "确认删除章节" }
                                    div { class: "warning-text", "⚠️ 删除章节后无法恢复，所有子章节也将被删除。确认继续吗？" }
                                    
                                    div { class: "form-actions",
                                        button {
                                            class: "btn btn-secondary",
                                            onclick: cancel_delete,
                                            "取消"
                                        }
                                        button {
                                            class: "btn btn-danger",
                                            onclick: execute_delete,
                                            "确认删除"
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div {
                        class: "chapter-empty-state",
                        p { "小说不存在" }
                    }
                }
            } else {
                div {
                    class: "chapter-empty-state",
                    p { "请先从左侧选择一部小说" }
                }
            }
        }
    }
}