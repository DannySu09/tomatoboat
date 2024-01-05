import { defineConfig } from 'unocss';
import presetWind from '@unocss/preset-wind';
import presetIcons from '@unocss/preset-icons';

export default defineConfig({
  theme: {
    colors: {
      green: {
        100: '#F2EEBF',
        200: '#B6D5B3',
        300: '#9EC0AF',
        400: '#7DA950',
        500: '#41AC4A',
        600: '#5B9035',
        700: '#006A35',
        800: '#42502C'
      },
      blue: {
        100: '#C3E1E0',
        200: '#B4D8E1',
        300: '#8FB5D2',
        400: '#00A5BF',
        500: '#00768E',
        600: '#005291',
        700: '#024D89',
        800: '#032851'
      },
      gray: {
        100: '#FCFBF8',
        200: '#E5E9EB',
        300: '#AAABAF',
        400: '#B1BCC6',
        500: '#6C7D89',
        600: '#4E595B',
        700: '#384C4F',
        800: '#404654'
      },
      pink: {
        100: '#FDF9F8',
        200: '#FCEBF5',
        300: '#F9D3E4',
        400: '#EE9AA4',
        500: '#EE9AA4',
        600: '#E84645',
        700: '#D395B0',
        800: '#B47879'
      }
    }
  },
  presets: [
    presetWind(),
    presetIcons({
      collections: {
        solar: () => import('@iconify-json/solar/icons.json').then(i => i.default as any)
      }
    }),
  ],
});