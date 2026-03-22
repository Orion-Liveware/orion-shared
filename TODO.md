# Shared Libraries — TODO

Extract reusable infrastructure from my-orion-life into shared libraries used by all apps (my-orion-life, orion-health, orion-coding).

## Phase 1: Shared Rust Crate (`orion-db`)

### Extract from my-orion-life/crates/core/
- [x] Create `shared/orion-db/` Rust crate with workspace-compatible Cargo.toml
- [x] Extract `Db` type alias (`Surreal<Any>`) and `init_db(url)` — universal DB connection
- [x] Extract `init_migration_table(db)` — just the `_migration` table setup (split from `init_core_schema`)
- [x] Extract `CoreError` enum with `From<surrealdb::Error>` — universal SurrealDB error wrapper
- [x] Extract `CommandError` enum with custom `Serialize` impl — universal Tauri IPC error boundary
- [x] Extract `From<CoreError> for CommandError` impl
- [x] Add `run_schema(db, sql, module_name)` helper — DRY pattern from `modules/*/src/schema.rs`
- [x] Add `specta` optional feature flag (same pattern as current `mol-core`)
- [x] Write tests for all extracted functions (in-memory SurrealDB)
- [x] Update my-orion-life's `crates/core/` to depend on `orion-db` and re-export, keeping domain/module code local
- [x] Update all my-orion-life modules to compile against the refactored core
- [x] Verify all 51 my-orion-life tests still pass

## Phase 2: Shared Theme Package (`orion-theme`)

### Extract from my-orion-life/frontend/src/theme/
- [x] Create `shared/orion-theme/` directory
- [x] Move `tokens/primitive.css` — spacing, typography, motion, sizing
- [x] Move `tokens/semantic.css` — surfaces, text, interactive, feedback
- [x] Move `themes/light.css`, `dark.css`, `low-stimulus.css`, `high-contrast.css`
- [x] Move `density.css` — compact/comfortable/spacious modes
- [x] Move `motion.css` — reduced motion defaults + calm mode
- [x] Keep `tokens/component.css` in each app (app-specific sidebar/panel dimensions)
- [x] Update my-orion-life's `index.css` to import from `../../shared/orion-theme/`
- [x] Verify my-orion-life frontend type-checks

## Phase 3: Shared Config Templates

- [x] Create `shared/templates/` directory
- [x] Document Cargo workspace dependency versions (surrealdb, tauri, specta pins)
- [x] Document Vite config baseline (plugins, aliases)
- [x] Document tsconfig strict settings
- [x] Document npm dependency baseline (React 19, TanStack, Tailwind v4, shadcn)

## Decisions
- [x] shadcn/ui components: each app owns its own (NOT shared) — shadcn is designed for copy-paste ownership
- [x] App shell components (sidebar, topbar): each app owns its own — too coupled to app-specific routing
- [x] Sub-modules (tasks, notes, etc.): stay in their respective apps — extract only when a second app needs the same module
