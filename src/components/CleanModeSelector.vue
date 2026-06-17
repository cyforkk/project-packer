<script setup lang="ts">
import type { CleanMode } from "../types";

defineProps<{ modelValue: CleanMode }>();
defineEmits<{ "update:modelValue": [value: CleanMode] }>();

const modes: { value: CleanMode; label: string; desc: string }[] = [
  { value: "Basic", label: "基础清理", desc: "只去依赖目录 (node_modules, target, venv 等)" },
  { value: "Medium", label: "中等清理", desc: "基础 + IDE 配置 (.idea, .vscode 等)" },
  { value: "Full", label: "全面清理", desc: "中等 + 构建产物 + .git + 日志 + 临时文件" },
  { value: "Custom", label: "自定义", desc: "我来选择要清理哪些项" },
];
</script>

<template>
  <el-card shadow="never">
    <template #header><span>清理模式</span></template>
    <el-radio-group :model-value="modelValue" @update:model-value="$emit('update:modelValue', $event)">
      <el-radio v-for="m in modes" :key="m.value" :value="m.value" style="display: block; margin-bottom: 12px">
        <strong>{{ m.label }}</strong>
        <span style="color: #909399; margin-left: 8px">{{ m.desc }}</span>
      </el-radio>
    </el-radio-group>
  </el-card>
</template>
