# openworkers-transform

SWC-based TypeScript/JavaScript transform for OpenWorkers.

- Strips TypeScript types
- Transforms `export default { ... }` → `globalThis.default = { ... }` (required for V8 classic script evaluation)

## Usage

```rust
use openworkers_transform::{parse_worker_code, CodeLanguage};

let code = b"export default { fetch() { return new Response('hello'); } }";
let result = parse_worker_code(code, CodeLanguage::JavaScript).unwrap();
// => "globalThis.default = { fetch () { return new Response('hello'); } };"
```

## Supported patterns

| Pattern | Result |
|---------|--------|
| `export default { ... }` | `globalThis.default = { ... }` |
| `export default function f() {}` | `globalThis.default = function f() {}` |
| `export default class C {}` | `globalThis.default = class C {}` |
| `export { x as default }` | `globalThis.default = x` |
| Re-exports (`from '...'`) | Kept as-is |
| Named exports (`export const`) | Kept as-is |
