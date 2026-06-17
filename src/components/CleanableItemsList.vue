<script setup lang="ts">
import { computed } from "vue";
import type { CleanableItem, CleanMode } from "../types";
import { formatSize, categoryLabels } from "../types";

const props = defineProps<{
  items: CleanableItem[];
  cleanMode: CleanMode;
  selectedPaths: string[];
}>();

const emit = defineEmits<{
  "update:selectedPaths": [value: string[]];
}>();

const visibleItems = computed(() => {
  if (props.cleanMode === "Custom") return props.items;
  const categoryMap: Record<string, string[]> = {
    Basic: ["Dependency", "Docs"],
    Medium: ["Dependency", "IdeConfig", "Docs"],
    Full: ["Dependency", "BuildArtifact", "IdeConfig", "VCS", "Log", "Temp", "Docs"],
  };
  const allowed = categoryMap[props.cleanMode] || [];
  return props.items.filter((item) => allowed.includes(item.category));
});

const totalCleanSize = computed(() =>
  visibleItems.value.reduce((sum, item) => sum + item.size, 0)
);

function toggleItem(path: string) {
  const newSelected = props.selectedPaths.includes(path)
    ? props.selectedPaths.filter((p) => p !== path)
    : [...props.selectedPaths, path];
  emit("update:selectedPaths", newSelected);
}

function toggleAll() {
  if (props.selectedPaths.length === visibleItems.value.length) {
    emit("update:selectedPaths", []);
  } else {
    emit("update:selectedPaths", visibleItems.value.map((i) => i.path));
  }
}
</script>

<template>
  <el-card shadow="never" v-if="items.length > 0">
    <template #header>
      <div style="display: flex; justify-content: space-between; align-items: center">
        <span>将被清理的目录/文件</span>
        <span style="color: #e6a23c">共 {{ formatSize(totalCleanSize) }}</span>
      </div>
    </template>

    <el-checkbox
      v-if="cleanMode === 'Custom'"
      :model-value="selectedPaths.length === visibleItems.length && visibleItems.length > 0"
      @change="toggleAll"
      style="margin-bottom: 8px"
    >
      全选
    </el-checkbox>

    <div v-for="item in visibleItems" :key="item.path" style="padding: 6px 0; border-bottom: 1px solid #f0f0f0">
      <el-checkbox
        :model-value="selectedPaths.includes(item.path)"
        @change="toggleItem(item.path)"
        :disabled="cleanMode !== 'Custom'"
      >
        <span>{{ item.name }}{{ item.is_dir ? "/" : "" }}</span>
        <el-tag size="small" type="info" style="margin-left: 8px">
          {{ categoryLabels[item.category] }}
        </el-tag>
        <span style="color: #909399; margin-left: 8px">{{ formatSize(item.size) }}</span>
      </el-checkbox>
    </div>
  </el-card>
</template>
