import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [
    react(),
  ],
  resolve: {
    alias: {
      // מאפשר import "@/components/..." במקום "../../../components/..."
      '@': path.resolve(__dirname, 'src'),
    },
  },
  build: {
    // תיקיית היעד שייווצר ב-root של הפרויקט
    outDir: 'dist',
    // מוחק קודם את כל הקבצים הישנים בתוך dist
    emptyOutDir: true,
    // לא יוצר sourcemaps בשלב CI (אופציונלי)
    sourcemap: false,
    // אם תרצה להתאים עוד הגדרות Rollup:
    // rollupOptions: {
    //   input: path.resolve(__dirname, 'index.html'),
    // },
  },
  // אם אתם מפרסמים ב-GitHub Pages מתחת לתיקיית /Resu-Me/,
  // אפשר להוסיף גם base:
  // base: '/Resu-Me/',
});
