<script setup lang="ts">
import { computed } from "vue";
import type { CleanableItem } from "../types";
import { formatSize } from "../types";

const props = defineProps<{
  visible: boolean;
  projectPath: string;
  cleanableItems: CleanableItem[];
  selectedPaths: string[];
}>();

defineEmits<{
  "update:visible": [value: boolean];
}>();

interface TreeNode {
  label: string;
  isCleaned: boolean;
  size?: string;
  children?: TreeNode[];
}

const treeData = computed<TreeNode[]>(() => {
  const cleanedPathSet = new Set(props.selectedPaths);
  const rootName = props.projectPath.split(/[/\\]/).pop() || "project";
  const children: TreeNode[] = props.cleanableItems.map((item) => ({
    label: item.name + (item.is_dir ? "/" : ""),
    isCleaned: cleanedPathSet.has(item.path),
    size: formatSize(item.size),
  }));

  return [{ label: rootName, isCleaned: false, children }];
});
</script>

<template>
  <el-dialog
    :model-value="visible"
    @update:model-value="$emit('update:visible', $event)"
    title="文件预览"
    width="600px"
    top="5vh"
  >
    <div style="margin-bottom: 12px">
      <el-tag type="success" effect="plain" style="margin-right: 8px">保留</el-tag>
      <el-tag type="danger" effect="plain">将清理</el-tag>
    </div>

    <el-tree :data="treeData" default-expand-all>
      <template #default="{ data }">
        <span :style="{ color: data.isCleaned ? '#f56c6c' : '#67c23a' }">
          {{ data.label }}
        </span>
        <span v-if="data.size" style="color: #909399; margin-left: 8px; font-size: 12px">
          ({{ data.size }})
        </span>
      </template>
    </el-tree>
  </el-dialog>
</template>
