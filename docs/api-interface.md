# 项目打包工具 - 后端接口文档

## Tauri Commands

### scan_project

扫描项目目录，识别项目类型和可清理项。

**输入参数：**

| 参数 | 类型 | 说明 |
|------|------|------|
| path | string | 项目文件夹的绝对路径 |

**返回 JSON：**

```json
{
  "path": "C:/projects/my-app",
  "name": "my-app",
  "project_type": "NodeJS",
  "total_size": 156000000,
  "estimated_clean_size": 12000000,
  "file_count": 1234,
  "cleanable_items": [
    {
      "path": "C:/projects/my-app/node_modules",
      "name": "node_modules",
      "size": 140000000,
      "category": "Dependency",
      "is_dir": true
    },
    {
      "path": "C:/projects/my-app/.git",
      "name": ".git",
      "size": 4000000,
      "category": "VCS",
      "is_dir": true
    }
  ]
}
```

**project_type 可选值：** Java, NodeJS, Python, Rust, Go, DotNet, PHP, Unknown

**cleanable_items 中 category 可选值：**
- Dependency — 依赖目录 (node_modules, target, venv 等)
- BuildArtifact — 构建产物 (dist, build, out 等)
- IdeConfig — IDE 配置 (.idea, .vscode 等)
- VCS — 版本控制 (.git, .svn)
- Log — 日志文件 (*.log)
- Temp — 临时文件 (.cache, .DS_Store 等)

---

### pack_project

执行打包操作。

**输入参数（PackConfig）：**

| 参数 | 类型 | 说明 |
|------|------|------|
| source_path | string | 源项目路径 |
| output_path | string | 输出路径（ZIP 文件路径或目标目录） |
| clean_mode | string | "Basic" / "Medium" / "Full" / "Custom" |
| excluded_paths | string[] | 要排除的路径列表 |
| output_type | string | "Zip" / "Copy" |

**clean_mode 说明：**
- Basic — 只清理依赖目录
- Medium — 依赖目录 + IDE 配置
- Full — 依赖目录 + 构建产物 + IDE 配置 + 版本控制 + 日志 + 临时文件
- Custom — 根据 excluded_paths 自定义

**返回 JSON（PackResult）：**

```json
{
  "success": true,
  "output_path": "C:/output/my-app.zip",
  "total_files": 45,
  "output_size": 8500000,
  "message": "打包完成，共 45 个文件"
}
```

---

## Events

### pack-progress

打包过程中的进度事件，通过 Tauri Event 系统推送。

**事件名：** `pack-progress`

**数据格式：**

```json
{
  "percent": 75.5,
  "current_file": "src/utils/helper.ts"
}
```

前端通过 `listen("pack-progress", callback)` 监听。

---

## 支持的自动识别项目类型

| 特征文件 | 项目类型 |
|----------|----------|
| pom.xml / build.gradle | Java |
| package.json | Node.js |
| requirements.txt / Pipfile / setup.py | Python |
| Cargo.toml | Rust |
| go.mod | Go |
| *.sln / *.csproj | .NET |
| composer.json | PHP |

---

## 内置清理规则

| 名称 | 匹配方式 | 分类 | 清理模式 |
|------|----------|------|----------|
| node_modules | 精确匹配 | Dependency | Basic+ |
| target | 精确匹配 | Dependency | Basic+ |
| venv / .venv | 精确匹配 | Dependency | Basic+ |
| vendor | 精确匹配 | Dependency | Basic+ |
| .m2 | 精确匹配 | Dependency | Basic+ |
| __pycache__ | 目录名 | Dependency | Basic+ |
| .eggs | 精确匹配 | Dependency | Basic+ |
| prototypes | 目录名 | Dependency | Basic+ |
| dist / build / out | 精确匹配 | BuildArtifact | Full |
| .next / .nuxt | 精确匹配 | BuildArtifact | Full |
| .idea / .vscode | 精确匹配 | IdeConfig | Medium+ |
| .settings / .project / .classpath | 精确匹配 | IdeConfig | Medium+ |
| .git / .svn | 精确匹配 | VCS | Full |
| *.log | 扩展名 | Log | Full |
| *.tmp | 扩展名 | Temp | Full |
| .cache | 精确匹配 | Temp | Full |
| .DS_Store / Thumbs.db | 精确匹配 | Temp | Full |
