# 项目打包工具

一键清理项目依赖和冗余文件，打包纯净源码用于服务器部署。

## 功能特性

- **智能识别** - 自动检测项目类型（Java / Node.js / Python / Rust / Go / .NET / PHP）
- **分级清理** - 四种清理模式，按需选择清理范围
- **灵活输出** - 支持 ZIP 压缩包或复制到目录
- **实时预览** - 打包前可预览将被清理的文件，支持自定义勾选
- **进度显示** - 打包过程中实时显示进度百分比和当前文件

## 清理模式

| 模式 | 清理范围 |
|------|---------|
| **基础** | 仅清理依赖目录（node_modules、venv、target 等） |
| **适中** | 依赖目录 + IDE 配置文件（.idea、.vscode 等） |
| **完全** | 依赖 + 构建产物 + IDE 配置 + 版本控制 + 日志 + 临时文件 |
| **自定义** | 用户手动勾选需要清理的项目 |

## 内置清理规则

| 分类 | 清理目标 |
|------|---------|
| 依赖目录 | `node_modules` `target` `venv` `.venv` `vendor` `.m2` `__pycache__` `.eggs` `prototypes` |
| 构建产物 | `dist` `build` `out` `.next` `.nuxt` |
| IDE 配置 | `.idea` `.vscode` `.settings` `.project` `.classpath` |
| 版本控制 | `.git` `.svn` |
| 日志文件 | `*.log` |
| 临时文件 | `*.tmp` `.cache` `.DS_Store` `Thumbs.db` |
| 文档目录 | `docs`（默认不勾选，需手动选择） |

## 技术栈

- **桌面框架：** Tauri 2.x
- **前端：** Vue 3 + TypeScript + Element Plus
- **后端：** Rust
- **构建：** Vite + Cargo

## 开发环境

### 前置要求

- [Node.js](https://nodejs.org/) (>= 18)
- [Rust](https://www.rust-lang.org/tools/install) (>= 1.77)
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (Windows)

### 安装与运行

```bash
# 克隆项目
git clone <repo-url>
cd project-packer

# 安装前端依赖
npm install

# 开发模式运行
npm run tauri dev
```

### 构建发布版

```bash
npm run tauri build
```

构建产物位于：

```
src-tauri/target/release/
├── project-packer.exe                              # 独立可执行文件
└── bundle/nsis/项目打包工具_0.1.0_x64-setup.exe     # NSIS 安装包
```

## 项目结构

```
project-packer/
├── src/                          # 前端源码 (Vue 3)
│   ├── App.vue                   # 主应用组件
│   ├── types.ts                  # TypeScript 类型定义
│   └── components/
│       ├── ProjectSelector.vue   # 项目选择器
│       ├── ProjectInfo.vue       # 项目信息面板
│       ├── CleanModeSelector.vue # 清理模式选择
│       ├── CleanableItemsList.vue# 可清理项列表
│       ├── OutputOptions.vue     # 输出选项
│       ├── PackProgress.vue      # 打包进度
│       └── FilePreviewDialog.vue # 文件预览对话框
├── src-tauri/                    # 后端源码 (Rust)
│   ├── src/
│   │   ├── commands/             # Tauri 命令（scan_project / pack_project）
│   │   ├── scanner/              # 项目扫描（类型检测 + 目录遍历）
│   │   ├── rules/                # 清理规则引擎
│   │   ├── packer/               # 打包执行（ZIP / 目录复制）
│   │   └── types.rs              # 共享类型定义
│   └── tauri.conf.json           # Tauri 配置
└── docs/                         # 文档
```

## 使用流程

1. **选择项目** - 点击"选择文件夹"按钮，选择要打包的项目目录
2. **查看信息** - 自动扫描并显示项目类型、大小、可清理项
3. **选择模式** - 选择清理模式（基础 / 适中 / 完全 / 自定义）
4. **设置输出** - 选择输出格式（ZIP / 复制到目录）和保存路径
5. **开始打包** - 点击"开始打包"，等待完成

## 许可证

MIT
