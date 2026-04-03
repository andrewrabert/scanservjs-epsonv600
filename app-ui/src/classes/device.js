export default {
  default() {
    return {
      id: 'Unspecified',
      name: 'No data available',
      features: {
        '--format': {
          'text': '--format jpeg|jpeg-xl|png|ppm|tiff [tiff]',
          'name': '--format',
          'default': 'tiff',
          'parameters': 'jpeg|jpeg-xl|png|ppm|tiff',
          'enabled': true,
          'options': [
            'jpeg',
            'jpeg-xl',
            'png',
            'ppm',
            'tiff'
          ]
        },
        '--color-space': {
          'text': '--color-space None|A|B [None]',
          'name': '--color-space',
          'default': 'sRGB IEC61966-2.1',
          'parameters': 'None|A|B',
          'enabled': true,
          'options': [
            'None',
            'Adobe RGB (1998)',
            'EPSON sRGB',
            'sRGB IEC61966-2.1'
          ]
        },
        '--gamma-correction': {
          'text': '--gamma-correction 1.0|1.8|2.0|2.2|2.4|2.8 [2.2]',
          'name': '--gamma-correction',
          'default': '2.2',
          'parameters': '1.0|1.8|2.0|2.2|2.4|2.8',
          'enabled': true,
          'options': [
            '1.0',
            '1.8',
            '2.0',
            '2.2',
            '2.4',
            '2.8'
          ]
        },
        '--depth': {
          'text': '--depth 8|16 [8]',
          'name': '--depth',
          'default': '8',
          'parameters': '8|16',
          'enabled': true,
          'options': [
            '8',
            '16'
          ]
        },
        '--mode': {
          default: '',
          options: [],
        },
        '--resolution': {
          default: 0,
          options: [],
        },
        '-l': {
          default: 0,
          limits: [0, 215],
        },
        '-t': {
          default: 297,
          limits: [0, 297],
        },
        '-x': {
          default: 0,
          limits: [0, 215],
        },
        '-y': {
          default: 297,
          limits: [0, 297],
        },
        '--brightness': {
          default: 0,
          limits: [-100, 100],
        },
        '--contrast': {
          default: 0,
          limits: [-100, 100],
        },
        '--disable-dynamic-lineart': {}
      },
      settings: {
        pipeline: {
          options: [],
          default: ''
        }
      }
    };
  }
};
