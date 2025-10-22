# Novel Writer 架构设计文档

本文档详细描述 Novel Writer 应用的系统架构、组件设计和数据流，帮助开发者理解系统的整体结构和工作原理。

## 整体架构

Novel Writer 采用分层架构设计，各层职责明确，边界清晰。整体架构如下：

```
+----------------------------+
|           UI 层            |
|  (Dioxus 组件和布局)        |
+----------------------------+
            |
+----------------------------+
|          核心层            |
|  (业务逻辑和管理器)         |
+----------------------------+
            |
+----------------------------+
|          数据层            |
|  (数据库和数据模型)         |
+----------------------------+
```

## 分层详解

### 1. UI 层

UI 层基于 Dioxus 框架构建，提供用户交互界面。主要组件包括：

- **App 组件**：应用的根组件，管理全局状态和视图切换
- **组件库**：包含各种功能组件，如编辑器、章节列表、灵感管理等
- **布局组件**：定义应用的整体布局结构

UI 层通过信号（Signal）机制管理状态，并与核心层交互获取和更新数据。

### 2. 核心层

核心层包含应用的业务逻辑，通过各种管理器类实现功能。主要模块包括：

- **NovelManager**：小说管理逻辑
- **ChapterManager**：章节管理逻辑
- **InspirationManager**：灵感管理逻辑
- **VersionManager**：版本控制逻辑
- **StatsManager**：统计功能逻辑
- **AppState**：应用状态管理

核心层封装了业务规则和数据处理逻辑，为 UI 层提供清晰的接口。

### 3. 数据层

数据层负责数据的持久化和访问，主要包括：

- **Database**：数据库操作类，提供对 SQLite 的访问
- **Models**：数据模型定义
- **Migrations**：数据库迁移和架构管理

数据层采用 SQLite 作为存储引擎，通过 Rusqlite 库提供的接口进行操作。

## 数据流

### 数据获取流程

1. UI 组件通过 Signal 订阅数据变化
2. 当需要获取数据时，UI 组件调用核心层的管理器方法
3. 管理器调用数据层的方法查询数据库
4. 数据层返回结果给管理器
5. 管理器处理数据并返回给 UI 组件
6. UI 组件更新 Signal，触发视图更新

### 数据更新流程

1. UI 组件收集用户输入
2. 用户触发操作（如保存、提交）
3. UI 组件调用核心层的管理器方法，传递更新数据
4. 管理器验证数据并调用数据层方法更新数据库
5. 数据层执行 SQL 操作并返回结果
6. 管理器更新业务状态并通知 UI 层
7. UI 层更新视图以反映更改

## 核心模块设计

### 1. 小说管理模块

#### 职责
- 管理小说的创建、读取、更新和删除
- 维护小说的元数据和状态
- 提供小说列表和详情访问

#### 设计

**NovelManager** 类封装了所有小说相关的业务逻辑：

```rust
pub struct NovelManager;

impl NovelManager {
    // 创建新小说
    pub async fn create_novel(&self, title: &str, author: Option<&str>) -> Result<Novel>
    
    // 获取所有小说
    pub async fn get_all_novels(&self) -> Result<Vec<Novel>>
    
    // 其他小说操作方法...
}
```

**Novel** 数据模型定义了小说的属性：

```rust
pub struct Novel {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub word_count: i32,
    pub status: NovelStatus,
    pub is_pinned: bool,
    pub pinned_order: Option<i32>,
}
```

#### 交互关系

NovelManager ←→ Database ←→ UI Components

### 2. 章节管理模块

#### 职责
- 管理章节的创建、编辑、移动和删除
- 维护章节的层级结构
- 提供章节树的构建和访问

#### 设计

**ChapterManager** 类负责章节的业务逻辑：

```rust
pub struct ChapterManager;

impl ChapterManager {
    // 构建章节树
    pub fn build_chapter_tree(&self, chapters: Vec<Chapter>) -> ChapterTree
    
    // 创建新章节
    pub async fn create_chapter(
        &self, 
        novel_id: i64, 
        title: &str, 
        parent_id: Option<i64>
    ) -> Result<Chapter>
    
    // 其他章节操作方法...
}
```

**ChapterTree** 和 **ChapterNode** 数据结构用于表示章节的层级关系：

```rust
pub struct ChapterTree {
    pub nodes: HashMap<i64, ChapterNode>,
    pub root_nodes: Vec<i64>,
}

pub struct ChapterNode {
    pub chapter: Chapter,
    pub children: Vec<i64>,
    pub depth: usize,
}
```

#### 交互关系

ChapterManager ←→ Database ←→ UI Components

### 3. 灵感管理模块

#### 职责
- 管理灵感记录的创建、编辑和删除
- 提供灵感的标签和关联功能
- 生成灵感统计和趋势分析

#### 设计

**InspirationManager** 类封装了灵感管理的业务逻辑：

```rust
pub struct InspirationManager;

impl InspirationManager {
    // 创建新灵感
    pub async fn create_inspiration(
        &self,
        novel_id: i64,
        title: &str,
        content: &str,
        tags: Vec<String>
    ) -> Result<Inspiration>
    
    // 获取灵感统计
    pub async fn get_inspiration_stats(&self, novel_id: i64) -> Result<InspirationStats>
    
    // 其他灵感操作方法...
}
```

**Inspiration** 数据模型定义了灵感记录的属性：

```rust
pub struct Inspiration {
    pub id: i64,
    pub novel_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_pinned: bool,
    pub tags: Vec<String>,
    pub linked_chapters: Vec<i64>,
}
```

#### 交互关系

InspirationManager ←→ Database ←→ UI Components

### 4. 数据库设计

#### 表结构

**novels 表**
- `id`：主键
- `title`：小说标题
- `author`：作者
- `description`：描述
- `created_at`：创建时间
- `updated_at`：更新时间
- `word_count`：字数统计
- `status`：状态（draft, writing, completed, abandoned）
- `is_pinned`：是否置顶
- `pinned_order`：置顶顺序

**chapters 表**
- `id`：主键
- `novel_id`：所属小说ID
- `parent_id`：父章节ID
- `title`：章节标题
- `content`：章节内容
- `sort_path`：排序路径（用于树状结构）
- `word_count`：字数统计
- `created_at`：创建时间
- `updated_at`：更新时间
- `chapter_type`：章节类型（volume, chapter, scene）
- `is_archived`：是否归档

**chapter_versions 表**
- `id`：主键
- `chapter_id`：所属章节ID
- `parent_version_id`：父版本ID
- `version_type`：版本类型（snapshot, diff）
- `content`：版本内容
- `created_at`：创建时间

## 状态管理

应用使用 Dioxus 的信号（Signal）系统进行状态管理。主要状态包括：

- **current_view**：当前活动视图
- **current_novel_id**：当前选中的小说ID
- **novels**：小说列表
- **current_chapter**：当前编辑的章节
- **inspiration_stats**：灵感统计信息
- **writing_report**：写作报告

这些状态在组件间共享，当状态变化时，相关组件会自动重新渲染。

## 配置管理

应用通过 `AppConfig` 结构体管理配置：

```rust
pub struct AppConfig {
    pub app_name: String,
    pub version: String,
    pub database_path: String,
    pub auto_save_interval: u64,
    pub theme: ThemeConfig,
    pub editor: EditorConfig,
}
```

配置文件默认存储在用户目录下，应用启动时加载配置，运行时使用这些配置控制行为。

## 扩展性设计

### 插件系统

应用设计支持通过插件系统扩展功能。插件可以：
- 添加新的编辑器功能
- 扩展导出格式
- 提供额外的统计分析

### API 设计

核心层提供了清晰的 API 接口，使 UI 层可以方便地调用功能。这些 API 设计遵循以下原则：
- 单一职责：每个方法只负责一个功能
- 异步优先：耗时操作使用异步模式
- 错误处理：使用 Result 类型统一处理错误

## 性能优化

### 延迟加载

UI 组件采用延迟加载策略，只在需要时加载数据，减少初始加载时间。

### 缓存

常用数据（如小说列表、章节树）会缓存在内存中，避免频繁的数据库查询。

### 数据库优化

- 使用 WAL（Write-Ahead Logging）模式提高写入性能
- 合理设计索引加速查询
- 使用事务确保数据一致性

## 安全性考虑

- 数据库文件权限控制，确保只有应用可以访问
- 输入验证，防止恶意数据
- 错误处理中避免泄露敏感信息

## 总结

Novel Writer 采用了清晰的分层架构和模块化设计，使系统易于理解、维护和扩展。核心层封装业务逻辑，数据层处理持久化，UI 层负责用户交互，各层之间通过定义良好的接口通信。这种设计使系统具有良好的可测试性和可扩展性，能够满足不同用户的需求。