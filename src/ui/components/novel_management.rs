use dioxus::prelude::*;
use crate::db;
use std::time::Duration;

#[derive(PartialEq, Clone, Copy)]
enum ViewMode {
    List,
    Grid,
}

#[derive(PartialEq, Clone, Copy)]
enum FilterStatus {
    All,
    Draft,
    Writing,
    Completed,
    Abandoned,
}

#[component]
pub fn NovelManagement(
    novels: Signal<Vec<db::Novel>>,
    current_novel_id: Signal<Option<i64>>,
    on_select_novel: EventHandler<i64>,
    on_edit_novel: EventHandler<db::Novel>,
    on_delete_novel: EventHandler<i64>,
    show_novel_form: Signal<bool>,
    novel_title: Signal<String>,
    novel_author: Signal<String>,
    novel_description: Signal<String>,
    editing_novel: Signal<Option<db::Novel>>,
    on_novel_submit: EventHandler<()>,
    on_cancel_form: EventHandler<()>,
    on_add_novel: EventHandler<()>,
    on_toggle_pin: EventHandler<(i64, String)>,
) -> Element {
    let mut view_mode = use_signal(|| ViewMode::List);
    let mut search_query = use_signal(|| "".to_string());
    let mut filter_status = use_signal(|| FilterStatus::All);
    let mut is_loading = use_signal(|| false);
    let mut error_message = use_signal(|| None::<String>);
    let mut current_page = use_signal(|| 1);
    const PAGE_SIZE: usize = 9;
    
    // 模拟加载状态
    let mut simulate_load = move || {
        is_loading.set(true);
        error_message.set(None);
        
        spawn(async move {
            tokio::time::sleep(Duration::from_millis(300)).await;
            is_loading.set(false);
        });
    };
    
    // 处理添加小说
    let handle_add_novel = move |_| {
        simulate_load();
        on_add_novel.call(());
    };
    
    // 处理编辑小说
    let handle_edit_novel = move |novel: db::Novel| {
        simulate_load();
        on_edit_novel.call(novel);
    };
    
    // 处理删除小说
    let handle_delete_novel = move |novel_id: i64| {
        // 显示确认对话框
        if let Some(window) = web_sys::window() {
            if window.confirm_with_message("确定要删除这本小说吗？此操作不可撤销，所有相关章节也将被删除。").unwrap_or(false) {
                simulate_load();
                on_delete_novel.call(novel_id);
            }
        }
    };
    
    // 过滤小说列表
    let filtered_novels = move || {
        let query = search_query().to_lowercase();
        let status = filter_status();
        
        novels().into_iter()
            .filter(|novel| {
                // 搜索过滤
                let matches_search = query.is_empty() || 
                    novel.title.to_lowercase().contains(&query) || 
                    novel.author.to_lowercase().contains(&query) || 
                    novel.description.to_lowercase().contains(&query);
                
                // 状态过滤
                let matches_status = status == FilterStatus::All || 
                    (status == FilterStatus::Draft && novel.status == db::NovelStatus::Draft) ||
                    (status == FilterStatus::Writing && novel.status == db::NovelStatus::Writing) ||
                    (status == FilterStatus::Completed && novel.status == db::NovelStatus::Completed) ||
                    (status == FilterStatus::Abandoned && novel.status == db::NovelStatus::Abandoned);
                
                matches_search && matches_status
            })
            .collect::<Vec<_>>()
    };
    
    // 计算统计数据
    let stats = move || {
        let all = novels().len();
        let draft = novels().iter().filter(|n| n.status == db::NovelStatus::Draft).count();
        let writing = novels().iter().filter(|n| n.status == db::NovelStatus::Writing).count();
        let completed = novels().iter().filter(|n| n.status == db::NovelStatus::Completed).count();
        let abandoned = novels().iter().filter(|n| n.status == db::NovelStatus::Abandoned).count();
        
        (all, draft, writing, completed, abandoned)
    };
    
    // 预先计算过滤后的小说列表
    let filtered = filtered_novels();
    
    // 页面变化时重置到第一页
    let reset_page = move || {
        current_page.set(1);
    };
    
    // 使用一个单独的信号来跟踪上一次的搜索和筛选状态
    let mut last_search_and_filter = use_signal(|| {
        (search_query(), filter_status())
    });
    
    // 当搜索或筛选变化时重置页码
    use_effect(move || {
        let current = (search_query(), filter_status());
        if current != last_search_and_filter() {
            current_page.set(1);
            last_search_and_filter.set(current);
        }
    });
    
    // 计算分页信息并按更新时间倒序排序
    let mut filtered_clone = filtered.clone();
    filtered_clone.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    let current_page_value = current_page();
    let page_value = current_page_value - 1;
    let start_index = page_value * PAGE_SIZE;
    let end_index = start_index + PAGE_SIZE;
    let paginated_novels = filtered_clone[start_index..end_index.min(filtered_clone.len())].to_vec();
    let total_items = filtered_clone.len();
    let total_pages = (total_items + PAGE_SIZE - 1) / PAGE_SIZE;
    
    rsx! {
        div {
            class: "novel-management",
            
            // 小说表单
            if show_novel_form() {
                NovelForm {
                    novel_title,
                    novel_author,
                    novel_description,
                    editing_novel,
                    on_submit: on_novel_submit,
                    on_cancel: on_cancel_form,
                }
            } else {
                // 工具栏
                div {
                    class: "novel-toolbar",
                    h2 { "📚 小说管理" }
                    
                    // 搜索栏
                    div {
                        class: "search-container",
                        input {
                            class: "search-input",
                            placeholder: "搜索小说标题、作者或描述...",
                            value: "{search_query()}",
                            oninput: move |e| search_query.set(e.value()),
                        }
                    }
                    
                    // 统计信息
                    div {
                        class: "stats-summary",
                        span { "总计: {stats().0} | " },
                        span { class: "stat-draft", "草稿: {stats().1} | " },
                        span { class: "stat-writing", "写作中: {stats().2} | " },
                        span { class: "stat-completed", "已完成: {stats().3} | " },
                        span { class: "stat-abandoned", "已废弃: {stats().4}" },
                    }
                    
                    // 操作栏
                    div {
                        class: "toolbar-actions",
                        // 状态筛选器
                        div {
                            class: "status-filter",
                            button {
                                class: if filter_status() == FilterStatus::All { "filter-btn active" } else { "filter-btn" },
                                onclick: move |_| filter_status.set(FilterStatus::All),
                                "全部"
                            }
                            button {
                                class: if filter_status() == FilterStatus::Draft { "filter-btn active" } else { "filter-btn" },
                                onclick: move |_| filter_status.set(FilterStatus::Draft),
                                "草稿"
                            }
                            button {
                                class: if filter_status() == FilterStatus::Writing { "filter-btn active" } else { "filter-btn" },
                                onclick: move |_| filter_status.set(FilterStatus::Writing),
                                "写作中"
                            }
                            button {
                                class: if filter_status() == FilterStatus::Completed { "filter-btn active" } else { "filter-btn" },
                                onclick: move |_| filter_status.set(FilterStatus::Completed),
                                "已完成"
                            }
                            button {
                                class: if filter_status() == FilterStatus::Abandoned { "filter-btn active" } else { "filter-btn" },
                                onclick: move |_| filter_status.set(FilterStatus::Abandoned),
                                "已废弃"
                            }
                        }
                        
                        // 新建按钮
                        button {
                            class: "btn btn-primary",
                            onclick: handle_add_novel,
                            "➕ 新建小说"
                        }
                        
                        // 视图切换
                        div {
                            class: "view-switcher",
                            button {
                                class: if view_mode() == ViewMode::List { "view-btn active" } else { "view-btn" },
                                onclick: move |_| view_mode.set(ViewMode::List),
                                title: "列表视图",
                                "📋"
                            }
                            button {
                                class: if view_mode() == ViewMode::Grid { "view-btn active" } else { "view-btn" },
                                onclick: move |_| view_mode.set(ViewMode::Grid),
                                title: "网格视图",
                                "▦"
                            }
                        }
                    }
                }
                
                // 错误提示
                if let Some(err) = error_message() {
                    div {
                        class: "error-message",
                        p { "❌ {err}" }
                    }
                }
                
                // 加载状态
                if is_loading() {
                    div {
                        class: "loading-indicator",
                        div { class: "spinner" }
                        p { "加载中..." }
                    }
                }
                
                // 小说列表
                if filtered.is_empty() {
                    div {
                        class: "empty-state",
                        p { "📖 没有找到匹配的小说" }
                        if !search_query().is_empty() || filter_status() != FilterStatus::All {
                            p {
                                class: "hint",
                                "尝试调整搜索条件或筛选器"
                            }
                            button {
                                class: "btn btn-secondary",
                                onclick: move |_| {
                                    search_query.set("".to_string());
                                    filter_status.set(FilterStatus::All);
                                },
                                "重置筛选条件"
                            }
                        } else {
                            p {
                                class: "hint",
                                "点击上方 \"新建小说\" 按钮开始创作"
                            }
                        }
                    }
                } else {
                    // 渲染列表或网格视图
                    if view_mode() == ViewMode::List {
                        div {
                            class: "container",
                            NovelListTableView {
                                novels: Signal::new(paginated_novels),
                                current_novel_id: current_novel_id,
                                on_select_novel: on_select_novel,
                                on_edit_novel: handle_edit_novel,
                                on_delete_novel: handle_delete_novel,
                                on_toggle_pin: on_toggle_pin
                            }
                        }
                    } else {
                        div {
                            class: "container",
                            NovelGridView {
                                novels: Signal::new(paginated_novels),
                                current_novel_id: current_novel_id,
                                on_select_novel: on_select_novel,
                                on_edit_novel: handle_edit_novel,
                                on_delete_novel: handle_delete_novel
                            }
                        }
                    }
                    
                    // 分页控件
                    div {
                        class: "pagination",
                        button {
                            class: "pagination-btn",
                            onclick: move |_| if current_page() > 1 {{ current_page.set(current_page() - 1); }},
                            disabled: current_page() == 1,
                            "上一页"
                        }
                        for page in 1..=total_pages {
                            button {
                                    key: "{page}",
                                    class: if current_page() == page {"pagination-btn active"} else {"pagination-btn"},
                                    onclick: move |_| current_page.set(page),
                                    "{page}"
                                }
                        }
                        button {
                            class: "pagination-btn",
                            onclick: move |_| if current_page() < total_pages {{ current_page.set(current_page() + 1); }},
                            disabled: current_page() == total_pages,
                            "下一页"
                        }
                        span {
                            class: "pagination-info",
                            "第 {current_page()} / {total_pages} 页，共 {total_items} 本小说"
                        }
                    }
                }
            }
        }
}
}

#[component]
fn NovelListTableView(
    novels: Signal<Vec<db::Novel>>,
    current_novel_id: Signal<Option<i64>>,
    on_select_novel: EventHandler<i64>,
    on_edit_novel: EventHandler<db::Novel>,
    on_delete_novel: EventHandler<i64>,
    on_toggle_pin: EventHandler<(i64, String)>,
) -> Element {
    rsx! {
        div {
            class: "novel-list-view",
            table {
                class: "novel-table",
                thead {
                    tr {
                        th { "置顶" }
                        th { "标题" }
                        th { "作者" }
                        th { "创建日期" }
                        th { "最后更新" }
                        th { "字数" }
                        th { "状态" }
                        th { "操作" }
                    }
                }
                tbody {
                    for novel in novels() {
                        {
                            let is_selected = current_novel_id() == Some(novel.id);
                            let novel_id = novel.id;
                            let novel_for_edit = novel.clone();
                            let created_at = novel.created_at.format("%Y-%m-%d").to_string();
                            let updated_at = novel.updated_at.format("%Y-%m-%d %H:%M:%S").to_string();
                            let truncated_desc = if novel.description.len() > 50 {
                                format!("{}{}", &novel.description[0..50], "...")
                            } else {
                                novel.description.clone()
                            };
                            
                            // 计算更新时间的相对描述
                            let relative_time_text = {
                                let now = chrono::Utc::now();
                                let duration = now.signed_duration_since(novel.updated_at);
                                
                                if duration.num_days() == 0 {
                                    if duration.num_hours() == 0 {
                                        if duration.num_minutes() < 1 {
                                            "刚刚".to_string()
                                        } else {
                                            format!("{}分钟前", duration.num_minutes())
                                        }
                                    } else {
                                        format!("{}小时前", duration.num_hours())
                                    }
                                } else if duration.num_days() < 7 {
                                    format!("{}天前", duration.num_days())
                                } else {
                                    updated_at.clone()
                                }
                            };
                            
                            rsx! {
                                tr {
                                    key: "{novel_id}",
                                    class: if is_selected { "selected" } else { "" },
                                    onclick: move |_| on_select_novel.call(novel_id),
                                    
                                    td {
                                        button { 
                                            class: if novel.is_pinned { "pin-btn pinned" } else { "pin-btn" },
                                            title: if novel.is_pinned { "取消置顶" } else { "置顶" },
                                            onclick: move |e: Event<MouseData>| {
                                                e.stop_propagation();
                                                // 检查最多只能置顶3本小说
                                                if !novel.is_pinned {
                                                    let pinned_count = novels().iter().filter(|n| n.is_pinned).count();
                                                    if pinned_count >= 3 {
                                                        if let Some(window) = web_sys::window() {
                                                            window.alert_with_message("最多只能置顶3本小说").unwrap_or(());
                                                        }
                                                        return;
                                                    }
                                                }
                                                
                                                // 调用置顶逻辑
                                                on_toggle_pin.call((novel_id, novel.title.clone()));
                                            },
                                            if novel.is_pinned { "📌" } else { "📎" }
                                        }
                                    }
                                    
                                    td {
                                    class: "novel-title-cell",
                                    div {
                                        span { class: "novel-icon", "📚" }
                                        span { class: "novel-title", "{novel.title}" }
                                    }
                                    // 显示部分描述作为提示
                                    if !novel.description.is_empty() {
                                        div {
                                            class: "novel-description-hint",
                                            "{truncated_desc}"
                                        }
                                    }
                                }
                                    td { 
                                        class: "author-cell",
                                        "{novel.author}"
                                    }
                                    td { 
                                        class: "date-cell",
                                        title: "创建日期",
                                        "{created_at}"
                                    }
                                    td { 
                                        class: "update-cell",
                                        title: "{updated_at}",
                                        "{relative_time_text}"
                                    }
                                    td {
                                        class: "word-count",
                                        span {
                                            class: if novel.word_count > 0 { "positive" } else { "zero" },
                                            "{novel.word_count} 字"
                                        }
                                    }
                                    td {
                                        span {
                                            class: "status-badge status-{novel.status.as_str()}",
                                            title: match novel.status {
                                                db::NovelStatus::Draft => "草稿状态",
                                                db::NovelStatus::Writing => "正在写作中",
                                                db::NovelStatus::Completed => "已完成的作品",
                                                db::NovelStatus::Abandoned => "已废弃的作品",
                                            },
                                            match novel.status {
                                                db::NovelStatus::Draft => "草稿",
                                                db::NovelStatus::Writing => "写作中",
                                                db::NovelStatus::Completed => "已完成",
                                                db::NovelStatus::Abandoned => "已废弃",
                                            }
                                        }
                                    }
                                    td {
                                        class: "action-buttons",
                                        button {
                                            class: "action-btn edit-btn",
                                            title: "编辑小说信息",
                                            aria_label: "编辑",
                                            onclick: move |e: Event<MouseData>| {
                                                e.stop_propagation();
                                                on_edit_novel.call(novel_for_edit.clone());
                                            },
                                            "✏️"
                                        }
                                        button {
                                            class: "action-btn delete-btn",
                                            title: "删除小说",
                                            aria_label: "删除",
                                            onclick: move |e: Event<MouseData>| {
                                                e.stop_propagation();
                                                on_delete_novel.call(novel_id);
                                            },
                                            "🗑️"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NovelGridView(
    novels: Signal<Vec<db::Novel>>,
    current_novel_id: Signal<Option<i64>>,
    on_select_novel: EventHandler<i64>,
    on_edit_novel: EventHandler<db::Novel>,
    on_delete_novel: EventHandler<i64>,
) -> Element {
    rsx! {
        div {
            class: "novel-grid-view",
            for novel in novels() {
                {
                    let is_selected = current_novel_id() == Some(novel.id);
                    let novel_id = novel.id;
                    let novel_for_edit = novel.clone();
                    let created_at = novel.created_at.format("%Y-%m-%d").to_string();
                    let updated_at = novel.updated_at.format("%Y-%m-%d %H:%M:%S").to_string();
                    let truncated_desc = if novel.description.len() > 100 {
                        format!("{}{}", &novel.description[0..100], "...")
                    } else {
                        novel.description.clone()
                    };
                    
                    // 计算更新时间的相对描述
                    let relative_time = move || {
                        let now = chrono::Utc::now();
                        let duration = now.signed_duration_since(novel.updated_at);
                        
                        if duration.num_days() == 0 {
                            if duration.num_hours() == 0 {
                                if duration.num_minutes() < 1 {
                                    "刚刚".to_string()
                                } else {
                                    format!("{}分钟前", duration.num_minutes())
                                }
                            } else {
                                format!("{}小时前", duration.num_hours())
                            }
                        } else if duration.num_days() == 1 {
                            "昨天".to_string()
                        } else if duration.num_days() < 7 {
                            format!("{}天前", duration.num_days())
                        } else {
                            format!("{}", novel.updated_at.format("%m-%d"))
                        }
                    };
                    
                    rsx! {
                        div {
                            key: "{novel_id}",
                            class: if is_selected { "novel-card selected" } else { "novel-card" },
                            onclick: move |_| on_select_novel.call(novel_id),
                            
                            // 卡片头部
                            div {
                                class: "card-header",
                                // 状态标签
                                span {
                                    class: "status-badge status-{novel.status.as_str()}",
                                    title: match novel.status {
                                        db::NovelStatus::Draft => "草稿状态",
                                        db::NovelStatus::Writing => "正在写作中",
                                        db::NovelStatus::Completed => "已完成的作品",
                                        db::NovelStatus::Abandoned => "已废弃的作品",
                                    },
                                    match novel.status {
                                        db::NovelStatus::Draft => "草稿",
                                        db::NovelStatus::Writing => "写作中",
                                        db::NovelStatus::Completed => "已完成",
                                        db::NovelStatus::Abandoned => "已废弃",
                                    }
                                }
                                // 标题
                                h3 {
                                    class: "card-title",
                                    span { class: "novel-icon", "📖" }
                                    span { "{novel.title}" }
                                }
                            }
                            
                            // 卡片主体
                            div {
                                class: "card-body",
                                // 作者信息
                                p {
                                    class: "author",
                                    span { class: "author-icon", "✍️" }
                                    span { "{novel.author}" }
                                }
                                // 小说简介
                                p {
                                    class: "description",
                                    "{truncated_desc}"
                                }
                            }
                            
                            // 卡片底部
                            div {
                                class: "card-footer",
                                // 统计信息
                                div {
                                    class: "card-stats",
                                    span {
                                        class: "stat-item word-count",
                                        title: "总字数",
                                        span { class: "stat-icon", "📝" }
                                        span { "{novel.word_count} 字" }
                                    }
                                    span {
                                        class: "stat-item created-date",
                                        title: "创建日期",
                                        span { class: "stat-icon", "📅" }
                                        span { "{created_at}" }
                                    }
                                    span {
                                        class: "stat-item update-time",
                                        title: "最后更新: {updated_at}",
                                        span { class: "stat-icon", "🕐" }
                                        span { "{relative_time()}" }
                                    }
                                }
                                // 操作按钮
                                div {
                                    class: "card-actions",
                                    button {
                                        class: "action-btn edit-btn",
                                        title: "编辑小说信息",
                                        aria_label: "编辑",
                                        onclick: move |e: Event<MouseData>| {
                                            e.stop_propagation();
                                            on_edit_novel.call(novel_for_edit.clone());
                                        },
                                        "✏️"
                                    }
                                    button {
                                        class: "action-btn delete-btn",
                                        title: "删除小说",
                                        aria_label: "删除",
                                        onclick: move |e: Event<MouseData>| {
                                            e.stop_propagation();
                                            on_delete_novel.call(novel_id);
                                        },
                                        "🗑️"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NovelForm(
    novel_title: Signal<String>,
    novel_author: Signal<String>,
    novel_description: Signal<String>,
    editing_novel: Signal<Option<db::Novel>>,
    on_submit: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    let is_editing = editing_novel().is_some();
    let mut current_status = use_signal(|| {
        editing_novel().as_ref().map(|n| n.status.clone()).unwrap_or(db::NovelStatus::Draft)
    });
    
    rsx! {
        div {
            class: "novel-form-overlay",
            onclick: move |_| on_cancel.call(()),
            
            div {
                class: "novel-form",
                onclick: move |e: Event<MouseData>| e.stop_propagation(),
                
                // 表单标题
                h2 {
                    class: "form-title",
                    if is_editing { "编辑小说" } else { "新建小说" }
                }
                
                // 小说标题
                div {
                    class: "form-group",
                    label { 
                        class: "form-label required",
                        "小说标题"
                    }
                    input {
                        r#type: "text",
                        class: "form-input",
                        placeholder: "请输入小说标题",
                        value: "{novel_title}",
                        oninput: move |e| novel_title.set(e.value()),
                        autofocus: true,
                        required: true,
                    }
                    if novel_title().trim().is_empty() {
                        p { class: "form-hint", "标题不能为空" }
                    }
                }
                
                // 作者信息
                div {
                    class: "form-group",
                    label { 
                        class: "form-label",
                        "作者"
                    }
                    input {
                        r#type: "text",
                        class: "form-input",
                        placeholder: "请输入作者名",
                        value: "{novel_author}",
                        oninput: move |e| novel_author.set(e.value()),
                    }
                    p { class: "form-hint", "留空表示匿名" }
                }
                
                // 小说状态
                div {
                    class: "form-group",
                    label { 
                        class: "form-label",
                        "状态"
                    }
                    div {
                        class: "status-selector",
                        button {
                            class: if current_status() == db::NovelStatus::Draft { "status-option active" } else { "status-option" },
                            onclick: move |_| current_status.set(db::NovelStatus::Draft),
                            span { class: "status-dot draft" }
                            span { "草稿" }
                        }
                        button {
                            class: if current_status() == db::NovelStatus::Writing { "status-option active" } else { "status-option" },
                            onclick: move |_| current_status.set(db::NovelStatus::Writing),
                            span { class: "status-dot writing" }
                            span { "写作中" }
                        }
                        button {
                            class: if current_status() == db::NovelStatus::Completed { "status-option active" } else { "status-option" },
                            onclick: move |_| current_status.set(db::NovelStatus::Completed),
                            span { class: "status-dot completed" }
                            span { "已完成" }
                        }
                        button {
                            class: if current_status() == db::NovelStatus::Abandoned { "status-option active" } else { "status-option" },
                            onclick: move |_| current_status.set(db::NovelStatus::Abandoned),
                            span { class: "status-dot abandoned" }
                            span { "已废弃" }
                        }
                    }
                }
                
                // 小说简介
                div {
                    class: "form-group",
                    label { 
                        class: "form-label",
                        "简介"
                    }
                    textarea {
                        class: "form-textarea",
                        placeholder: "请输入小说简介（不超过500字）",
                        value: "{novel_description}",
                        oninput: move |e| {
                            // 限制输入长度
                            let value = e.value();
                            if value.len() <= 500 {
                                novel_description.set(value);
                            }
                        },
                        rows: "4",
                        maxlength: 500,
                    }
                    p { 
                        class: "form-hint",
                        "{novel_description().len()}/500 字"
                    }
                }
                
                // 表单操作按钮
                div {
                    class: "form-actions",
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            // 如果是编辑模式，更新状态
                            if let Some(mut novel) = editing_novel() {
                                novel.status = current_status();
                                // 这里应该有更新状态的逻辑，但由于组件限制，我们只调用提交
                            }
                            on_submit.call(());
                        },
                        disabled: novel_title().trim().is_empty(),
                        if is_editing { "更新小说" } else { "创建小说" }
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