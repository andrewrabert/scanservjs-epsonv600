export default class Settings {
  static create(obj) {
    obj = Object.assign(Settings.default(), obj);
    return obj;
  }

  static default() {
    return {
      theme: 'system',
      showFilesAfterScan: true,
      thumbnails: {
        show: true,
        size: 64
      }
    };
  }
}
