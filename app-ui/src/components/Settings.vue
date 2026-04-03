<template>
  <div>
    <settings-section>
      <template #items>
        <settings-item>
          <template #description>
            Theme
            <br>
            If you use system theme and change it, you will need to reload the app.
          </template>
          <template #action>
            <div style="max-width: 10rem;">
              <v-select v-model="settings.theme" :items="themes" item-title="text" @update:model-value="reload" />
            </div>
          </template>
        </settings-item>
        <settings-item>
          <template #description>
            Show files after scan completes
          </template>
          <template #action>
            <div style="max-width: 10rem;">
              <v-switch v-model="settings.showFilesAfterScan" />
            </div>
          </template>
        </settings-item>
      </template>
    </settings-section>

    <div class="text-body-1 mb-4">
      OpenAPI documentation:
      <a target="_blank" href="api-docs">/api-docs</a>
    </div>
  </div>
</template>

<script>
import Constants from '../classes/constants';
import Storage from '../classes/storage';

import SettingsSection from './SettingsSection.vue';
import SettingsItem from './SettingsItem.vue';

const storage = Storage.instance();

export default {
  name: 'Settings',

  components: {
    SettingsSection,
    SettingsItem
  },

  emits: ['mask', 'notify'],

  data() {
    return {
      settings: storage.settings
    };
  },

  computed: {
    themes() {
      return Object.keys(Constants.Themes).map(t => ({
        text: t,
        value: Constants.Themes[t]
      }));
    }
  },

  watch: {
    settings: {
      handler(settings) {
        storage.settings = settings;
      },
      deep: true
    }
  },

  methods: {
    reload() {
      location.href = `?anticache=${Date.now()}${location.hash}`;
    }
  }
};
</script>
