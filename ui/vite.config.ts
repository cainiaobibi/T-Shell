import { defineConfig, loadEnv } from 'vite';
import { getRootPath, getSrcPath, viteDefine, setupVitePlugins, createViteProxy } from './build';
import { getServiceEnvConfig } from './.env-config';

export default defineConfig(configEnv => {
  const viteEnv = loadEnv(configEnv.mode, process.cwd()) as unknown as ImportMetaEnv;

  const rootPath = getRootPath();
  const srcPath = getSrcPath();

  const isOpenProxy = viteEnv.VITE_HTTP_PROXY === 'Y';
  const envConfig = getServiceEnvConfig(viteEnv);

  return {
    base: viteEnv.VITE_BASE_URL,
    resolve: {
      alias: {
        '~': rootPath,
        '@': srcPath
      }
    },
    define: viteDefine,
    plugins: setupVitePlugins(viteEnv),
    css: {
      preprocessorOptions: {
        scss: {
          additionalData: `@use "./src/styles/scss/global.scss" as *;`
        }
      }
    },
    server: {
      host: '0.0.0.0',
      port: 3200,
      open: true,
      proxy: createViteProxy(isOpenProxy, envConfig)
    },
    preview: {
      port: 5050
    },
    optimizeDeps: {
      include: [
        '@antv/data-set',
        '@antv/g2',
        '@better-scroll/core',
        'echarts',
        'swiper',
        'swiper/vue',
        'vditor',
        'wangeditor',
        'xgplayer'
      ]
    },
    build: {
      reportCompressedSize: false,
      sourcemap: false,
      commonjsOptions: {
        ignoreTryCatch: false,
        // 生产环境时移除console.log()
        drop_console: true,
        drop_debugger: true
      }
    }
  };
});
