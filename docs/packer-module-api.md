# Packer 模块接口文档

## 模块概述

Packer 模块负责将项目文件打包输出，支持两种输出方式：ZIP 压缩和目录复制。

## 模块结构

```
src-tauri/src/packer/
├── mod.rs       # 模块入口
├── zipper.rs    # ZIP 压缩打包
└── copier.rs    # 目录复制打包
```

## zipper.rs - ZIP 打包

### `pack_to_zip`

将源目录打包为 ZIP 文件。

```rust
pub fn pack_to_zip(
    source: &Path,
    output: &Path,
    excluded_paths: &[String],
    on_progress: impl FnMut(ProgressEvent),
) -> std::io::Result<PackResult>
```

**参数：**

| 参数 | 类型 | 说明 |
|------|------|------|
| source | &Path | 源项目目录 |
| output | &Path | 输出 ZIP 文件路径 |
| excluded_paths | &[String] | 需要排除的路径列表（绝对路径） |
| on_progress | impl FnMut(ProgressEvent) | 进度回调 |

**返回：** `std::io::Result<PackResult>`

```json
{
  "success": true,
  "output_path": "C:/output/project.zip",
  "total_files": 42,
  "output_size": 102400,
  "message": "打包完成，共 42 个文件"
}
```

## copier.rs - 目录复制

### `pack_to_directory`

将源目录复制到目标位置（排除指定路径）。

```rust
pub fn pack_to_directory(
    source: &Path,
    output: &Path,
    excluded_paths: &[String],
    on_progress: impl FnMut(ProgressEvent),
) -> std::io::Result<PackResult>
```

**参数：**

| 参数 | 类型 | 说明 |
|------|------|------|
| source | &Path | 源项目目录 |
| output | &Path | 输出目录路径 |
| excluded_paths | &[String] | 需要排除的路径列表（绝对路径） |
| on_progress | impl FnMut(ProgressEvent) | 进度回调 |

**返回：** `std::io::Result<PackResult>`

```json
{
  "success": true,
  "output_path": "C:/output/project_clean",
  "total_files": 42,
  "output_size": 204800,
  "message": "复制完成，共 42 个文件"
}
```

## 共用类型

### PackResult

```rust
pub struct PackResult {
    pub success: bool,
    pub output_path: String,
    pub total_files: u64,
    pub output_size: u64,
    pub message: String,
}
```

### ProgressEvent

```rust
pub struct ProgressEvent {
    pub percent: f64,       // 0.0 ~ 100.0
    pub current_file: String, // 当前处理的文件相对路径
}
```
