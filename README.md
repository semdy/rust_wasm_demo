# Instructions
Rust借助wasm-bindgen通过web-sys调用web canvas api示例

1. 构建wasm文件
```shell
# 安装 wasm-pack
$ cargo install wasm-pack
# 构建 wasm 项目
$ wasm-pack build --target web
```

2. web集成

```javascript
import init, { draw_circle } from 'rust_wasm_project'

init().then(() => {
    draw_circle('my_canvas')
})
```