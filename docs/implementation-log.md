# 项目打包工具 - 实现记录

## 技术栈

| 层 | 技术 | 版本 |
|----|------|------|
| 桌面框架 | Tauri | 2.x |
| 前端 | Vue 3 + TypeScript | 3.5 |
| UI 组件库 | Element Plus | 2.x |
| 后端 | Rust | 1.96 |
| 文件遍历 | walkdir crate | 2.x |
| 压缩 | zip crate | 2.x |

## 实现过程

### 1. 项目脚手架搭建
- 使用 Tauri 2.x + Vue 3 + TypeScript 模板
- 窗口尺寸设为 1000x700

### 2. Rust 后端实现
- **类型定义** — 定义 ProjectType、CleanCategory、CleanMode、OutputType 等枚举和结构体，全部实现 Serialize/Deserialize 用于前后端通信
- **项目检测器** — 通过特征文件（pom.xml, package.json 等）自动识别项目类型，支持 7 种项目类型
- **清理规则引擎** — 27 条内置规则，支持三种匹配模式（精确匹配、扩展名匹配、目录名匹配），包含 prototypes 目录排除规则，归类为依赖/冗余目录
- **目录扫描器** — 先扫描根目录的直接子项（目录匹配），再递归查找文件模式匹配（如 *.log）
- **打包引擎** — ZIP 压缩使用 Deflate 算法，路径分隔符统一转为 `/`；目录复制自动创建中间目录
- **Tauri 命令** — scan_project 和 pack_project 都使用 spawn_blocking 避免阻塞 Tauri 主线程，进度通过 Event 系统推送

### 3. Vue 前端实现
- 7 个独立组件，每个组件单一职责
- 选择文件夹后自动触发扫描（watch projectPath）
- 切换清理模式自动更新选中项（watch cleanMode）
- 自定义模式允许手动勾选/取消
- 打包进度通过 Tauri Event 实时更新

## 遇到的问题与解决方案

### 1. tsconfig 配置问题
- **问题** — tsconfig.node.json 缺少 `composite: true`，导致 vue-tsc 编译失败
- **解决** — 添加 `"composite": true` 并将 `"noEmit": true` 改为 `"emitDeclarationOnly": true`

### 2. Tauri 插件依赖
- **问题** — 脚手架默认安装 tauri-plugin-shell，但我们需要 dialog 和 opener
- **解决** — Cargo.toml 添加 tauri-plugin-dialog 和 tauri-plugin-opener，npm 安装对应前端包，移除 shell 插件

### 3. capabilities 配置
- **问题** — Tauri 2.x 需要 capabilities 配置才能使用 dialog 插件
- **解决** — 创建 `src-tauri/capabilities/default.json`，添加 dialog:allow-open 和 dialog:allow-save 权限

### 4. Rust PATH 问题
- **问题** — rustup 安装后，非交互式终端中 PATH 未自动加载
- **解决** — 手动执行 `export PATH="$HOME/.cargo/bin:$PATH"`

### 5. prototypes 规则匹配范围
- **问题** — prototypes 是目录名，使用精确匹配会把同名普通文件也识别为可清理项
- **解决** — 将 prototypes 规则改为目录名匹配，并增加同名文件不匹配的单元测试

### 6. prototypes 列表不显示
- **问题** — prototypes 归类为构建产物时，基础/适中模式的可清理列表不会显示该目录
- **解决** — 将 prototypes 归类为依赖/冗余目录，使基础、适中、完全模式都能显示并默认排除，同时增加扫描层单元测试

## 测试覆盖

- **Rust 单元测试** — 33 个测试，覆盖类型序列化、项目检测、规则匹配、目录扫描、ZIP 打包、目录复制
- **前端 TypeScript** — vue-tsc 类型检查通过
- **构建验证** — Rust release 构建和 Vite 生产构建均成功
