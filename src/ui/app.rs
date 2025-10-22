use dioxus::prelude::*;
use crate::db;
use web_sys;
use super::components::{Header, Sidebar, NovelForm, StatusBar, ChapterManagement};
use super::components::novel_management::NovelManagement;
use super::components::inspiration_management::InspirationManagement;
use super::components::inspiration_stats::InspirationStatsView;
use super::components::stats_view::StatsView;
use super::components::settings_view::SettingsView;

#[component]
pub fn App() -> Element {
    let current_view = use_signal(|| "novels".to_string());
    let mut current_novel_id = use_signal(|| None::<i64>);
    let search_query = use_signal(|| "".to_string());
    let mut novels = use_signal(|| Vec::<db::Novel>::new());
    let mut novel_title = use_signal(|| "".to_string());
    let mut novel_author = use_signal(|| "".to_string());
    let mut novel_description = use_signal(|| "".to_string());
    let mut show_novel_form = use_signal(|| false);
    let mut editing_novel = use_signal(|| None::<db::Novel>);
    
    // 灵感统计相关信号
    let mut inspiration_stats = use_signal(|| None::<crate::core::inspiration_manager::InspirationStats>);
    let inspiration_trends = use_signal(|| None::<crate::core::inspiration_manager::InspirationTrends>);
    let inspiration_recommendations = use_signal(|| Vec::<String>::new());
    
    // 写作统计相关信号
    let mut writing_report = use_signal(|| None::<crate::core::stats_manager::WritingReport>);
    
    // 加载小说列表
    use_effect(move || {
        log::debug!("开始加载小说列表");
        match db::get_database() {
            Ok(db) => {
                match db.get_all_novels() {
                    Ok(mut novels_list) => {
                        log::debug!("成功加载{}部小说", novels_list.len());
                        
                        // 按更新时间倒序排序
                        novels_list.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
                        novels.set(novels_list.clone());
                        
                        // 自动选择最新的小说
                        if let Some(latest_novel) = novels_list.first() {
                            log::debug!("自动选择最新小说: ID={}, title='{}'", latest_novel.id, latest_novel.title);
                            current_novel_id.set(Some(latest_novel.id));
                        }
                    },
                    Err(e) => {
                        log::error!("加载小说列表失败: {}", e);
                    }
                }
            },
            Err(e) => {
                log::error!("获取数据库连接失败: {}", e);
            }
        }
    });
    
    // 编辑小说
    
    // 编辑小说
    let edit_novel = move |novel: db::Novel| {
        novel_title.set(novel.title.clone());
        novel_author.set(novel.author.clone());
        novel_description.set(novel.description.clone());
        editing_novel.set(Some(novel));
        show_novel_form.set(true);
    };
    
    // 处理小说表单提交
    let handle_novel_submit = move |_| {
        log::debug!("开始处理小说表单提交");
        match db::get_database() {
            Ok(db) => {
                if !novel_title().is_empty() {
                    if let Some(editing) = editing_novel() {
                        // 更新现有小说
                        let mut updated_novel = editing.clone();
                        updated_novel.title = novel_title();
                        updated_novel.author = novel_author();
                        updated_novel.description = novel_description();
                        updated_novel.updated_at = chrono::Utc::now();
                        
                        log::debug!("更新小说ID: {}", updated_novel.id);
                        match db.update_novel(&updated_novel) {
                            Ok(()) => {
                                log::debug!("小说更新成功");
                                // 重新加载列表
                                if let Ok(novels_list) = db.get_all_novels() {
                                    log::debug!("更新后加载{}部小说", novels_list.len());
                                    novels.set(novels_list);
                                }
                                
                                // 重置表单
                                novel_title.set("".to_string());
                                novel_author.set("".to_string());
                                novel_description.set("".to_string());
                                show_novel_form.set(false);
                                editing_novel.set(None);
                            },
                            Err(e) => {
                                log::error!("更新小说失败: {}", e);
                            }
                        }
                    } else {
                        // 创建新小说
                        log::debug!("创建新小说: {}", novel_title());
                        match db.create_novel(&novel_title()) {
                            Ok(novel) => {
                                log::debug!("小说创建成功，ID: {}", novel.id);
                                // 更新作者和描述
                                let mut updated_novel = novel.clone();
                                updated_novel.author = novel_author();
                                updated_novel.description = novel_description();
                                
                                match db.update_novel(&updated_novel) {
                                    Ok(()) => log::debug!("小说详情更新成功"),
                                    Err(e) => log::error!("更新小说详情失败: {}", e)
                                }
                                
                                // 重新加载列表
                                if let Ok(novels_list) = db.get_all_novels() {
                                    log::debug!("创建后加载{}部小说", novels_list.len());
                                    novels.set(novels_list);
                                }
                                
                                // 重置表单
                                novel_title.set("".to_string());
                                novel_author.set("".to_string());
                                novel_description.set("".to_string());
                                show_novel_form.set(false);
                                editing_novel.set(None);
                            },
                            Err(e) => {
                                log::error!("创建小说失败: {}", e);
                            }
                        }
                    }
                }
            },
            Err(e) => {
                log::error!("获取数据库连接失败: {}", e);
            }
        }
    };
    
    // 删除小说
    let delete_novel = move |novel_id: i64| {
        log::debug!("删除小说ID: {}", novel_id);
        match db::get_database() {
            Ok(db) => {
                match db.delete_novel(novel_id) {
                    Ok(()) => {
                        log::debug!("小说删除成功");
                        // 重新加载列表
                        if let Ok(novels_list) = db.get_all_novels() {
                            log::debug!("删除后加载{}部小说", novels_list.len());
                            novels.set(novels_list);
                        }
                        
                        // 如果删除的是当前选中的小说，清空选择
                        if current_novel_id() == Some(novel_id) {
                            current_novel_id.set(None);
                        }
                    },
                    Err(e) => {
                        log::error!("删除小说失败: {}", e);
                    }
                }
            },
            Err(e) => {
                log::error!("获取数据库连接失败: {}", e);
            }
        }
    };
    
    // 选择小说
    let select_novel = move |novel_id: i64| {
        current_novel_id.set(Some(novel_id));
        
        // 简单设置状态，避免使用spawn_local
        log::debug!("选择小说，ID: {}", novel_id);
        
        // 由于generate_writing_report是异步方法，我们需要特殊处理
        // 为了避免Tokio运行时错误，这里暂时只设置当前小说ID
        // 后续我们可以在StatsView组件内部实现数据加载逻辑
        log::debug!("已选择小说，ID: {}", novel_id);
        // 注意：generate_writing_report是异步方法，需要在正确的异步上下文中调用
    };
    
    // 显示新建小说表单
    let show_new_novel_form = move |_| {
        show_novel_form.set(true);
        editing_novel.set(None);
        novel_title.set("".to_string());
        novel_author.set("".to_string());
        novel_description.set("".to_string());
    };
    
    // 处理小说置顶/取消置顶
    let toggle_novel_pin = move |args: (i64, String)| {
        let (novel_id, current_title) = args;
        log::debug!("切换小说置顶状态，ID: {}", novel_id);
        
        // 在Web环境中，我们需要创建一个新的异步任务来处理这个操作
        // 由于在组件内部使用spawn_local可能会导致运行时错误，我们使用简单的方法处理
        match db::get_database() {
            Ok(db) => {
                // 直接在主线程中处理置顶逻辑（简化版本）
                if let Some(mut novel) = match db.get_novel_by_id(novel_id) {
                    Ok(novel) => novel,
                    Err(e) => {
                        log::error!("获取小说失败: {}", e);
                        None
                    }
                } {
                    let current_pinned = novel.is_pinned;
                    
                    if !current_pinned {
                        // 检查当前置顶小说数量
                        if let Ok(all_novels) = db.get_all_novels() {
                            let pinned_count = all_novels.iter().filter(|n| n.is_pinned).count();
                            
                            if pinned_count >= 3 {
                                log::warn!("置顶数量已达上限");
                                if let Some(window) = web_sys::window() {
                                    window.alert_with_message("最多只能置顶3本小说").unwrap_or(());
                                }
                                return;
                            }
                            
                            // 设置为置顶
                            novel.is_pinned = true;
                            novel.pinned_order = Some((pinned_count + 1) as i32);
                            log::debug!("置顶小说，ID: {}, 顺序: {}", novel_id, pinned_count + 1);
                        }
                    } else {
                        // 取消置顶
                        novel.is_pinned = false;
                        novel.pinned_order = None;
                        log::debug!("取消置顶小说，ID: {}", novel_id);
                    }
                    
                    novel.updated_at = chrono::Utc::now();
                    
                    // 更新数据库
                    if let Ok(()) = db.update_novel(&novel) {
                        log::debug!("小说置顶状态更新成功");
                        
                        // 重新加载小说列表
                        if let Ok(novels_list) = db.get_all_novels() {
                            novels.set(novels_list);
                        }
                        
                        // 显示操作结果
                        if let Some(window) = web_sys::window() {
                            if novel.is_pinned {
                                window.alert_with_message(&format!("已置顶《{}》", current_title)).unwrap_or(())
                            } else {
                                window.alert_with_message(&format!("已取消《{}》的置顶", current_title)).unwrap_or(())
                            }
                        }
                    } else {
                        log::error!("更新小说置顶状态失败");
                    }
                }
            },
            Err(e) => {
                log::error!("获取数据库连接失败: {}", e);
            }
        }
    };
    
    // 取消表单
    let cancel_form = move |_| {
        show_novel_form.set(false);
        editing_novel.set(None);
        novel_title.set("".to_string());
        novel_author.set("".to_string());
        novel_description.set("".to_string());
    };
    
    rsx! {
        div {
            class: "app-container",
            
            // 顶部导航栏
            Header {
                search_query: search_query.clone(),
            }
            
            // 主内容区域
            div {
                class: "main-content",
                
                // 侧边栏
                Sidebar {
                    current_view: current_view.clone(),
                    current_novel_id: current_novel_id.clone(),
                    novels: novels.clone(),
                    on_select_novel: select_novel,
                    on_edit_novel: edit_novel,
                    on_delete_novel: delete_novel,
                    on_new_novel: show_new_novel_form,
                    on_toggle_pin: toggle_novel_pin,
                }
                
                // 主工作区
                main {
                    class: "workspace",
                    
                    // 小说表单
                    if show_novel_form() {
                        NovelForm {
                            novel_title: novel_title.clone(),
                            novel_author: novel_author.clone(),
                            novel_description: novel_description.clone(),
                            editing_novel: editing_novel.clone(),
                            on_submit: handle_novel_submit,
                            on_cancel: cancel_form,
                        }
                    }
                    
                    // 根据当前视图显示不同内容
                    else if current_view() == "novels" {
                        NovelManagement {
                        novels: novels.clone(),
                        current_novel_id: current_novel_id.clone(),
                        on_select_novel: select_novel,
                        on_edit_novel: edit_novel,
                        on_delete_novel: delete_novel,
                        show_novel_form: show_novel_form.clone(),
                        novel_title: novel_title.clone(),
                        novel_author: novel_author.clone(),
                        novel_description: novel_description.clone(),
                        editing_novel: editing_novel.clone(),
                        on_novel_submit: handle_novel_submit,
                        on_cancel_form: cancel_form,
                        on_add_novel: show_new_novel_form,
                        on_toggle_pin: toggle_novel_pin,
                    }
                    } else if current_view() == "chapters" {
                        ChapterManagement {
                            current_novel_id: current_novel_id.clone(),
                            novels: novels.clone(),
                        }
                    } else if current_view() == "inspirations" {
                        if current_novel_id().is_some() {
                            InspirationManagement {
                                current_novel_id: current_novel_id.clone(),
                                novels: novels.clone(),
                            }
                        } else {
                            div {
                                class: "inspiration-management",
                                p { "请先从左侧选择一部小说" }
                            }
                        }
                    } else if current_view() == "stats" {
                        if let Some(novel_id) = current_novel_id() {
                            if let Some(novel) = novels().iter().find(|n| n.id == novel_id) {
                                div { 
                                    class: "stats-container",
                                    div { 
                                        class: "stats-section writing-stats",
                                        StatsView { writing_report: writing_report.clone() }
                                    }
                                    
                                    div { 
                                        class: "stats-section inspiration-stats",
                                        h3 { "💡 灵感统计" }
                                        InspirationStatsView {
                                            stats: inspiration_stats.clone(),
                                            trends: inspiration_trends.clone(),
                                            recommendations: inspiration_recommendations.clone(),
                                        }
                                    }
                                }
                            }
                        } else {
                            div { 
                                class: "stats-view",
                                h2 { "📊 统计信息" }
                                p { "请先从左侧选择一部小说" }
                            }
                        }
                    } else if current_view() == "settings" {
                        SettingsView {}
                    } else {
                        div { "未知视图" }
                    }
                }
            }
            
            // 底部状态栏
            StatusBar {
                current_novel_id: current_novel_id.clone(),
                novels: novels.clone(),
            }
            
            // 内联样式
            style { {include_str!("../../assets/styles.css")} }
        }
    }
}