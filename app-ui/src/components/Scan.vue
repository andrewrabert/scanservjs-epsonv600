<template>
  <div>
    <v-row>
      <v-spacer />

      <v-col cols="12" md="3" class="mb-10 mb-md-0">
        <v-select
          v-model="request.params.source"
          no-data-text="No data available" label="Source"
          :items="sources" item-value="value" item-title="text" />

        <v-select
          v-model="request.params.colorCorrection"
          no-data-text="No data available" label="Color Correction"
          :items="colorCorrections" item-value="value" item-title="text" />

        <v-select
          v-model="request.params.colorSpace"
          no-data-text="No data available" label="Color Space"
          :items="device.features['--color-space']['options']" />

        <v-select
          v-model="request.params.gammaCorrection"
          no-data-text="No data available" label="Gamma Correction"
          :items="device.features['--gamma-correction']['options']" />

        <v-select
          v-model="request.params.depth"
          no-data-text="No data available" label="Depth"
          :items="device.features['--depth']['options']" />

        <v-select
          v-model="request.params.resolution"
          no-data-text="No data available" label="Resolution"
          :items="device.features['--resolution']['options']" />

        <v-select
          v-model="request.params.mode"
          no-data-text="No data available" label="Mode"
          :items="modes" item-value="value" item-title="text" />

        <v-select
          v-model="request.params.format"
          no-data-text="No data available" label="Format"
          :items="formats" item-value="value" item-title="text" />

          <div>
          <v-label class="text-caption">Scan Area (mm)</v-label>
          <div class="d-flex flex-row flex-wrap">
            <v-text-field v-model="request.params.top" label="Top" type="number" step="any" @blur="onCoordinatesChange" />
            <v-text-field v-model="request.params.left" label="Left" type="number" step="any" @blur="onCoordinatesChange" />
            <v-text-field v-model="request.params.width" label="Width" type="number" step="any" @blur="onCoordinatesChange" />
            <v-text-field v-model="request.params.height" label="Height" type="number" step="any" @blur="onCoordinatesChange" />
            <v-menu class="height-screen" offset-y>
              <template #activator="{ props }">
                  <v-btn color="grey" class="height-25" v-bind="props"><v-icon :icon="mdiNewspaper" /></v-btn>
              </template>
              <v-list dense>
                <v-list-item
                  v-for="(item, index) in paperSizes"
                  :key="index"
                  @click="updatePaperSize(item)">
                  <v-list-item-title>{{ item.name }}</v-list-item-title>
                </v-list-item>
              </v-list>
            </v-menu>
          </div>
          </div>

        <div class="d-flex flex-row-reverse flex-wrap">
          <v-btn color="blue" class="ml-1 mb-1" @click="scan(1)">Scan <v-icon class="ml-2" :icon="mdiCamera" /></v-btn>
          <v-btn color="green" class="ml-1 mb-1" @click="createPreview">Preview <v-icon class="ml-2" :icon="mdiMagnify" /></v-btn>
          <v-btn color="amber" class="ml-1 mb-1" @click="deletePreview">Clear <v-icon class="ml-2" :icon="mdiDelete" /></v-btn>
        </div>
      </v-col>

      <v-col cols="12" md="auto" class="mb-10 mb-md-0" :style="{width: `${preview.width}px`}">
        <cropper ref="cropper" :key="preview.key" class="cropper border-lg" :transition-time="10" :wheel-resize="false"
            :default-position="cropperDefaultPosition" :default-size="cropperDefaultSize"
            :src="img" @change="onCropperChange" />
      </v-col>
      <v-spacer />
    </v-row>
  </div>
</template>

<script>
import { mdiCamera, mdiDelete, mdiNewspaper, mdiMagnify, mdiRefresh } from '@mdi/js';
import { Cropper } from 'vue-advanced-cropper';

import Common from '../classes/common';
import Device from '../classes/device';
import Request from '../classes/request';
import Storage from '../classes/storage';

import 'vue-advanced-cropper/dist/style.css';

const storage = Storage.instance();

const context = {
  'devices': [
    {
      'id': 'EpsonPerfectionV600',
      'name': 'EpsonPerfectionV600',
      'features': {
        '--mode': {
          'text': '--mode Binary|Gray|Color [Color]',
          'name': '--mode',
          'default': 'Color',
          'parameters': 'Binary|Gray|Color',
          'enabled': true,
          'options': [
            'Binary',
            'Gray',
            'Color'
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
        '--gamma-correction': {
          'text': '--gamma-correction User defined (Gamma=1.0)|User defined (Gamma=1.8) [User defined (Gamma=1.8)]',
          'name': '--gamma-correction',
          'default': '2.2',
          'parameters': 'User defined (Gamma=1.0)|User defined (Gamma=1.8)',
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
        '--resolution': {
          'text': '--resolution 200|400|800|1600|3200|6400dpi [400]',
          'name': '--resolution',
          'default': 400,
          'parameters': '200|400|800|1600|3200|6400dpi',
          'enabled': true,
          'options': [
            200,
            400,
            800,
            1600,
            3200,
            6400
          ]
        },
        '--x-resolution': {
          'text': '--x-resolution 200|400|600|800|1200|1600|3200|6400dpi [200] [advanced]',
          'name': '--x-resolution',
          'default': '200',
          'parameters': '200|400|600|800|1200|1600|3200|6400dpi',
          'meta': 'advanced',
          'enabled': true
        },
        '--y-resolution': {
          'text': '--y-resolution 200|240|320|400|600|800|1200|1600|3200|4800|6400dpi [320] [advanced]',
          'name': '--y-resolution',
          'default': '320',
          'parameters': '200|240|320|400|600|800|1200|1600|3200|4800|6400dpi',
          'meta': 'advanced',
          'enabled': true
        },
        '--speed': {
          'text': '--speed[=(yes|no)] [no]',
          'name': '--speed',
          'default': 'no',
          'parameters': '[=(yes|no)]',
          'enabled': true
        },
        '--short-resolution': {
          'text': '--short-resolution[=(yes|no)] [no]',
          'name': '--short-resolution',
          'default': 'no',
          'parameters': '[=(yes|no)]',
          'enabled': true
        },
        '--wait-for-button': {
          'text': '--wait-for-button[=(yes|no)] [no] [advanced]',
          'name': '--wait-for-button',
          'default': 'no',
          'parameters': '[=(yes|no)]',
          'meta': 'advanced',
          'enabled': true
        },
        '--preview': {
          'text': '--preview[=(yes|no)] [no]',
          'name': '--preview',
          'default': 'no',
          'parameters': '[=(yes|no)]',
          'enabled': true
        },
        '--preview-speed': {
          'text': '--preview-speed[=(yes|no)] [no]',
          'name': '--preview-speed',
          'default': 'no',
          'parameters': '[=(yes|no)]',
          'enabled': true
        },
        '--scan-area': {
          'text': '--scan-area Maximum|A4|A5 Landscape|A5 Portrait|B5|Letter|Executive|CD [Maximum]',
          'name': '--scan-area',
          'default': 'Maximum',
          'parameters': 'Maximum|A4|A5 Landscape|A5 Portrait|B5|Letter|Executive|CD',
          'enabled': true
        },
        '-l': {
          'text': '-l 0..215.9mm [0]',
          'name': '-l',
          'default': 0,
          'parameters': '0..215.9mm',
          'enabled': true,
          'limits': [
            0,
            215.9
          ],
          'interval': 1
        },
        '-t': {
          'text': '-t 0..297.18mm [0]',
          'name': '-t',
          'default': 0,
          'parameters': '0..297.18mm',
          'enabled': true,
          'limits': [
            0,
            297.1
          ],
          'interval': 1
        },
        '-x': {
          'text': '-x 0..215.9mm [215.9]',
          'name': '-x',
          'default': 215.9,
          'parameters': '0..215.9mm',
          'enabled': true,
          'limits': [
            0,
            215.9
          ],
          'interval': 1
        },
        '-y': {
          'text': '-y 0..297.18mm [297.18]',
          'name': '-y',
          'default': 297.1,
          'parameters': '0..297.18mm',
          'enabled': true,
          'limits': [
            0,
            297.1
          ],
          'interval': 1
        },
        '--quick-format': {
          'text': '--quick-format Maximum|A4|A5 Landscape|A5 Portrait|B5|Letter|Executive|CD [Maximum]',
          'name': '--quick-format',
          'default': 'Maximum',
          'parameters': 'Maximum|A4|A5 Landscape|A5 Portrait|B5|Letter|Executive|CD',
          'enabled': true
        },
        '--source': {
          'text': '--source Flatbed|Transparency Unit [Flatbed]',
          'name': '--source',
          'default': 'Flatbed',
          'parameters': 'Flatbed|Transparency Unit',
          'enabled': true,
          'options': [
            'Flatbed',
            'Transparency Unit'
          ]
        },
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
        '--color-correction': {
          'text': '--color-correction[=(yes|no)] [yes]',
          'name': '--color-correction',
          'default': 'yes',
          'parameters': '[=(yes|no)]',
          'enabled': true,
          'options': [
            'yes',
            'no'
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
        }
      },
      'string': '\nAll options specific to device `EpsonPerfectionV600\':\n    --mode Binary|Gray|Color [Color]\n        Selects the scan mode (e.g., lineart, monochrome, or color).\n    --depth 8|16 [8]\n        Number of bits per sample, typical values are 1 for "line-art" and 8\n        for multibit scans.\n    --halftoning None|Halftone A (Hard Tone)|Halftone B (Soft Tone)|Halftone C (Net Screen) [inactive]\n        Selects the halftone.\n    --dropout None|Red|Green|Blue [inactive]\n        Selects the dropout.\n    --sharpness -2..2 [inactive]\n        \n    --gamma-correction User defined (Gamma=1.0)|User defined (Gamma=1.8) [User defined (Gamma=1.8)]\n        Selects the gamma correction value from a list of pre-defined devices\n        or the user defined table, which can be downloaded to the scanner\n    --color-correction User defined [inactive]\n        Sets the color correction table for the selected output device.\n    --resolution 200|400|800|1600|3200|6400dpi [400]\n        Sets the resolution of the scanned image.\n    --x-resolution 200|400|600|800|1200|1600|3200|6400dpi [200] [advanced]\n        Sets the horizontal resolution of the scanned image.\n    --y-resolution 200|240|320|400|600|800|1200|1600|3200|4800|6400dpi [320] [advanced]\n        Sets the vertical resolution of the scanned image.\n    --threshold 0..255 [inactive]\n        Select minimum-brightness to get a white point\n    --mirror[=(yes|no)] [inactive]\n        Mirror the image.\n    --speed[=(yes|no)] [no]\n        Determines the speed at which the scan proceeds.\n    --auto-area-segmentation[=(yes|no)] [inactive]\n        \n    --short-resolution[=(yes|no)] [no]\n        Display short resolution list\n    --zoom 50..200 [inactive]\n        Defines the zoom factor the scanner will use\n    --red-gamma-table 0..255,...\n        Gamma-correction table for the red band.\n    --green-gamma-table 0..255,...\n        Gamma-correction table for the green band.\n    --blue-gamma-table 0..255,...\n        Gamma-correction table for the blue band.\n    --wait-for-button[=(yes|no)] [no] [advanced]\n        After sending the scan command, wait until the button on the scanner\n        is pressed to actually start the scan process.\n    --monitor-button[=(yes|no)] [no] [read-only]\n        Indicates whether a button on the scanner has been pressed.\n    --polling-time <int> [1000000] [read-only]\n        Time between queries when waiting for device state changes.\n    --needs-polling[=(yes|no)] [no] [read-only]\n        Indicates whether the scanner needs to poll.\n    --preview[=(yes|no)] [no]\n        Request a preview-quality scan.\n    --preview-speed[=(yes|no)] [no]\n        \n    --scan-area Maximum|A4|A5 Landscape|A5 Portrait|B5|Letter|Executive|CD [Maximum]\n        Select an area to scan based on well-known media sizes.\n    -l 0..215.9mm [0]\n        Top-left x position of scan area.\n    -t 0..297.18mm [0]\n        Top-left y position of scan area.\n    -x 0..215.9mm [215.9]\n        Width of scan-area.\n    -y 0..297.18mm [297.18]\n        Height of scan-area.\n    --quick-format Maximum|A4|A5 Landscape|A5 Portrait|B5|Letter|Executive|CD [Maximum]\n        Select an area to scan based on well-known media sizes. (DEPRECATED)\n     --format jpeg|jpeg-xl|png|ppm|tiff [tiff]\n        Selects the output format\n    --auto-eject[=(yes|no)] [inactive]\n        Eject document after scanning\n    --film-type Positive Film|Negative Film [inactive]\n        \n    --focus-position Focus on glass|Focus 2.5mm above glass [inactive]\n        Sets the focus position to either the glass or 2.5mm above the glass\n    --bay  1 | 2 | 3 | 4 | 5 | 6  [inactive]\n        Select bay to scan\n    --eject [inactive]\n        Eject the sheet in the ADF\n    --adf-mode Simplex|Duplex [inactive]\n        Selects the ADF mode (simplex/duplex)\n    --detect-doc-size[=(yes|no)] [inactive]\n        Activates document size auto-detection.  The scan area will be set to\n        match the detected document size.\n    --scan-area-is-valid[=(yes|no)] [yes] [read-only]\n        Indicates whether the current scan area settings are valid.\n    --adf-auto-scan[=(yes|no)] [inactive]\n        Skips per sheet device setup for faster throughput.\n    --double-feed-detection-sensitivity None|Low|High [inactive]\n        Sets the sensitivity with which multi-sheet page feeds are detected\n        and reported as errors.\n    --ext-sane-status 0..2 [0] [read-only]\n        Ugly kludge to provide additional status message strings to a\n        frontend.\n    --adf-duplex-direction-matches[=(yes|no)] [inactive]\n        Indicates whether the device\'s ADF duplex mode, if available, scans in\n        the same direction for the front and back.\n    --deskew[=(yes|no)] [inactive]\n        Rotate image so it appears upright.\n    --autocrop[=(yes|no)] [inactive]\n        Determines empty margins in the scanned image and removes them.  This\n        normally reduces the image to the size of the original document but may\n        remove more.\n    --calibrate [inactive]\n        Performs color matching to make sure that the document\'s color tones\n        are scanned correctly.\n    --clean [inactive]\n        Cleans the scanners reading section.\n\n',
      'settings': {
        'pipeline': {
          'options': [
            'JPG',
            'PNG',
            'TIF'
          ],
          'default': 'JPG'
        }
      }
    }
  ],
  'version': '3.0.3',
  'paperSizes': [
    {
      'name': 'A3 (Portrait)',
      'dimensions': {
        'x': 297,
        'y': 420
      }
    },
    {
      'name': 'A4 (Portrait)',
      'dimensions': {
        'x': 210,
        'y': 297
      }
    },
    {
      'name': 'A5 (Portrait)',
      'dimensions': {
        'x': 148,
        'y': 210
      }
    },
    {
      'name': 'A5 (Landscape)',
      'dimensions': {
        'x': 210,
        'y': 148
      }
    },
    {
      'name': 'A6 (Portrait)',
      'dimensions': {
        'x': 105,
        'y': 148
      }
    },
    {
      'name': 'A6 (Landscape)',
      'dimensions': {
        'x': 148,
        'y': 105
      }
    },
    {
      'name': 'B3 (Portrait)',
      'dimensions': {
        'x': 353,
        'y': 500
      }
    },
    {
      'name': 'B4 (Portrait)',
      'dimensions': {
        'x': 250,
        'y': 353
      }
    },
    {
      'name': 'B5 (Portrait)',
      'dimensions': {
        'x': 176,
        'y': 250
      }
    },
    {
      'name': 'B5 (Landscape)',
      'dimensions': {
        'x': 250,
        'y': 176
      }
    },
    {
      'name': 'B6 (Portrait)',
      'dimensions': {
        'x': 125,
        'y': 176
      }
    },
    {
      'name': 'B6 (Landscape)',
      'dimensions': {
        'x': 176,
        'y': 125
      }
    },
    {
      'name': 'DIN D3 (Portrait)',
      'dimensions': {
        'x': 272,
        'y': 385
      }
    },
    {
      'name': 'DIN D4 (Portrait)',
      'dimensions': {
        'x': 192,
        'y': 272
      }
    },
    {
      'name': 'DIN D5 (Portrait)',
      'dimensions': {
        'x': 136,
        'y': 192
      }
    },
    {
      'name': 'DIN D5 (Landscape)',
      'dimensions': {
        'x': 192,
        'y': 136
      }
    },
    {
      'name': 'DIN D6 (Portrait)',
      'dimensions': {
        'x': 96,
        'y': 136
      }
    },
    {
      'name': 'DIN D6 (Landscape)',
      'dimensions': {
        'x': 136,
        'y': 96
      }
    },
    {
      'name': 'Letter (Portrait)',
      'dimensions': {
        'x': 216,
        'y': 279
      }
    },
    {
      'name': 'Legal (Portrait)',
      'dimensions': {
        'x': 216,
        'y': 356
      }
    },
    {
      'name': 'Tabloid (Portrait)',
      'dimensions': {
        'x': 279,
        'y': 432
      }
    },
    {
      'name': 'Ledger (Portrait)',
      'dimensions': {
        'x': 432,
        'y': 279
      }
    },
    {
      'name': 'Junior Legal (Portrait)',
      'dimensions': {
        'x': 127,
        'y': 203
      }
    },
    {
      'name': 'Half Letter (Portrait)',
      'dimensions': {
        'x': 140,
        'y': 216
      }
    }
  ],
  'actions': []
};


function round(n, dp) {
  const f = Math.pow(10, dp || 0);
  return Math.round(n * f) / f;
}

export default {

  name: 'Scan',

  components: {
    Cropper,
  },

  emits: ['mask', 'notify'],

  setup() {
    return {
      mdiCamera,
      mdiDelete,
      mdiNewspaper,
      mdiMagnify,
      mdiRefresh,
    };
  },

  data() {
    const device = Device.default();
    const request = Request.create(null, device);

    return {
      device: device,
      img: null,
      request: request,
      preview: {
        timer: 0,
        width: 400,
        key: 0
      }
    };
  },

  computed: {
    geometry() {
      return ['-x', '-y', '-l', '-t'].every(s => s in this.device.features);
    },

    deviceSize() {
      return !this.geometry ? undefined : {
        width: this.device.features['-x'].limits[1],
        height: this.device.features['-y'].limits[1]
      };
    },

    modes() {
      return '--mode' in this.device.features
        ? this.device.features['--mode'].options.map(mode => ({ text: mode, value: mode }))
        : undefined;
    },

    colorCorrections() {
      return '--color-correction' in this.device.features
        ? this.device.features['--color-correction'].options.map(cc => ({ text: cc, value: cc }))
        : undefined;
    },

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

    formats() {
      return this.device.features['--format'].options.map(format => ({ text: format, value: format }));
    },

    sources() {
      return '--source' in this.device.features
        ? this.device.features['--source'].options.map(source => ({ text: source, value: source }))
        : undefined;
    }
  },

  watch: {
    request: {
      handler(request) {
        storage.request = request;
      },
      deep: true
    }
  },

  mounted() {
    this._resizePreview();
    this.device = context.devices[0];
    this.request = this.buildRequest();
    this.readPreview();
    window.addEventListener('resize', () => {
      clearTimeout(this.preview.timer);
      this.preview.timer = setTimeout(this._resizePreview, 100);
    });
  },

  methods: {
    _resizePreview() {
      const paperRatio = this.geometry
        ? this.deviceSize.width / this.deviceSize.height
        : 210 / 297;

      // This only makes a difference when the col-width="auto" - so md+
      const mdBreakpoint = 960;
      if (window.innerWidth >= mdBreakpoint) {
        const appbarHeight = 80;
        const availableWidth = window.innerWidth - 30;
        const availableHeight = window.innerHeight - appbarHeight;
        const desiredWidth = availableHeight * paperRatio;
        this.preview.width = Math.min(availableWidth / 2, desiredWidth);
        this.preview.key += 1;
      }
    },

    _fetch(url, options) {
      this.mask(1);
      return Common.fetch(url, options)
        .then(data => {
          this.mask(-1);
          return data;
        })
        .catch(error => {
          this.notify({ type: 'e', message: error });
          this.mask(-1);
          return error;
        });
    },

    createPreview() {
      this.mask(1);

      let data = Common.clone(this.request);

      this._fetch('api/v1/preview', {
        method: 'POST',
        body: JSON.stringify(data),
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json'
        }
      }).then(() => {
        this.readPreview();
        this.mask(-1);
      }).catch(() => {
        this.mask(-1);
      });
    },

    deletePreview() {
      this.mask(1);
      this.resetPaperSize();
      Common.fetch('api/v1/preview', {
        method: 'DELETE'
      }).then(() => {
        this.notify({ type: 'i', message: 'Deleted preview' });
        this.readPreview();
        this.mask(-1);
      }).catch(error => {
        this.notify({ type: 'e', message: error });
        this.mask(-1);
      });
    },

    pixelsPerMm() {
      const scanner = this.deviceSize;

      // The preview image may not have perfectly scaled dimensions
      // because pixel counts are integers. So we report a horizontal
      // and vertical resolution
      const image = this.$refs.cropper.imageSize;
      return {
        x: image.width / scanner.width,
        y: image.height / scanner.height
      };
    },

    scaleCoordinates(coordinates, xScale, yScale) {
      return {
        width: round(coordinates.width * xScale, 1),
        height: round(coordinates.height * yScale, 1),
        left: round(coordinates.left * xScale, 1),
        top: round(coordinates.top * yScale, 1)
      };
    },

    cropperDefaultPosition() {
      const adjusted = this.scaleCoordinates(
        this.request.params,
        this.pixelsPerMm().x,
        this.pixelsPerMm().y);

      return {
        left: adjusted.left,
        top: adjusted.top
      };
    },

    cropperDefaultSize() {
      const adjusted = this.scaleCoordinates(
        this.request.params,
        this.pixelsPerMm().x,
        this.pixelsPerMm().y);

      return {
        width: adjusted.width,
        height: adjusted.height
      };
    },

    mask(add) {
      this.$emit('mask', add);
    },

    notify(notification) {
      this.$emit('notify', notification);
    },

    onCoordinatesChange() {
      const adjusted = this.scaleCoordinates(
        this.request.params,
        this.pixelsPerMm().x,
        this.pixelsPerMm().y);

      this.$refs.cropper.setCoordinates(adjusted);
    },

    onCropperChange({ coordinates }) {
      const adjusted = this.scaleCoordinates(
        coordinates,
        1 / this.pixelsPerMm().x,
        1 / this.pixelsPerMm().y);

      // The cropper changes even when coordinates are set manually. This will
      // result in manually set values being overwritten because of rounding.
      // If someone is taking the trouble to set values manually then they
      // should be preserved. We should only update the values if they breach
      // a threshold or the scanner dimensions
      const scanner = this.deviceSize;
      const params = this.request.params;
      const threshold = 0.4;
      const boundAndRound = (n, min, max) => round(Math.min(Math.max(min, n), max), 1);
      const bestValue = (current, crop, min, max) => Math.abs(current - crop) < threshold
        ? boundAndRound(current, min, max)
        : boundAndRound(crop, min, max);

      params.width = bestValue(params.width, adjusted.width, 0, scanner.width);
      params.height = bestValue(params.height, adjusted.height, 0, scanner.height);
      params.left = bestValue(params.left, adjusted.left, 0, scanner.width);
      params.top = bestValue(params.top, adjusted.top, 0, scanner.height);
    },

    readPreview() {
      // Gets the preview image as a base64 encoded jpg and updates the UI
      this._fetch('api/v1/preview', {
        cache: 'no-store',
        method: 'GET'
      }).then(data => {
        this.img = 'data:image/jpeg;base64,' + data.content;
        this._resizePreview();
      });
    },

    buildRequest() {
      let request = storage.request;
      if (request && request.params) {
        this.device = context.devices.filter(d => d.id === request.params.deviceId)[0]
          || context.devices[0];
      }

      request = Request.create(request, this.device);
      return request;
    },

    clear() {
      storage.request = null;
      this.request = this.buildRequest();
    },

    scan() {
      const data = Common.clone(this.request);
      this._fetch('api/v1/scan', {
        method: 'POST',
        body: JSON.stringify(data),
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json'
        }
      }).then(() => {
        if (storage.settings.showFilesAfterScan) {
          this.$router.push('/files');
        } else {
          this.readPreview();
        }
      });
    },

    updatePaperSize(value) {
      if (value.dimensions) {
        this.request.params.width = value.dimensions.x;
        this.request.params.height = value.dimensions.y;
        this.onCoordinatesChange();
      }
    },

    resetPaperSize() {
      this.request.params.width = this.device.features['-x'].limits[1];
      this.request.params.height = this.device.features['-y'].limits[1];
      this.onCoordinatesChange();
    }
  }
};
</script>

<style scoped>
#mask {
  position: fixed;
  width: 100%;
  height: 100%;
  background: rgba(0,0,0,.4);
  top: 0;
  left: 0;
  z-index: 10;
}
</style>
