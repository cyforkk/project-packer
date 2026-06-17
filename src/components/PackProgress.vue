<script setup lang="ts">
import type { ProgressEvent } from "../types";

defineProps<{
  packing: boolean;
  progress: ProgressEvent | null;
  result: { success: boolean; message: string } | null;
}>();
</script>

<template>
  <el-card shadow="never" v-if="packing || result">
    <template #header><span>打包进度</span></template>
    <el-progress
      :percentage="progress?.percent || 0"
      :status="result?.success ? 'success' : result ? 'exception' : undefined"
      :stroke-width="20"
      style="margin-bottom: 8px"
    />
    <p v-if="progress && packing" style="color: #909399; font-size: 12px; word-break: break-all">
      {{ progress.current_file }}
    </p>
    <el-alert
      v-if="result"
      :title="result.message"
      :type="result.success ? 'success' : 'error'"
      show-icon
    />
  </el-card>
</template>
