# Orion App Templates

Reference configurations for all Orion apps. Copy and adapt — don't import directly.

## Cargo Workspace Dependencies

Pin these exact versions across all apps to avoid compatibility issues:

```toml
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
tracing = "0.1"
tokio = { version = "1", features = ["full"] }
surrealdb = { version = "2", features = ["kv-mem", "kv-surrealkv"] }
specta = { version = "=2.0.0-rc.22", features = ["derive", "serde_json"] }
specta-typescript = "0.0"
tauri-specta = { version = "=2.0.0-rc.21", features = ["derive", "typescript"] }
```

Each app's core crate should depend on `orion-db`:
```toml
orion-db = { path = "../../../shared/orion-db" }
```

## Vite Config Baseline

```typescript
import path from 'path'
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import { TanStackRouterVite } from '@tanstack/router-plugin/vite'

export default defineConfig({
  plugins: [
    TanStackRouterVite({ target: 'react', autoCodeSplitting: true }),
    react(),
    tailwindcss(),
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      // Add if using colocated modules outside frontend/:
      // '@modules': path.resolve(__dirname, '../modules'),
    },
    // REQUIRED when importing from modules outside frontend/.
    // Forces listed packages to resolve from frontend/node_modules/
    // instead of the importing file's directory.
    // Add every npm package used by modules/*/frontend/ code.
    dedupe: [
      'react', 'react-dom', 'react/jsx-runtime',
      '@tanstack/react-query', '@tanstack/react-router',
      'lucide-react', '@tauri-apps/api',
      'zod', 'react-hook-form',
      'clsx', 'tailwind-merge',
    ],
  },
  // Allow Vite dev server to serve files from outside frontend/
  server: {
    fs: { allow: [path.resolve(__dirname, '..')] },
  },
})
```

## Tauri Config (tauri.conf.json) Build Section

Use the object format with `cwd` since `tauri.conf.json` lives in `crates/tauri-app/`:

```json
"build": {
  "frontendDist": "../../frontend/dist",
  "devUrl": "http://localhost:5173",
  "beforeDevCommand": {
    "script": "npm run dev",
    "cwd": "../../frontend"
  },
  "beforeBuildCommand": {
    "script": "npm run build",
    "cwd": "../../frontend"
  }
}
```

**Critical:** `beforeDevCommand` must run `npm run dev` (starts live Vite HMR server), NOT `npm run build` (produces static files and exits). The `devUrl` expects a running server.

## Frontend package.json Scripts

```json
"scripts": {
  "dev": "vite",
  "build": "vite build",
  "typecheck": "tsc --noEmit",
  "lint": "eslint .",
  "preview": "vite preview"
}
```

**Note:** The `build` script runs `vite build` only — no `tsc`. Vite handles TypeScript compilation via esbuild. Run `npm run typecheck` separately for type checking. This split exists because `tsc -b` has project boundary restrictions that conflict with colocated modules outside `frontend/`.

## TypeScript Config (tsconfig.app.json)

All strict settings enabled:

```jsonc
{
  "compilerOptions": {
    "target": "ES2023",
    "lib": ["ES2023", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "moduleResolution": "bundler",
    "jsx": "react-jsx",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,
    "noUncheckedSideEffectImports": true,
    "verbatimModuleSyntax": true,
    "erasableSyntaxOnly": true,
    "noEmit": true,
    "skipLibCheck": true,
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"],
      "@modules/*": ["../modules/*"]
    }
  },
  "include": ["src", "../modules/*/frontend"]
}
```

## NPM Dependency Baseline

### Dependencies
- `react` ^19, `react-dom` ^19
- `@tanstack/react-query` ^5
- `@tanstack/react-router` ^1
- `@tauri-apps/api` ^2, `@tauri-apps/plugin-log` ^2
- `@hookform/resolvers` ^5, `react-hook-form` ^7, `zod` ^4
- `class-variance-authority` ^0.7, `clsx` ^2, `tailwind-merge` ^3
- `lucide-react` ^0.577
- `@fontsource-variable/atkinson-hyperlegible-next`, `@fontsource-variable/atkinson-hyperlegible-mono`
- `radix-ui` ^1, `shadcn` ^4, `tw-animate-css` ^1

### Dev Dependencies
- `typescript` ~5.9
- `vite` ^7, `@vitejs/plugin-react` ^5
- `tailwindcss` ^4, `@tailwindcss/vite` ^4
- `@tanstack/router-plugin` ^1
- `eslint` ^9, `typescript-eslint` ^8

## Theme Import

Each app's `index.css` should import from the shared theme:
```css
/* Shared theme */
@import "../../../shared/orion-theme/tokens/primitive.css";
@import "../../../shared/orion-theme/tokens/semantic.css";
@import "../../../shared/orion-theme/themes/low-stimulus.css";
@import "../../../shared/orion-theme/themes/light.css";
@import "../../../shared/orion-theme/themes/dark.css";
@import "../../../shared/orion-theme/themes/high-contrast.css";
@import "../../../shared/orion-theme/density.css";
@import "../../../shared/orion-theme/motion.css";
/* App-specific component tokens */
@import "./theme/tokens/component.css";
```
