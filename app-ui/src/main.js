import { createApp } from 'vue';
import { createRouter, createWebHashHistory } from 'vue-router';
import { createVuetify } from 'vuetify';
import { aliases, mdi } from 'vuetify/iconsets/mdi-svg';
import '@/styles/main.scss';
import { VueToastr } from 'vue-toastr';
import 'vue-toastr/dist/style.css';

import App from './App.vue';
import Files from './components/Files.vue';
import Scan from './components/Scan.vue';
import Settings from './components/Settings.vue';

const vuetify = createVuetify({
  display: {
    thresholds: {
      md: 960,
      lg: 1280,
      xl: 1920,
      xxl: 2560,
    },
  },
  defaults: {
    VBtn: {
      variant: 'tonal',
    },
    VSelect: {
      variant: 'plain',
    },
    VTextField: {
      variant: 'plain',
    }
  },
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: {
      mdi,
    }
  },
  theme: {
    themes: {
      light: {
        colors: {
          primary: 'rgb(25, 118, 210)',
          secondary: 'rgb(66, 66, 66)'
        }
      },
      dark: {
        colors: {
          primary: 'rgb(25, 118, 210)',
          secondary: 'rgb(66, 66, 66)'
        }
      }
    }
  }
});

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/files', component: Files },
    { path: '/settings', component: Settings },
    { path: '/scan', component: Scan },
    { path: '/', component: Scan }
  ]
});

createApp(App)
  .use(vuetify)
  .use(router)
  .use(VueToastr)
  .mount('#app');
