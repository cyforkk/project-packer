<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";

const props = defineProps<{ modelValue: string }>();
const emit = defineEmits<{ "update:modelValue": [value: string] }>();

async function selectFolder() {
  const selected = await open({ directory: true, multiple: false });
  if (selected) {
    emit("update:modelValue", selected as string);
  }
}
</script>

<template>
  <el-input :model-value="props.modelValue" placeholder="请选择项目文件夹" readonly>
    <template #append>
      <el-button @click="selectFolder">
        <el-icon><FolderOpened /></el-icon>
        选择文件夹
      </el-button>
    </template>
  </el-input>
</template>
