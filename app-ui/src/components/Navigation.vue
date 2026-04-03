<template>
  <div>
    <v-app-bar class="elevation-0" app>
      <v-toolbar-title class="unselectable">Epson Perfection V600 Photo</v-toolbar-title>
      <v-tabs>
        <v-tab elevation="0" @click="go('/scan')"><v-icon class="mr-2" :icon="mdiCamera" />Scan</v-tab>
        <v-tab elevation="0" @click="go('/files')"><v-icon class="mr-2" :icon="mdiFileDocumentMultiple" />Files</v-tab>
        <v-tab elevation="0" @click="go('/settings')"><v-icon class="mr-2" :icon="mdiCog" />Settings</v-tab>
      </v-tabs>
    </v-app-bar>
  </div>
</template>

<script>
import { mdiCamera, mdiCog, mdiFileDocumentMultiple } from '@mdi/js';
export default {
  name: 'Navigation',

  setup() {
    return {
      mdiCamera,
      mdiCog,
      mdiFileDocumentMultiple
    };
  },

  methods: {
    go(location) {
      if (this.$route.path !== location) {
        this.$router.push(location);
      }
    }
  }
};
</script>

<style>
.unselectable {
  -moz-user-select: none;
  -webkit-user-select: none;
  -ms-user-select: none;
  user-select: none;
}
</style>
