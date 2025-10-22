use dioxus::prelude::*;
use std::collections::HashMap;
use std::result::Result as StdResult;
use std::rc::Rc;

use crate::db;
use crate::db::Novel;
use crate::core::inspiration_manager::Inspiration;

// 为Result提供类型别名
type Result<T> = StdResult<T, Box<dyn std::error::Error>>;

// 灵感视图模式枚举
#[derive(PartialEq, Clone, Copy)]
pub enum InspirationViewMode {
    List,
    Form,
}

// 灵感表单数据结构
#[derive(Clone)]
pub struct InspirationFormData {
    pub title: String,
    pub content: String,
    pub tags: String,
}

// 灵感管理组件
#[derive(Props, Clone, PartialEq)]
pub struct InspirationManagementProps {
    pub novels: Signal<Vec<Novel>>,
    pub current_novel_id: Signal<Option<i64>>,
}

// 导入InspirationList工具类
pub struct InspirationList;

impl InspirationList {
    pub fn new() -> Self {
        Self
    }
    
    // 获取小说灵感列表
    pub fn get_inspirations(&self, novel_id: i64) -> Result<Vec<Inspiration>> {
        let db = db::get_database()?;
        db.get_inspirations_by_novel(novel_id).map_err(|e| e.into())
    }
    
    // 创建新灵感
    pub fn create_inspiration(&self, novel_id: i64, title: &str, content: &str) -> Result<Inspiration> {
        let db = db::get_database()?;
        db.create_inspiration(novel_id, title, content).map_err(|e| e.into())
    }
    
    // 更新灵感
    pub fn update_inspiration(&self, inspiration_id: i64, title: &str, content: &str) -> Result<()> {
        let db = db::get_database()?;
        db.update_inspiration(inspiration_id, title, content).map_err(|e| e.into())
    }
    
    // 删除灵感
    pub fn delete_inspiration(&self, inspiration_id: i64) -> Result<()> {
        let db = db::get_database()?;
        db.delete_inspiration(inspiration_id).map_err(|e| e.into())
    }
    
    // 切换置顶状态
    pub fn toggle_pin(&self, inspiration_id: i64) -> Result<bool> {
        let db = db::get_database()?;
        db.toggle_inspiration_pin(inspiration_id).map_err(|e| e.into())
    }
    
    // 添加标签
    pub fn add_tags(&self, inspiration_id: i64, tags: &[String]) -> Result<()> {
        let db = db::get_database()?;
        db.add_inspiration_tags(inspiration_id, tags).map_err(|e| e.into())
    }
}

// 工具函数：格式化日期
fn format_date(date_str: &str) -> String {
    match chrono::DateTime::parse_from_rfc3339(date_str) {
        Ok(date) => {
            let date_utc = date.with_timezone(&chrono::Utc);
            date_utc.format("%Y-%m-%d %H:%M:%S").to_string()
        },
        Err(_) => date_str.to_string(),
    }
}

// 工具函数：截断文本
fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        text[..max_length].to_string() + "..."
    }
}

// 灵感项组件
#[component]
pub fn InspirationItem(
    inspiration: Rc<Inspiration>,
    on_toggle_pin: Callback<i64>,
    on_edit: Callback<Inspiration>,
    on_delete: Callback<i64>,
) -> Element {
    let is_pinned = inspiration.is_pinned;
    
    // 增强的截断文本函数，更好地处理UTF-8字符
    let truncate_text = |text: &str, max_length: usize| -> String {
        let mut chars = text.chars();
        let mut result = String::new();
        let mut count = 0;
        
        while let Some(c) = chars.next() {
            if count >= max_length {
                result.push_str("...");
                break;
            }
            result.push(c);
            count += 1;
        }
        
        result
    };
    
    // 标题截断显示，鼠标悬停时显示完整标题
    let truncated_title = truncate_text(&inspiration.title, 40);
    let full_title = inspiration.title.clone();
    
    // 内容截断显示，鼠标悬停时显示完整内容的提示
    let truncated_content = truncate_text(&inspiration.content, 120);
    let full_content = inspiration.content.clone();
    
    // 直接使用created_at字符串，因为它已经是格式化的字符串
    let formatted_date = inspiration.created_at.clone();
    
    rsx! {
        div {
            class: "inspiration-item",
            // 置顶标记和主要内容区域
            div {
                class: "inspiration-header",
                if is_pinned {
                    span { 
                        class: "pin-icon", 
                        title: "已置顶",
                        "📌" 
                    }
                }
                div {
                    class: "title-section",
                    h3 {
                        class: "inspiration-title",
                        title: full_title,
                        {truncated_title}
                    }
                    span {
                        class: "inspiration-date",
                        {formatted_date}
                    }
                }
            }
            
            // 内容区域
            div {
                class: "inspiration-body",
                p {
                    class: "inspiration-text",
                    title: full_content,
                    {truncated_content}
                }
            }
            
            // 标签区域
            if !inspiration.tags.is_empty() {
                div {
                    class: "inspiration-tags",
                    for tag in &inspiration.tags {
                        span {
                            class: "tag",
                            title: tag.clone(),
                            {tag.clone()}
                        }
                    }
                }
            }
            
            // 操作按钮区域
            div {
                class: "inspiration-actions",
                button {
                    class: "action-button pin-button",
                    onclick: { 
                        let id = inspiration.id as i64;
                        move |_| on_toggle_pin.call(id)
                    },
                    title: if is_pinned { "取消置顶" } else { "置顶" },
                    {if is_pinned { "取消置顶" } else { "置顶" }}
                }
                button {
                    class: "action-button edit-button",
                    onclick: { 
                        let id = inspiration.id;
                        let novel_id = inspiration.novel_id;
                        let title = inspiration.title.clone();
                        let content = inspiration.content.clone();
                        let created_at = inspiration.created_at.clone();
                        let updated_at = inspiration.updated_at.clone();
                        let is_pinned = inspiration.is_pinned;
                        let tags = inspiration.tags.clone();
                        let linked_chapters = inspiration.linked_chapters.clone();
                        move |_| {
                            on_edit.call(Inspiration {
                                id,
                                novel_id,
                                title: title.clone(),
                                content: content.clone(),
                                created_at: created_at.clone(),
                                updated_at: updated_at.clone(),
                                is_pinned,
                                tags: tags.clone(),
                                linked_chapters: linked_chapters.clone(),
                            });
                        }
                    },
                    title: "编辑",
                    "编辑"
                }
                button {
                    class: "action-button delete-button",
                    onclick: { 
                        let id = inspiration.id as i64;
                        move |_| on_delete.call(id)
                    },
                    title: "删除",
                    "删除"
                }
            }
        }
    }
}

// 标签按钮组件
#[component]
fn TagButton(
    tag: String,
    count: usize,
    selected_tag: Option<String>,
    on_click: EventHandler<String>,
) -> Element {
    let is_selected = selected_tag == Some(tag.clone());
    let class = if is_selected {
        "tag-button selected"
    } else {
        "tag-button"
    };
    
    rsx! {
        button {
            class: class,
            onclick: move |_| on_click.call(tag.clone()),
            "{tag} ({count})"
        }
    }
}

#[allow(non_snake_case)]
#[component]
pub fn InspirationManagement(props: InspirationManagementProps) -> Element {
    // 状态管理
    // 不再需要novels变量
    let current_novel_id = props.current_novel_id;
    let inspirations = use_signal(|| Vec::<Inspiration>::new());
    let mut view_mode = use_signal(|| InspirationViewMode::List);
    let mut editing_inspiration = use_signal(|| None::<Inspiration>);
    let mut form_data = use_signal(|| InspirationFormData {
        title: String::new(),
        content: String::new(),
        tags: String::new(),
    });
    let mut search_query = use_signal(String::new);
    let mut selected_tag = use_signal(|| None::<String>);
    let filtered_inspirations = use_signal(|| Vec::<Inspiration>::new());
    let all_tags = use_signal(|| Vec::<(String, usize)>::new());
    
    // 加载灵感列表
    let load_inspirations = move |novel_id: i64| {
        let mut inspirations_clone = inspirations.clone();
        let mut filtered_inspirations = filtered_inspirations.clone();
        let mut all_tags = all_tags.clone();
        let search_query = search_query.clone();
        let selected_tag = selected_tag.clone();
        
        // 直接同步执行
        match InspirationList::new().get_inspirations(novel_id) {
            Ok(result) => {
                inspirations_clone.set(result.clone());
                
                // 简单的过滤逻辑
                let query = search_query();
                let tag = selected_tag();
                let filtered = result
                    .into_iter()
                    .filter(|insp| {
                        let query_match = query.is_empty() || 
                            insp.title.to_lowercase().contains(&query.to_lowercase()) ||
                            insp.content.to_lowercase().contains(&query.to_lowercase());
                        
                        let tag_match = tag.is_none() || 
                            insp.tags.contains(&tag.as_ref().unwrap());
                        
                        query_match && tag_match
                    })
                    .collect();
                
                filtered_inspirations.set(filtered);
                
                // 提取所有标签
                let mut tag_counts = HashMap::new();
                for insp in &inspirations_clone() {
                    for tag in &insp.tags {
                        *tag_counts.entry(tag.clone()).or_insert(0) += 1;
                    }
                }
                
                let tags_with_counts: Vec<_> = tag_counts
                    .iter()
                    .map(|(tag, count)| (tag.clone(), *count))
                    .collect();
                
                all_tags.set(tags_with_counts);
            },
            Err(e) => {
                println!("加载灵感失败: {}", e);
            }
        }
    };
    
    // 处理小说切换
    use_effect(move || {
        if let Some(novel_id) = current_novel_id() {
            load_inspirations(novel_id);
        }
    });
    
    // 创建新灵感
    let handle_create_inspiration = move |_| {
        if let Some(novel_id) = current_novel_id() {
            let title = form_data.with(|data| data.title.clone());
            let content = form_data.with(|data| data.content.clone());
            let tags_str = form_data.with(|data| data.tags.clone());
            
            let mut form_data_clone = form_data.clone();
            let mut view_mode_clone = view_mode.clone();
            
            // 直接同步执行
              match InspirationList::new().create_inspiration(novel_id, &title, &content) {
                        Ok(new_inspiration) => {
                            // 添加标签
                            let tags: Vec<String> = tags_str
                                .split(",")
                                .map(|t| t.trim().to_string())
                                .filter(|t| !t.is_empty())
                                .collect();
                            
                            if !tags.is_empty() {
                                if let Err(e) = InspirationList::new().add_tags(new_inspiration.id, &tags) {
                                    println!("添加标签失败: {}", e);
                                }
                            }
                            
                            // 重新加载列表
                            load_inspirations(novel_id);
                            
                            // 重置表单
                            form_data_clone.set(InspirationFormData {
                                title: String::new(),
                                content: String::new(),
                                tags: String::new(),
                            });
                            
                            // 切换回列表视图
                            view_mode_clone.set(InspirationViewMode::List);
                        },
                        Err(e) => {
                            println!("创建灵感失败: {}", e);
                        }
                    }
        }
    };
    
    // 更新灵感
    let handle_update_inspiration = move |_| {
        if let Some(inspiration) = editing_inspiration() {
            let title = form_data.with(|data| data.title.clone());
            let content = form_data.with(|data| data.content.clone());
            let tags_str = form_data.with(|data| data.tags.clone());
            
            let mut editing_inspiration_clone = editing_inspiration.clone();
            let mut view_mode_clone = view_mode.clone();
            let novel_id = current_novel_id().unwrap_or(0);
            
            // 直接同步执行
              match InspirationList::new().update_inspiration(inspiration.id, &title, &content) {
                        Ok(_) => {
                            // 更新标签
                            let tags: Vec<String> = tags_str
                                .split(",")
                                .map(|t| t.trim().to_string())
                                .filter(|t| !t.is_empty())
                                .collect();
                            
                            if !tags.is_empty() {
                                if let Err(e) = InspirationList::new().add_tags(inspiration.id, &tags) {
                                    println!("更新标签失败: {}", e);
                                }
                            }
                            
                            // 重新加载列表
                            load_inspirations(novel_id);
                            
                            // 重置状态
                            editing_inspiration_clone.set(None);
                            view_mode_clone.set(InspirationViewMode::List);
                        },
                        Err(e) => {
                            println!("更新灵感失败: {}", e);
                        }
                    }
        }
    };
    
    // 删除灵感
    let handle_delete_inspiration = move |id: i64| {
        let novel_id = current_novel_id().unwrap_or(0);
        
        // 直接同步执行
        match InspirationList::new().delete_inspiration(id) {
            Ok(_) => {
                // 重新加载列表
                load_inspirations(novel_id);
            },
            Err(e) => {
                println!("删除灵感失败: {}", e);
            }
        }
    };
    
    // 切换置顶状态
    let handle_toggle_pin = move |id: i64| {
        let novel_id = current_novel_id().unwrap_or(0);
        
        // 直接同步执行
        match InspirationList::new().toggle_pin(id) {
            Ok(_) => {
                // 重新加载列表
                load_inspirations(novel_id);
            },
            Err(e) => {
                println!("切换置顶状态失败: {}", e);
            }
        }
    };
    
    // 开始编辑
    let start_edit = move |insp: Inspiration| {
        editing_inspiration.set(Some(insp.clone()));
        form_data.set(InspirationFormData {
            title: insp.title,
            content: insp.content,
            tags: insp.tags.join(", "),
        });
        view_mode.set(InspirationViewMode::Form);
    };
    
    // 取消编辑
    let cancel_edit = move |_| {
        editing_inspiration.set(None);
        form_data.set(InspirationFormData {
            title: String::new(),
            content: String::new(),
            tags: String::new(),
        });
        view_mode.set(InspirationViewMode::List);
    };
    
    // 处理搜索
    let handle_search = move |e: Event<FormData>| {
        search_query.set(e.value().clone());
        
        if let Some(novel_id) = current_novel_id() {
            load_inspirations(novel_id);
        }
    };
    
    // 处理标签选择
    let handle_tag_select = move |tag: String| {
        if selected_tag() == Some(tag.clone()) {
            selected_tag.set(None);
        } else {
            selected_tag.set(Some(tag.clone()));
        }
        
        if let Some(novel_id) = current_novel_id() {
            load_inspirations(novel_id);
        }
    };
    
    // 渲染组件
    rsx! {
        div {
            class: "inspiration-management",
            if current_novel_id().is_some() {
                if view_mode() == InspirationViewMode::List {
                    div {
                        class: "inspiration-list-container",
                        // 搜索和过滤栏
                        div {
                            class: "search-filter-bar",
                            input {
                                class: "search-input",
                                placeholder: "搜索灵感...",
                                value: search_query(),
                                oninput: handle_search
                            }
                        }
                        
                        // 标签筛选
                        div {
                            class: "tags-filter",
                            for (tag, count) in all_tags() {
                                TagButton {
                                    tag: tag.clone(),
                                    count: count,
                                    selected_tag: selected_tag(),
                                    on_click: handle_tag_select,
                                }
                            }
                        }
                        
                        // 操作按钮
                        div {
                            class: "action-buttons",
                            button {
                                class: "btn-primary",
                                onclick: move |_| {
                                    editing_inspiration.set(None);
                                    form_data.set(InspirationFormData {
                                        title: String::new(),
                                        content: String::new(),
                                        tags: String::new(),
                                    });
                                    view_mode.set(InspirationViewMode::Form);
                                },
                                "新建灵感"
                            }
                        }
                        
                        // 灵感列表
                        div {
                            class: "inspiration-list-wrapper",
                            div {
                                class: "inspiration-list",
                                if filtered_inspirations().is_empty() {
                                    div {
                                        class: "empty-state",
                                        p { "暂无灵感" }
                                    }
                                } else {
                                    for insp in filtered_inspirations() {
                                        InspirationItem {
                                            inspiration: Rc::new(insp.clone()),
                                            on_toggle_pin: handle_toggle_pin,
                                            on_edit: start_edit,
                                            on_delete: handle_delete_inspiration,
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div {
                        class: "inspiration-form",
                        h3 { if editing_inspiration().is_some() { "编辑灵感" } else { "新建灵感" } }
                        
                        div {
                            class: "form-group",
                            label { "标题" }
                            input {
                                class: "form-input",
                                value: form_data.with(|data| data.title.clone()),
                                oninput: move |e| {
                                    let new_title = e.value().clone();
                                    form_data.with_mut(|data| data.title = new_title);
                                },
                                placeholder: "请输入灵感标题",
                            }
                        }
                        
                        div {
                            class: "form-group",
                            label { "内容" }
                            textarea {
                                class: "form-textarea",
                                value: form_data.with(|data| data.content.clone()),
                                oninput: move |e| {
                                    let new_content = e.value().clone();
                                    form_data.with_mut(|data| data.content = new_content);
                                },
                                placeholder: "请输入灵感内容",
                                rows: "10",
                            }
                        }
                        
                        div {
                            class: "form-group",
                            label { "标签 (用逗号分隔)" }
                            input {
                                class: "form-input",
                                value: form_data.with(|data| data.tags.clone()),
                                oninput: move |e| {
                                    let new_tags = e.value().clone();
                                    form_data.with_mut(|data| data.tags = new_tags);
                                },
                                placeholder: "例如: 角色, 情节, 场景",
                            }
                        }
                        
                        div {
                            class: "form-actions",
                            button {
                                class: "btn-primary",
                                onclick: move |_| {
                                    if editing_inspiration().is_some() {
                                        handle_update_inspiration(());
                                    } else {
                                        handle_create_inspiration(());
                                    }
                                },
                                {if editing_inspiration().is_some() { "更新" } else { "保存" }}
                            }
                            button {
                                class: "btn-secondary",
                                onclick: cancel_edit,
                                "取消"
                            }
                        }
                    }
                }
            } else {
                div {
                    class: "empty-state",
                    p { "请先从左侧选择一部小说" }
                }
            }
        }
    }
}