<script setup lang="ts">
import type { ProjectInfo } from "../types";
import { formatSize, projectTypeLabels } from "../types";

defineProps<{ info: ProjectInfo | null }>();
</script>

<template>
  <el-card shadow="never" v-if="info">
    <template #header><span>项目信息</span></template>
    <el-descriptions :column="2" border>
      <el-descriptions-item label="项目名称">{{ info.name }}</el-descriptions-item>
      <el-descriptions-item label="项目类型">
        {{ projectTypeLabels[info.project_type] }}
      </el-descriptions-item>
      <el-descriptions-item label="总大小">{{ formatSize(info.total_size) }}</el-descriptions-item>
      <el-descriptions-item label="清理后预计">
        <span style="color: #67c23a; font-weight: bold">
          {{ formatSize(info.estimated_clean_size) }}
        </span>
      </el-descriptions-item>
      <el-descriptions-item label="文件数">{{ info.file_count }} 个</el-descriptions-item>
      <el-descriptions-item label="可清理项">
        {{ info.cleanable_items.length }} 个
      </el-descriptions-item>
    </el-descriptions>
  </el-card>
</template>
