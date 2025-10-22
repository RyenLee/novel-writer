# Novel Writer - 专业小说写作工具

一个使用 Rust 和 Dioxus 构建的专业小说写作工具，提供小说管理、章节编辑、灵感记录和写作统计等功能。

## 仓库概览

Novel Writer 是一个专为小说家设计的写作工具，提供以下核心功能：
- 多小说项目管理，支持草稿、写作中、已完成、已废弃等状态
- 章节树状结构管理，支持卷、章、场景的层级组织
- 灵感记录与管理，支持标签和章节关联
- 实时字数统计和写作进度跟踪
- 章节版本控制，支持历史版本回溯
- 自定义编辑器设置，包括字体、主题、自动保存等

## 目录结构

```
src/
├── assets/            # 静态资源
├── config.rs          # 配置管理
├── core/              # 核心业务逻辑
│   ├── app_state.rs   # 应用状态管理
│   ├── chapter_manager.rs # 章节管理
│   ├── formatter.rs   # 内容格式化
│   ├── inspiration_manager.rs # 灵感管理
│   ├── mod.rs
│   ├── novel_manager.rs # 小说管理
│   ├── stats_manager.rs # 统计功能
│   └── version_manager.rs # 版本控制
├── db/                # 数据库层
│   ├── migrations.rs  # 数据库迁移
│   ├── mod.rs         # 数据库操作
│   ├── models.rs      # 数据模型
│   └── schema.sql     # 数据库架构
├── examples/          # 示例代码
├── init.rs            # 应用初始化
├── lib.rs             # 库入口
├── main.rs            # 主程序入口
├── ui/                # 用户界面
│   ├── app.rs         # 应用主组件
│   ├── components/    # UI组件
│   ├── layouts/       # 布局组件
│   └── mod.rs
└── utils/             # 工具函数
    ├── diff_utils.rs  # 差异比较工具
    └── mod.rs
```

## 系统架构与主流程

Novel Writer 采用分层架构设计，各层职责明确：

1. **数据层**：基于 SQLite 的数据库存储，通过 `db` 模块提供数据持久化
2. **核心层**：包含各种管理器类，处理业务逻辑，如小说管理、章节管理、灵感管理等
3. **UI层**：使用 Dioxus 构建的现代化界面，提供用户交互
4. **配置层**：管理应用设置和用户偏好

主要流程：
1. 应用启动时，通过 `init` 模块初始化数据库和应用环境
2. UI 加载并显示小说列表
3. 用户可以创建、编辑、删除小说
4. 进入小说详情后，可以管理章节、记录灵感、查看统计信息
5. 编辑器提供实时编辑体验，支持自动保存和版本控制

## 核心功能模块

### 1. 小说管理

小说管理模块允许用户创建、编辑、删除和组织多个小说项目。

- 支持小说标题、作者、描述等基本信息管理
- 提供小说状态跟踪（草稿、写作中、已完成、已废弃）
- 允许固定常用小说，方便快速访问
- 提供小说统计信息，包括总字数、章节数等

主要实现位于 `src/core/novel_manager.rs`，通过 `NovelManager` 类提供相关功能。

### 2. 章节管理

章节管理模块支持以树状结构组织小说内容，实现卷、章、场景的层级关系。

- 支持创建不同类型的章节（卷、章、场景）
- 提供章节重排序和层级调整功能
- 实现章节内容的编辑和保存
- 支持章节归档功能

核心实现位于 `src/core/chapter_manager.rs`，通过 `ChapterManager` 类和 `ChapterTree` 数据结构提供功能。

### 3. 灵感管理

灵感管理模块帮助作者记录创作灵感，并与小说内容关联。

- 支持创建带标题和内容的灵感记录
- 提供标签系统，方便分类和查找
- 允许将灵感与特定章节关联
- 提供灵感统计和趋势分析
- 支持灵感置顶功能

主要实现位于 `src/core/inspiration_manager.rs`，通过 `InspirationManager` 类提供相关功能。

### 4. 版本控制

版本控制模块自动保存章节内容的历史版本，支持回溯查看和恢复。

- 自动记录章节修改历史
- 支持查看不同版本之间的差异
- 允许恢复到之前的版本

实现位于 `src/core/version_manager.rs`，通过 `VersionManager` 类提供功能。

### 5. 统计功能

统计模块提供写作进度和习惯的详细分析。

- 实时字数统计（总字数、章节字数）
- 写作进度跟踪
- 灵感统计（数量、标签分布、趋势等）
- 写作报告生成

核心实现位于 `src/core/stats_manager.rs`，通过 `StatsManager` 类提供功能。

## 核心 API/类/函数

### 1. NovelManager

管理小说相关操作的核心类。

```rust
pub struct NovelManager;

impl NovelManager {
    // 创建新小说
    pub async fn create_novel(&self, title: &str, author: Option<&str>) -> Result<Novel>
    
    // 获取所有小说
    pub async fn get_all_novels(&self) -> Result<Vec<Novel>>
    
    // 根据ID获取小说
    pub async fn get_novel_by_id(&self, novel_id: i64) -> Result<Option<Novel>>
    
    // 更新小说信息
    pub async fn update_novel_title(&self, novel_id: i64, new_title: &str) -> Result<()>
    pub async fn update_novel_author(&self, novel_id: i64, author: &str) -> Result<()>
    pub async fn update_novel_description(&self, novel_id: i64, description: &str) -> Result<()>
}
```

### 2. ChapterManager

管理章节相关操作的核心类，支持树状结构的章节组织。

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
    
    // 移动章节（调整父节点和位置）
    pub async fn move_chapter(
        &self,
        chapter_id: i64,
        new_parent_id: Option<i64>,
        new_position: usize
    ) -> Result<()>
}
```

### 3. InspirationManager

管理灵感记录的核心类。

```rust
pub struct InspirationManager;

impl InspirationManager {
    // 获取灵感统计信息
    pub async fn get_inspiration_stats(&self, novel_id: i64) -> Result<InspirationStats>
    
    // 创建新灵感
    pub async fn create_inspiration(
        &self,
        novel_id: i64,
        title: &str,
        content: &str,
        tags: Vec<String>
    ) -> Result<Inspiration>
    
    // 获取小说的所有灵感
    pub async fn get_inspirations_by_novel(&self, novel_id: i64) -> Result<Vec<Inspiration>>
}
```

### 4. Database

数据库操作的核心类，提供数据持久化功能。

```rust
pub struct Database {
    conn: Connection,
}

impl Database {
    // 创建数据库连接
    pub fn new() -> Result<Self>
    
    // 小说操作
    pub fn create_novel(&self, title: &str) -> Result<Novel>
    pub fn get_all_novels(&self) -> Result<Vec<Novel>>
    pub fn update_novel(&self, novel: &Novel) -> Result<()>
    
    // 章节操作
    pub fn create_chapter(&self, novel_id: i64, title: &str, parent_id: Option<i64>) -> Result<Chapter>
    pub fn get_chapters_by_novel(&self, novel_id: i64) -> Result<Vec<Chapter>>
    
    // 灵感操作
    pub fn create_inspiration(&self, inspiration: &Inspiration) -> Result<i64>
    pub fn get_inspirations_by_novel(&self, novel_id: i64) -> Result<Vec<Inspiration>>
}
```

### 5. AppState

应用状态管理，保存当前编辑上下文。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub current_novel: Option<Novel>,
    pub current_chapter: Option<Chapter>,
    pub chapter_tree: Option<ChapterTree>,
    pub editing_mode: EditingMode,
    pub view_settings: ViewSettings,
    pub auto_save_enabled: bool,
    pub last_save_time: Option<chrono::DateTime<chrono::Utc>>,
}
```

## 技术栈与依赖

| 技术/依赖 | 用途 | 来源 |
|----------|------|------|
| Rust | 主要开发语言 | <mcfile name="Cargo.toml" path="d:\Dev\workspace\rustws\dm\novel-writer\Cargo.toml"></mcfile> |
| Dioxus | UI 框架，提供类似 React 的组件系统 | <mcfile name="src\main.rs" path="d:\Dev\workspace\rustws\dm\novel-writer\src\main.rs"></mcfile> |
| SQLite | 数据库存储 | <mcfile name="src\db\mod.rs" path="d:\Dev\workspace\rustws\dm\novel-writer\src\db\mod.rs"></mcfile> |
| Rusqlite | SQLite 的 Rust 绑定 | <mcfile name="src\db\mod.rs" path="d:\Dev\workspace\rustws\dm\novel-writer\src\db\mod.rs"></mcfile> |
| Serde | 序列化和反序列化 | <mcfile name="src\config.rs" path="d:\Dev\workspace\rustws\dm\novel-writer\src\config.rs"></mcfile> |
| Chrono | 日期和时间处理 | <mcfile name="src\config.rs" path="d:\Dev\workspace\rustws\dm\novel-writer\src\config.rs"></mcfile> |
| Log | 日志系统 | <mcfile name="src\init.rs" path="d:\Dev\workspace\rustws\dm\novel-writer\src\init.rs"></mcfile> |

## 关键模块与典型用例

### 小说管理模块

**功能说明**：用于创建、编辑和管理多个小说项目。

**配置与依赖**：
* 依赖 `db` 模块提供数据持久化
* 通过 `NovelManager` 类提供功能接口

**使用示例**：

```rust
// 创建新小说
let novel_manager = NovelManager::new();
let novel = novel_manager.create_novel("我的小说标题", Some("作者名")).await?;

// 获取所有小说
let novels = novel_manager.get_all_novels().await?;

// 更新小说信息
novel_manager.update_novel_title(novel.id, "新标题").await?;
```

### 章节管理模块

**功能说明**：以树状结构组织小说章节，支持层级关系。

**配置与依赖**：
* 依赖 `db` 模块进行数据存储
* 使用 `ChapterTree` 数据结构管理层级关系

**使用示例**：

```rust
// 创建章节管理器
let chapter_manager = ChapterManager::new();

// 创建新章节
let chapter = chapter_manager.create_chapter(novel_id, "第一章", None).await?;

// 创建子章节（场景）
let scene = chapter_manager.create_chapter(novel_id, "场景一", Some(chapter.id)).await?;

// 获取并构建章节树
let chapters = db.get_chapters_by_novel(novel_id)?;
let chapter_tree = chapter_manager.build_chapter_tree(chapters);
```

### 灵感管理模块

**功能说明**：记录创作灵感，支持标签和章节关联。

**配置与依赖**：
* 依赖 `db` 模块进行数据存储
* 提供统计功能分析灵感趋势

**使用示例**：

```rust
// 创建灵感管理器
let inspiration_manager = InspirationManager::new();

// 创建新灵感
let tags = vec!["情节", "角色", "高潮"].iter().map(|s| s.to_string()).collect();
let inspiration = inspiration_manager.create_inspiration(
    novel_id,
    "关键情节灵感",
    "主角在关键时刻做出的选择...",
    tags
).await?;

// 获取灵感统计
let stats = inspiration_manager.get_inspiration_stats(novel_id).await?;
```

## 配置、部署与开发

### 配置

应用配置通过 `AppConfig` 结构体管理，主要配置项包括：

- 应用名称和版本
- 数据库路径
- 自动保存间隔
- 主题设置（亮色/暗色）
- 编辑器设置（拼写检查、制表符大小、自动换行等）

配置文件默认存储在：
- Windows: `%APPDATA%\NovelWriter\config.json`

### 部署

作为 Rust 应用，可以通过以下步骤构建和运行：

```bash
# 构建应用
cargo build --release

# 运行应用
cargo run
```

### 开发环境设置

1. 确保安装了 Rust 和 Cargo
2. 克隆仓库后，运行 `cargo build` 安装依赖
3. 使用 `cargo run` 启动开发服务器

## 监控与维护

应用使用标准的日志系统记录操作和错误信息，日志级别可通过环境变量调整。主要日志包括：

- 应用启动和初始化日志
- 数据库操作日志
- 错误和异常日志

常见问题排查：

1. 数据库连接失败：检查 `data` 目录权限和存在性
2. 应用崩溃：查看日志文件中的错误信息
3. 数据丢失：确保开启自动保存功能，定期备份 `data` 目录

## 总结与亮点回顾

Novel Writer 是一个功能全面的小说写作工具，具有以下亮点：

- 树状章节结构，支持复杂的小说组织
- 强大的灵感管理系统，帮助捕捉创作灵感
- 自动版本控制，防止内容意外丢失
- 详细的统计分析，帮助追踪写作进度
- 使用 Rust 语言开发，具有出色的性能和稳定性
- 现代化的 UI 设计，提供良好的用户体验

这个工具适合各类小说创作者使用，无论是初学者还是专业作家，都能从中获益。