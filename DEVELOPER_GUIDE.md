# Novel Writer 开发指南

本文档旨在帮助开发者了解 Novel Writer 项目的开发环境设置、代码规范和贡献流程。

## 目录

1. [环境设置](#环境设置)
2. [项目结构](#项目结构)
3. [代码规范](#代码规范)
4. [开发工作流](#开发工作流)
5. [测试](#测试)
6. [调试](#调试)
7. [提交规范](#提交规范)
8. [文档](#文档)
9. [发布流程](#发布流程)

## 环境设置

### 系统要求

- Rust 1.60+ （使用 [rustup](https://rustup.rs/) 安装）
- Cargo
- Git

### 安装步骤

1. 克隆代码仓库：

```bash
git clone https://github.com/your-username/novel-writer.git
cd novel-writer
```

2. 构建项目：

```bash
cargo build
```

3. 运行项目：

```bash
cargo run
```

### 开发依赖

开发过程中可能需要的额外工具：

- [Clippy](https://github.com/rust-lang/rust-clippy)：Rust linter，用于代码质量检查
- [rustfmt](https://github.com/rust-lang/rustfmt)：代码格式化工具
- [Dioxus CLI](https://dioxuslabs.com/cli)：用于 Dioxus 项目的命令行工具

## 项目结构

```
src/
├── assets/            # 静态资源
├── config.rs          # 配置管理
├── core/              # 核心业务逻辑
├── db/                # 数据库层
├── examples/          # 示例代码
├── init.rs            # 应用初始化
├── lib.rs             # 库入口
├── main.rs            # 主程序入口
├── ui/                # 用户界面
│   ├── app.rs         # 应用主组件
│   ├── components/    # UI组件
│   └── layouts/       # 布局组件
└── utils/             # 工具函数
```

### 模块职责

- **config.rs**：管理应用配置和设置
- **core/**：包含业务逻辑和各种管理器
- **db/**：处理数据模型和数据库操作
- **ui/**：实现用户界面组件
- **utils/**：提供通用工具函数

## 代码规范

### Rust 代码规范

项目遵循 Rust 的官方代码风格，使用 rustfmt 进行格式化：

```bash
cargo fmt
```

使用 Clippy 进行代码质量检查：

```bash
cargo clippy
```

### 命名约定

- **模块和文件**：使用小写字母和下划线（snake_case）
- **结构体和枚举**：使用大写字母开头的驼峰命名法（CamelCase）
- **函数和方法**：使用小写字母和下划线（snake_case）
- **常量**：使用全大写和下划线（SCREAMING_SNAKE_CASE）
- **变量和字段**：使用小写字母和下划线（snake_case）

### 注释规范

- 使用三斜杠注释（///）为公共API添加文档
- 使用双斜杠注释（//）添加代码说明
- 关键算法和复杂逻辑需要详细注释
- 避免冗余注释，代码本身应当具有可读性

### 文档测试

为公共函数和方法添加文档测试，示例：

```rust
/// 添加两个数字
/// 
/// # Examples
/// 
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## 开发工作流

### 分支策略

- **main**：主分支，包含稳定代码
- **develop**：开发分支，包含最新开发的功能
- **feature/xxx**：功能分支，用于开发新功能
- **bugfix/xxx**：修复分支，用于修复 bug

### 开发流程

1. 从 develop 分支创建新的功能分支：

```bash
git checkout develop
git checkout -b feature/your-feature-name
```

2. 实现功能并提交代码：

```bash
git add .
git commit -m "feat: 添加新功能"
```

3. 定期从 develop 分支拉取更新：

```bash
git checkout develop
git pull
git checkout feature/your-feature-name
git merge develop
```

4. 推送到远程仓库并创建 Pull Request：

```bash
git push origin feature/your-feature-name
```

## 测试

### 单元测试

为核心功能编写单元测试，存放在相应模块的 `#[cfg(test)]` 块中：

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_some_function() {
        let result = some_function();
        assert!(result.is_ok());
    }
}
```

运行单元测试：

```bash
cargo test
```

### 集成测试

集成测试存放在项目根目录的 `tests` 文件夹中：

```bash
cargo test --test integration
```

## 调试

### 日志系统

项目使用 `log` 库进行日志记录，可以通过设置环境变量调整日志级别：

```bash
RUST_LOG=debug cargo run
```

主要日志级别：
- **error**：错误信息
- **warn**：警告信息
- **info**：一般信息
- **debug**：调试信息
- **trace**：详细追踪信息

### 常见问题排查

1. **数据库连接失败**：检查数据目录权限和 SQLite 驱动
2. **UI 渲染问题**：检查组件状态和 Props 传递
3. **异步操作错误**：确保正确处理异步结果和错误

## 提交规范

遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范，提交信息格式如下：

```
<类型>[可选范围]: <描述>

[可选正文]

[可选脚注]
```

### 提交类型

- **feat**：新功能
- **fix**：bug 修复
- **docs**：文档更新
- **style**：代码风格更改（不影响功能）
- **refactor**：代码重构（不添加新功能或修复 bug）
- **perf**：性能优化
- **test**：添加或修改测试
- **chore**：构建过程或辅助工具变动

### 提交示例

```
feat(chapter): 添加章节重排序功能

允许用户通过拖放方式调整章节顺序，并自动更新数据库中的排序信息。

Closes #42
```

## 文档

### API 文档

使用 Rust 的文档注释为公共 API 编写文档：

```bash
cargo doc --open
```

### 架构文档

参考项目根目录下的 `ARCHITECTURE.md` 文件，了解系统架构。

### 用户文档

用户文档存放在项目根目录下的 `USER_GUIDE.md` 文件中。

## 发布流程

### 版本号规范

遵循 [Semantic Versioning](https://semver.org/) 规范：

- **MAJOR.MINOR.PATCH**
- **MAJOR**：不兼容的 API 变更
- **MINOR**：向后兼容的功能性新增
- **PATCH**：向后兼容的问题修复

### 发布步骤

1. 更新版本号（在 `Cargo.toml` 和 `config.rs` 中）
2. 更新 CHANGELOG.md
3. 提交更新：

```bash
git commit -m "chore: 准备 v1.2.0 发布"
```

4. 创建标签：

```bash
git tag v1.2.0
git push origin v1.2.0
```

5. 构建发布版本：

```bash
cargo build --release
```

## 贡献指南

我们欢迎社区贡献！以下是贡献的基本步骤：

1. Fork 项目仓库
2. 创建功能分支
3. 实现功能或修复 bug
4. 编写测试
5. 提交 Pull Request

在提交 Pull Request 前，请确保：

- 代码通过所有测试
- 没有 Clippy 警告
- 代码已正确格式化
- 添加了适当的文档
- 提交信息符合规范

## 联系方式

如有任何问题或建议，请通过以下方式联系：

- GitHub Issues
- 邮件：[项目邮箱地址]

## 许可

[项目许可信息]