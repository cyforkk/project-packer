<script setup lang="ts">
import { save, open } from "@tauri-apps/plugin-dialog";
import type { OutputType } from "../types";

const props = defineProps<{
  outputType: OutputType;
  outputPath: string;
  projectName: string;
}>();

const emit = defineEmits<{
  "update:outputType": [value: OutputType];
  "update:outputPath": [value: string];
}>();

async function browseZip() {
  const filePath = await save({
    defaultPath: `${props.projectName}.zip`,
    filters: [{ name: "ZIP 文件", extensions: ["zip"] }],
  });
  if (filePath) {
    emit("update:outputPath", filePath);
  }
}

async function browseDir() {
  const dir = await open({ directory: true, multiple: false });
  if (dir) {
    emit("update:outputPath", dir as string);
  }
}
</script>

<template>
  <el-card shadow="never">
    <template #header><span>输出方式</span></template>
    <el-radio-group
      :model-value="outputType"
      @update:model-value="emit('update:outputType', $event)"
      style="margin-bottom: 12px"
    >
      <el-radio value="Zip">ZIP 压缩包</el-radio>
      <el-radio value="Copy">复制到目录</el-radio>
    </el-radio-group>

    <el-input :model-value="outputPath" placeholder="请选择输出路径" readonly>
      <template #append>
        <el-button @click="outputType === 'Zip' ? browseZip() : browseDir()">
          浏览
        </el-button>
      </template>
    </el-input>
  </el-card>
</template>
