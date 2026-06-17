<script setup lang="ts">
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ElMessage } from "element-plus";
import type { ProjectInfo, CleanMode, OutputType, ProgressEvent, PackResult } from "./types";

import ProjectSelector from "./components/ProjectSelector.vue";
import ProjectInfoPanel from "./components/ProjectInfo.vue";
import CleanModeSelector from "./components/CleanModeSelector.vue";
import CleanableItemsList from "./components/CleanableItemsList.vue";
import OutputOptions from "./components/OutputOptions.vue";
import PackProgress from "./components/PackProgress.vue";
import FilePreviewDialog from "./components/FilePreviewDialog.vue";

const projectPath = ref("");
const projectInfo = ref<ProjectInfo | null>(null);
const cleanMode = ref<CleanMode>("Full");
const selectedPaths = ref<string[]>([]);
const outputType = ref<OutputType>("Zip");
const outputPath = ref("");
const packing = ref(false);
const progress = ref<ProgressEvent | null>(null);
const packResult = ref<PackResult | null>(null);
const scanning = ref(false);
const showPreview = ref(false);

watch(projectPath, async (newPath) => {
  if (!newPath) return;
  scanning.value = true;
  projectInfo.value = null;
  selectedPaths.value = [];
  try {
    const info = await invoke<ProjectInfo>("scan_project", { path: newPath });
    projectInfo.value = info;
    // 默认不自动勾选 docs 类项，让用户手动决定
    selectedPaths.value = info.cleanable_items
      .filter((i) => i.category !== "Docs")
      .map((i) => i.path);
  } catch (e) {
    ElMessage.error(`扫描失败: ${e}`);
  } finally {
    scanning.value = false;
  }
});

listen<ProgressEvent>("pack-progress", (event) => {
  progress.value = event.payload;
});

watch(cleanMode, (mode) => {
  if (!projectInfo.value) return;
  if (mode === "Custom") return;
  const categoryMap: Record<string, string[]> = {
    Basic: ["Dependency"],
    Medium: ["Dependency", "IdeConfig"],
    Full: ["Dependency", "BuildArtifact", "IdeConfig", "VCS", "Log", "Temp"],
  };
  const allowed = categoryMap[mode] || [];
  selectedPaths.value = projectInfo.value.cleanable_items
    .filter((item) => allowed.includes(item.category))
    .map((item) => item.path);
});

async function startPack() {
  if (!projectInfo.value) {
    ElMessage.warning("请先选择项目文件夹");
    return;
  }
  if (!outputPath.value) {
    ElMessage.warning("请选择输出路径");
    return;
  }
  packing.value = true;
  progress.value = null;
  packResult.value = null;
  try {
    const result = await invoke<PackResult>("pack_project", {
      config: {
        source_path: projectInfo.value.path,
        output_path: outputPath.value,
        clean_mode: cleanMode.value,
        excluded_paths: selectedPaths.value,
        output_type: outputType.value,
      },
    });
    packResult.value = result;
    if (result.success) {
      ElMessage.success(result.message);
    }
  } catch (e) {
    packResult.value = {
      success: false,
      output_path: "",
      total_files: 0,
      output_size: 0,
      message: String(e),
    };
    ElMessage.error(`打包失败: ${e}`);
  } finally {
    packing.value = false;
  }
}
</script>

<template>
  <div class="app-container" v-loading="scanning" element-loading-text="扫描中...">
    <el-scrollbar>
      <div class="content">
        <h2 style="margin-bottom: 20px">项目打包工具</h2>

        <ProjectSelector v-model="projectPath" />

        <div style="margin-top: 16px">
          <ProjectInfoPanel :info="projectInfo" />
        </div>

        <div style="margin-top: 16px">
          <CleanModeSelector v-model="cleanMode" />
        </div>

        <div style="margin-top: 16px">
          <PackProgress :packing="packing" :progress="progress" :result="packResult" />
        </div>

        <div style="margin-top: 16px">
          <OutputOptions
            :output-type="outputType"
            :output-path="outputPath"
            :project-name="projectInfo?.name || 'project'"
            @update:output-type="outputType = $event"
            @update:output-path="outputPath = $event"
          />
        </div>

        <div style="margin-top: 20px; display: flex; gap: 12px; justify-content: flex-end">
          <el-button @click="projectPath = ''">重置</el-button>
          <el-button :disabled="!projectInfo" @click="showPreview = true">预览详情</el-button>
          <el-button
            type="primary"
            size="large"
            :loading="packing"
            :disabled="!projectInfo || !outputPath"
            @click="startPack"
          >
            开始打包
          </el-button>
        </div>

        <div style="margin-top: 16px; margin-bottom: 20px">
          <CleanableItemsList
            v-if="projectInfo"
            :items="projectInfo.cleanable_items"
            :clean-mode="cleanMode"
            v-model:selected-paths="selectedPaths"
          />
        </div>
      </div>
    </el-scrollbar>

    <FilePreviewDialog
      v-model:visible="showPreview"
      :project-path="projectPath"
      :cleanable-items="projectInfo?.cleanable_items || []"
      :selected-paths="selectedPaths"
    />
  </div>
</template>

<style scoped>
.app-container {
  height: 100%;
}
.content {
  max-width: 700px;
  margin: 0 auto;
}
</style>
