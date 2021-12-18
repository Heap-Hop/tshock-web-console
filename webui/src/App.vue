<template>
  <n-layout style="height: 100vh;">
    <n-layout-header style="height: 64px; padding: 24px;" bordered><strong>泰拉瑞亚 TShock Console</strong></n-layout-header>
    <n-layout-content
      position="absolute"
      style="top: 64px;"
      content-style="padding: 24px;"
      :native-scrollbar="false"
    >
      <layout-content />
    </n-layout-content>
  </n-layout>
</template>

<script lang="ts" setup>
import { NLayout, NLayoutHeader, NLayoutContent } from 'naive-ui'
import LayoutContent from './components/LayoutContent.vue'

</script>

<style>
/* .n-layout-header,
.n-layout-footer {
  background: rgba(128, 128, 128, 0.2);
  padding: 24px;
}

.n-layout-content {
  background: rgba(128, 128, 128, 0.4);
} */
</style>
