# Frontend Dependency Upgrade Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Upgrade all frontend dependencies to latest (Vuetify 4, Vite 8, ESLint 10, etc.), remove i18n, drop nodemon.

**Architecture:** Incremental upgrades in dependency groups, verifying the build compiles after each task. i18n removal replaces all `$t()`/`$d()` calls with plain English strings.

**Tech Stack:** Vue 3.5, Vuetify 4, Vite 8, ESLint 10 (flat config), vue-router 5

---

## File Map

**Modify:**
- `app-ui/package.json` — dependency versions, remove nodemon/i18n deps/scripts
- `app-ui/vite.config.js` — remove i18n plugin, update for Vite 8 compatibility
- `app-ui/src/main.js` — remove i18n setup, remove Vuetify locale adapter
- `app-ui/src/App.vue` — remove `$t()`, `$i18n`, RTL, locale handling
- `app-ui/src/components/Scan.vue` — remove `$t()`, simplify computed props, fix paper size names
- `app-ui/src/components/Files.vue` — remove `$t()`/`$d()`, replace with plain strings
- `app-ui/src/components/Settings.vue` — remove `$t()`, remove dead computed props
- `app-ui/src/components/Navigation.vue` — remove `$t()`
- `app-ui/src/classes/constants.js` — remove Locales, RtlLocales, DateTimeFormat, Colors
- `app-ui/src/classes/settings.js` — remove locale from defaults
- `app-ui/src/classes/manifest-builder.js` — update `vuetify/lib/util/colors` import path
- `app-ui/src/styles/main.scss` — update `@use 'vuetify'` to `@use 'vuetify/settings'`

**Create:**
- `app-ui/eslint.config.js` — flat config replacement for `.eslintrc.json`

**Delete:**
- `app-ui/src/locales/*.json` — all 19 locale files
- `app-ui/.env` — i18n locale env vars
- `app-ui/.eslintrc.json` — replaced by flat config
- `app-ui/missing-translations.js` — no longer needed

---

### Task 1: Drop nodemon, bump sass

**Files:**
- Modify: `app-ui/package.json`

- [ ] **Step 1: Update package.json**

Remove `nodemon` from devDependencies and its `nodemonConfig` section. Bump `sass` from `1.69.5` to `1.99.0`. Remove the `missing-translations` script.

```json
{
  "devDependencies": {
    "eslint": "8.53.0",
    "eslint-plugin-vue": "9.18.1",
    "sass": "1.99.0",
    "vite": "4.5.0"
  }
}
```

Remove the `nodemonConfig` block and the `"missing-translations"` script entry.

- [ ] **Step 2: Install and verify build**

```bash
cd app-ui && npm install && npm run build
```

Expected: Build succeeds.

- [ ] **Step 3: Commit**

```bash
git add app-ui/package.json app-ui/package-lock.json
git commit -m "Drop nodemon, bump sass to 1.99"
```

---

### Task 2: Remove i18n

This is the largest code change. Remove `vue-i18n`, `@intlify/unplugin-vue-i18n`, all locale files, and replace every `$t()`/`$d()`/`te()` call with plain English strings.

**Files:**
- Modify: `app-ui/package.json`, `app-ui/vite.config.js`, `app-ui/src/main.js`, `app-ui/src/App.vue`, `app-ui/src/components/Scan.vue`, `app-ui/src/components/Files.vue`, `app-ui/src/components/Settings.vue`, `app-ui/src/components/Navigation.vue`, `app-ui/src/classes/constants.js`, `app-ui/src/classes/settings.js`
- Delete: `app-ui/src/locales/*.json` (19 files), `app-ui/.env`, `app-ui/missing-translations.js` (if it exists)

- [ ] **Step 1: Remove i18n packages from package.json**

Remove `vue-i18n` and `@intlify/unplugin-vue-i18n` from dependencies. Remove the `missing-translations` script if not already removed.

- [ ] **Step 2: Update vite.config.js**

Remove the `VueI18nPlugin` import and its entry in the `plugins` array. Remove the `path` import if it's only used for i18n. The result:

```js
import { defineConfig } from "vite";
import vue from '@vitejs/plugin-vue';
import vuetify from 'vite-plugin-vuetify';
import { fileURLToPath } from 'url';
import packageJson from './package.json';

export default defineConfig({
  base: '',
  server: {
    proxy: {
      '/api': 'http://localhost:8080',
      '/api-docs': 'http://localhost:8080',
    }
  },
  plugins: [
    vue(),
    vuetify(),
  ],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  css: {
    preprocessorOptions: {
      scss: {
        quietDeps: true
      },
    }
  },
  define: {
    __PACKAGE_VERSION__: JSON.stringify(packageJson.version)
  },
});
```

Note: Replace `path.resolve(__dirname, "./src")` with `fileURLToPath(new URL('./src', import.meta.url))` since Vite 8 is ESM-only and `__dirname` is not available.

- [ ] **Step 3: Update main.js**

Remove all i18n imports, configuration, and Vuetify locale adapter. Remove `i18n` from `createApp().use()` chain:

```js
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
```

- [ ] **Step 4: Update App.vue**

Remove `$i18n`, `$vuetify.rtl`, locale handling from `beforeMount()` and `mounted()`. Remove the `Constants` import if only used for locale/RTL:

In `setup()`: no changes needed (theme logic stays).

Remove the entire `beforeMount()` hook (it only sets locale).

In `mounted()`: remove both lines (`this.$vuetify.rtl = ...` and `this.$i18n.locale = ...`). Keep the default route logic:

```js
mounted() {
  if (this.$route.matched.length === 0) {
    this.$router.replace('/scan');
  }
},
```

- [ ] **Step 5: Update Navigation.vue**

Replace `$t()` calls with plain strings:

```html
<v-toolbar-title class="unselectable">Epson Perfection V600 Photo</v-toolbar-title>
<v-tabs>
  <v-tab elevation="0" @click="go('/scan')"><v-icon class="mr-2" :icon="mdiCamera" />Scan</v-tab>
  <v-tab elevation="0" @click="go('/files')"><v-icon class="mr-2" :icon="mdiFileDocumentMultiple" />Files</v-tab>
  <v-tab elevation="0" @click="go('/settings')"><v-icon class="mr-2" :icon="mdiCog" />Settings</v-tab>
</v-tabs>
```

Remove `Constants` import (only used for `Version` which is stored in `data()` but never rendered).
Remove `version` from `data()` if unused in template.

- [ ] **Step 6: Update Scan.vue**

Major changes:

1. Remove `useI18n` import and its usage in `setup()` (remove `te`).
2. Remove the `sanitiseLocaleKey` function.
3. Remove `console.log(this.device)` from `formats()` computed.
4. Replace all `$t()` in template with plain strings:

| Template `$t()` | Replacement |
|---|---|
| `$t('global.no-data-text')` | `"No data available"` |
| `$t('scan.source')` | `"Source"` |
| `$t('scan.color-correction')` | `"Color Correction"` |
| `$t('scan.color-space')` | `"Color Space"` |
| `$t('scan.gamma-correction')` | `"Gamma Correction"` |
| `$t('scan.depth')` | `"Depth"` |
| `$t('scan.resolution')` | `"Resolution"` |
| `$t('scan.mode')` | `"Mode"` |
| `$t('scan.format')` | `"Format"` |
| `$t('scan.top')` | `"Top"` |
| `$t('scan.left')` | `"Left"` |
| `$t('scan.width')` | `"Width"` |
| `$t('scan.height')` | `"Height"` |
| `$t('scan.btn-scan')` | `"Scan"` |
| `$t('scan.btn-preview')` | `"Preview"` |
| `$t('scan.btn-clear')` | `"Clear"` |
| `$t('scan.message:deleted-preview')` | `"Deleted preview"` |

5. Simplify computed properties — remove i18n translation attempts, just use raw values:

```js
modes() {
  return '--mode' in this.device.features
    ? this.device.features['--mode'].options.map(mode => ({
        text: mode, value: mode
      }))
    : undefined;
},

colorCorrections() {
  return '--color-correction' in this.device.features
    ? this.device.features['--color-correction'].options.map(cc => ({
        text: cc, value: cc
      }))
    : undefined;
},

formats() {
  return this.device.features['--format'].options.map(format => ({
    text: format, value: format
  }));
},

sources() {
  return '--source' in this.device.features
    ? this.device.features['--source'].options.map(source => ({
        text: source, value: source
      }))
    : undefined;
},
```

6. Fix paper size names in the `context` object. Replace i18n references with plain English:

| Old name | New name |
|---|---|
| `"A4 (@:paper-size.portrait)"` | `"A4 (Portrait)"` |
| `"A5 (@:paper-size.landscape)"` | `"A5 (Landscape)"` |
| `"@:paper-size.letter (@:paper-size.portrait)"` | `"Letter (Portrait)"` |
| `"@:paper-size.legal (@:paper-size.portrait)"` | `"Legal (Portrait)"` |
| `"@:paper-size.tabloid (@:paper-size.portrait)"` | `"Tabloid (Portrait)"` |
| `"@:paper-size.ledger (@:paper-size.portrait)"` | `"Ledger (Portrait)"` |
| `"@:paper-size.junior-legal (@:paper-size.portrait)"` | `"Junior Legal (Portrait)"` |
| `"@:paper-size.half-letter (@:paper-size.portrait)"` | `"Half Letter (Portrait)"` |
| (all similar patterns) | (replace `@:paper-size.X` with the English value) |

7. Simplify `paperSizes` computed to remove the i18n variable replacement logic:

```js
paperSizes() {
  if (!this.geometry) {
    return undefined;
  }
  const deviceSize = {
    x: this.device.features['-x'].limits[1],
    y: this.device.features['-y'].limits[1]
  };
  return [
    { name: 'Maximum', dimensions: { x: deviceSize.x, y: deviceSize.y } },
    ...context.paperSizes.filter(
      paper => paper.dimensions.x <= deviceSize.x && paper.dimensions.y <= deviceSize.y
    )
  ];
},
```

8. In `data()`, remove `device.name = this.$t('global.no-data-text')` — `Device.default()` already sets `name: 'No data available'`.

- [ ] **Step 7: Update Files.vue**

Replace all `$t()` and `$d()` calls:

Template replacements:
| Old | New |
|---|---|
| `$t('files.button:delete-selected')` | `"Delete Selected"` |
| `$t('files.button:action-selected')` | `"Run action..."` |
| `$t('files.dialog:rename')` | `"Change file name"` |
| `$t('files.dialog:rename-cancel')` | `"Cancel"` |
| `$t('files.dialog:rename-save')` | `"Save"` |
| `$d(new Date(item.lastModified), 'long')` | `new Date(item.lastModified).toLocaleString()` |

Script replacements:
| Old | New |
|---|---|
| `this.$t('files.filename')` | `'Filename'` |
| `this.$t('files.date')` | `'Date'` |
| `this.$t('files.size')` | `'Size'` |
| `this.$t('files.actions')` | `'Actions'` |
| `` `${this.$t('files.message:deleted', [data.name])}` `` | `` `Deleted ${data.name}` `` |
| `` `${this.$t('files.message:renamed')}` `` | `'File renamed'` |
| `` `${this.$t('files.message:action', [actionName, filename])}` `` | `` `Ran ${actionName} on ${filename}` `` |

Since `headers` is a computed that uses `this.$t()`, convert it to a plain data property or simplify:

```js
computed: {
  headers() {
    const headers = [
      { align: 'start', sortable: false, value: 'thumb', key: 'thumb' },
      { title: 'Filename', align: 'start', sortable: true, key: 'name' },
      { title: 'Date', align: 'start', sortable: true, key: 'lastModified' },
      { title: 'Size', align: 'start', sortable: true, key: 'sizeString' },
      { title: 'Actions', value: 'actions', sortable: false, key: 'actions' },
    ];
    if (this.smAndDown) {
      headers.splice(2, 2);
    }
    return headers;
  }
},
```

- [ ] **Step 8: Update Settings.vue**

Replace `$t()` calls in template:

```html
<template #description>
  Theme
  <br>
  If you use system theme and change it, you will need to reload the app.
</template>
```

```html
<template #description>
  Show files after scan completes
</template>
```

```html
<div class="text-body-1 mb-4">
  OpenAPI documentation:
  <a target="_blank" href="api-docs">/api-docs</a>
</div>
```

Simplify `themes` computed — remove i18n, use plain strings:

```js
themes() {
  return Object.keys(Constants.Themes).map(t => ({
    text: Constants.Themes[t].charAt(0).toUpperCase() + Constants.Themes[t].slice(1),
    value: Constants.Themes[t]
  }));
}
```

Remove dead `colors()` and `locales()` computed properties.
Remove unused imports: `Common`, `mdiDelete`, `mdiRefresh`.

- [ ] **Step 9: Update constants.js**

Remove `Locales`, `RtlLocales`, `DateTimeFormat`, and `Colors`. Keep `Version`, `Keys`, `Themes`:

```js
const Constants = {
  Version: __PACKAGE_VERSION__,

  Keys: {
    enter: 13,
    escape: 27
  },

  Themes: {
    Dark: 'dark',
    Light: 'light',
    System: 'system'
  }
};

export default Constants;
```

- [ ] **Step 10: Update settings.js**

Remove `locale` and `appColor` from default settings:

```js
export default class Settings {
  static create(obj) {
    obj = Object.assign(Settings.default(), obj);
    return obj;
  }

  static default() {
    return {
      version: Constants.Version,
      theme: 'system',
      showFilesAfterScan: true,
      thumbnails: {
        show: true,
        size: 64
      }
    };
  }
}
```

- [ ] **Step 11: Update App.vue — remove appColor**

Since `appColor` was removed from settings, update `App.vue`:

In `data()`, change `appColor: storage.settings.appColor` to a static value or remove it. The Navigation component uses `appColor` prop. Since it was the Vuetify color name for the app bar, hardcode it or remove the prop:

```js
data() {
  return {
    maskRef: 0,
  };
},
```

Update the template: `<navigation />` (remove `:app-color="appColor"`).

Update `Navigation.vue`: remove the `appColor` prop, use a fixed color or Vuetify default on `v-app-bar`.

- [ ] **Step 12: Simplify manifest-builder.js**

Since `appColor` is no longer configurable, simplify the manifest builder. Remove the color switching logic and the `vuetify/lib/util/colors` import:

```js
export default class ManifestBuilder {
  constructor() {}

  static create() {
    return new ManifestBuilder();
  }

  withDark(dark) {
    this.dark = dark;
    return this;
  }

  build() {
    return {
      theme_color: this.dark ? '#272727' : '#F5F5F5',
      background_color: this.dark ? '#000000' : '#FFFFFF',
      display: 'standalone',
      scope: '/',
      start_url: '/#/scan',
      name: 'Epson Perfection V600 Photo',
      short_name: 'Epson V600',
      icons: [
        { src: './icons/android-chrome-192x192.png', sizes: '192x192', type: 'image/png', purpose: 'any' },
        { src: './icons/android-chrome-512x512.png', sizes: '512x512', type: 'image/png', purpose: 'any' },
        { src: './icons/android-chrome-maskable-192x192.png', sizes: '192x192', type: 'image/png', purpose: 'maskable' },
        { src: './icons/android-chrome-maskable-512x512.png', sizes: '512x512', type: 'image/png', purpose: 'maskable' }
      ]
    };
  }
}
```

Update `App.vue` setup to not pass `storage` to manifest builder:
```js
const manifest = ManifestBuilder.create()
  .withDark(theme === Constants.Themes.Dark)
  .build();
```

- [ ] **Step 13: Delete locale files, .env, missing-translations.js**

```bash
rm -rf app-ui/src/locales/
rm -f app-ui/.env
rm -f app-ui/missing-translations.js
```

- [ ] **Step 14: Install and verify build**

```bash
cd app-ui && npm install && npm run build
```

Expected: Build succeeds with no i18n-related errors.

- [ ] **Step 15: Commit**

```bash
git add -A app-ui/
git commit -m "Remove i18n: replace translations with plain English strings"
```

---

### Task 3: Vue ecosystem upgrade

**Files:**
- Modify: `app-ui/package.json`

- [ ] **Step 1: Update Vue and vue-router versions**

In `package.json`, update:
- `"vue": "3.5.31"`
- `"vue-router": "5.0.4"`

vue-router 5 has zero breaking changes for vue-router 4 users (no code changes needed).

- [ ] **Step 2: Install and verify build**

```bash
cd app-ui && npm install && npm run build
```

Expected: Build succeeds.

- [ ] **Step 3: Commit**

```bash
git add app-ui/package.json app-ui/package-lock.json
git commit -m "Upgrade Vue to 3.5.31, vue-router to 5.0.4"
```

---

### Task 4: Vite ecosystem upgrade

**Files:**
- Modify: `app-ui/package.json`, `app-ui/vite.config.js`

- [ ] **Step 1: Update Vite and plugin versions**

In `package.json`, update:
- `"vite": "8.0.3"` (devDependencies)
- `"@vitejs/plugin-vue": "6.0.5"` (dependencies)

- [ ] **Step 2: Update vite.config.js for Vite 8**

Vite 8 is ESM-only. Replace `path`/`__dirname` usage with `import.meta.url` (should already be done in Task 2's vite.config.js update). If the SCSS `quietDeps` option causes issues with the modern Sass API, update to use `silenceDeprecations`:

```js
css: {
  preprocessorOptions: {
    scss: {
      silenceDeprecations: ['import'],
    },
  }
}
```

If `quietDeps` still works, keep it. Only change if the build fails.

- [ ] **Step 3: Install and verify build**

```bash
cd app-ui && npm install && npm run build
```

Expected: Build succeeds. If there are warnings about `quietDeps`, apply the SCSS fix from step 2.

- [ ] **Step 4: Commit**

```bash
git add app-ui/package.json app-ui/package-lock.json app-ui/vite.config.js
git commit -m "Upgrade Vite to 8.0.3, @vitejs/plugin-vue to 6.0.5"
```

---

### Task 5: Vuetify 3 to 4

**Files:**
- Modify: `app-ui/package.json`, `app-ui/src/styles/main.scss`, `app-ui/src/main.js`

- [ ] **Step 1: Update Vuetify and plugin versions**

In `package.json`, update:
- `"vuetify": "4.0.5"` (dependencies)
- `"vite-plugin-vuetify": "2.1.3"` (dependencies)

- [ ] **Step 2: Update main.scss for Vuetify 4**

Change `@use 'vuetify'` to `@use 'vuetify/settings'`:

```scss
$font: 'Segoe UI', 'Roboto', 'Lucida Grande', 'San Francisco', 'Lucidabright', 'Helvetica';

@use 'vuetify/settings' with (
  $body-font-family: $font,
);
```

- [ ] **Step 3: Review Vuetify 4 breaking changes in our code**

Key Vuetify 4 changes that affect this codebase:

1. **Default theme is now "system"** — our code already handles system theme, so this is fine.

2. **CSS Reset removed** — `h1-h6, p` no longer have `margin: 0`. Our app doesn't rely on this.

3. **Breakpoints changed** — md: 960→840, lg: 1280→1145. The Scan.vue `_resizePreview` method hardcodes `mdBreakpoint = 960`. Update to match Vuetify 4 default (840) or keep 960 and set custom breakpoints. Keep 960 by adding to `createVuetify`:

```js
display: {
  thresholds: {
    md: 960,
    lg: 1280,
    xl: 1920,
    xxl: 2560,
  },
},
```

4. **Elevation reduced to 0-5** — our code uses `elevation="0"` and `elevation-2` which are fine.

5. **VBtn text-transform removed** — buttons no longer uppercase by default. This is probably fine for our UI.

6. **Typography classes renamed** — we use `text-h5` (in Files.vue dialog title) and `text-body-1` (in Settings.vue). These map to MD3 classes. Check if they still work or need updating.

7. **VForm slot variables no longer refs** — not applicable (we don't use VForm slots).

- [ ] **Step 4: Install and verify build**

```bash
cd app-ui && npm install && npm run build
```

Fix any build errors. Common issues:
- SCSS compilation errors from changed Vuetify SASS API
- Import path changes

- [ ] **Step 5: Commit**

```bash
git add app-ui/package.json app-ui/package-lock.json app-ui/src/styles/main.scss app-ui/src/main.js
git commit -m "Upgrade Vuetify to 4.0.5, vite-plugin-vuetify to 2.1.3"
```

---

### Task 6: ESLint flat config migration

**Files:**
- Create: `app-ui/eslint.config.js`
- Delete: `app-ui/.eslintrc.json`
- Modify: `app-ui/package.json`

- [ ] **Step 1: Update ESLint versions**

In `package.json`, update:
- `"eslint": "10.1.0"` (devDependencies)
- `"eslint-plugin-vue": "10.8.0"` (devDependencies)

- [ ] **Step 2: Create eslint.config.js**

Replace `.eslintrc.json` with flat config:

```js
import js from '@eslint/js';
import pluginVue from 'eslint-plugin-vue';

export default [
  js.configs.recommended,
  ...pluginVue.configs['flat/recommended'],
  {
    languageOptions: {
      ecmaVersion: 2022,
      sourceType: 'module',
      globals: {
        __PACKAGE_VERSION__: 'readonly',
        window: 'readonly',
        document: 'readonly',
        localStorage: 'readonly',
        navigator: 'readonly',
        location: 'readonly',
        fetch: 'readonly',
        URLSearchParams: 'readonly',
        setTimeout: 'readonly',
        clearTimeout: 'readonly',
      }
    },
    rules: {
      'array-bracket-spacing': 'warn',
      'brace-style': 'warn',
      'comma-spacing': 'warn',
      'eol-last': 'warn',
      'eqeqeq': 'warn',
      'indent': ['error', 2, { 'SwitchCase': 1 }],
      'keyword-spacing': 'warn',
      'no-mixed-spaces-and-tabs': 'warn',
      'no-undef': 'error',
      'no-unused-vars': 'warn',
      'no-var': 'warn',
      'object-shorthand': ['warn', 'methods'],
      'prefer-arrow-callback': 'warn',
      'quotes': ['error', 'single'],
      'semi': ['error', 'always'],
      'space-before-blocks': 'warn',
      'space-infix-ops': 'warn',
      'vue/first-attribute-linebreak': 'off',
      'vue/html-closing-bracket-newline': 'off',
      'vue/html-indent': 'off',
      'vue/max-attributes-per-line': 'off',
      'vue/multi-word-component-names': 'off',
      'vue/singleline-html-element-content-newline': 'off',
    }
  }
];
```

Note: ESLint 10 requires `@eslint/js` as a dependency. Add it:
- `"@eslint/js": "10.1.0"` (devDependencies)

- [ ] **Step 3: Update lint script in package.json**

```json
"lint": "eslint src"
```

(ESLint 10 auto-detects `.vue` and `.js` files; no need for `--ext`.)

- [ ] **Step 4: Delete .eslintrc.json**

```bash
rm app-ui/.eslintrc.json
```

- [ ] **Step 5: Install, lint, and fix**

```bash
cd app-ui && npm install && npm run lint
```

Fix any lint errors. Common issues:
- Global variable declarations may need updating
- Plugin compatibility

- [ ] **Step 6: Verify build still works**

```bash
cd app-ui && npm run build
```

- [ ] **Step 7: Commit**

```bash
git add app-ui/eslint.config.js app-ui/package.json app-ui/package-lock.json
git rm app-ui/.eslintrc.json
git commit -m "Migrate ESLint to v10 with flat config"
```

---

### Task 7: Verify and cleanup

- [ ] **Step 1: Full build verification**

```bash
cd app-ui && npm run build
```

Expected: Clean build with no errors or warnings.

- [ ] **Step 2: Run lint**

```bash
cd app-ui && npm run lint
```

Fix any remaining lint issues.

- [ ] **Step 3: Remove empty `<style>` blocks**

Settings.vue has an empty `<style>` block — remove it.

- [ ] **Step 4: Remove the empty div wrapper in Scan.vue template**

Lines 70-71 have an empty `<div class="d-flex flex-row flex-wrap">` — remove it.

- [ ] **Step 5: Remove the TODO comments**

- Scan.vue line 104: `// TODO: remove` — the context object is needed (it's the device data), but remove the TODO comment.
- Scan.vue line 706: `// TODO: enable the timer...` — remove the commented-out timer code.
- Files.vue line 180: `// TODO remove` — the `actionList()` method just sets `this.actions = []`. Remove the TODO.

- [ ] **Step 6: Verify dev server works**

```bash
cd app-ui && npm run dev
```

Open in browser, verify scan page loads, settings page works, files page works.

- [ ] **Step 7: Final commit**

```bash
git add -A app-ui/
git commit -m "Post-upgrade cleanup: remove dead code and empty blocks"
```
