# Src Dependency Graph

`src/` 以下の各ファイルの import 依存関係を解析した結果グラフ。

```mermaid
graph LR
        subgraph "routes"
            src_routes__layout_svelte["+layout.svelte"]
            src_routes__layout_ts["+layout.ts"]
            src_routes__page_svelte["+page.svelte"]
        end
```
