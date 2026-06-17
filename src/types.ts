export type ProjectType = "Java" | "NodeJS" | "Python" | "Rust" | "Go" | "DotNet" | "PHP" | "Unknown";
export type CleanCategory = "Dependency" | "BuildArtifact" | "IdeConfig" | "VCS" | "Log" | "Temp" | "Docs";
export type CleanMode = "Basic" | "Medium" | "Full" | "Custom";
export type OutputType = "Zip" | "Copy";

export interface CleanableItem {
  path: string;
  name: string;
  size: number;
  category: CleanCategory;
  is_dir: boolean;
}

export interface ProjectInfo {
  path: string;
  name: string;
  project_type: ProjectType;
  total_size: number;
  estimated_clean_size: number;
  file_count: number;
  cleanable_items: CleanableItem[];
}

export interface PackConfig {
  source_path: string;
  output_path: string;
  clean_mode: CleanMode;
  excluded_paths: string[];
  output_type: OutputType;
}

export interface PackResult {
  success: boolean;
  output_path: string;
  total_files: number;
  output_size: number;
  message: string;
}

export interface ProgressEvent {
  percent: number;
  current_file: string;
}

export const projectTypeLabels: Record<ProjectType, string> = {
  Java: "Java",
  NodeJS: "Node.js",
  Python: "Python",
  Rust: "Rust",
  Go: "Go",
  DotNet: ".NET",
  PHP: "PHP",
  Unknown: "未知项目",
};

export const categoryLabels: Record<CleanCategory, string> = {
  Dependency: "依赖目录",
  BuildArtifact: "构建产物",
  IdeConfig: "IDE 配置",
  VCS: "版本控制",
  Log: "日志文件",
  Temp: "临时文件",
  Docs: "文档目录",
};

export function formatSize(bytes: number): string {
  if (bytes === 0) return "0 B";
  const units = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return (bytes / Math.pow(1024, i)).toFixed(1) + " " + units[i];
}
