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
