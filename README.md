# TinyApp
这只是基于 iced 构建跨平台应用的学习总结与分享，

## 目录结构组织
```bash
├── Cargo.lock
├── Cargo.toml
├── build.rs                                    # 编译期补充编译常量
├── rust-toolchain                              # 约束编译项目版本号
├── src                                         
│   ├── app.rs                                  # App / Message /
│   ├── main.rs                                 # 入口点
│   ├── msyh.ttc                                # 需要加载的字体
│   ├── pages                                   # 多页面集合 mod 目录
│   │   ├── about.rs
│   │   ├── contact.rs
│   │   └── home.rs
│   └── pages.rs                                # mod pages
└── tests                                       # 单元测试集合目录
└── tiny_macro                                  # 辅助项目的导出宏
    ├── Cargo.toml
    └── src
        └── lib.rs
```

## 更新说明
### 2024-08-03 / 增加 wasm 编译说明
+ edge 浏览器各种各样的原因，可能会无法正常显示，考虑 chrome或 safari (本人只测试了这个)
+ 预装命令
```shell
# 安装工具链
rustup target add wasm32-unknown-unknown

# 安装命令行生成 (对应 wasm-bindgen 命令)
cargo install wasm-bindgen-cli
```
+ 发布与生成
```shell
# 在项目目录中运行 （当前用例为 rust_tiny  根目录)
cargo build --target wasm32-unknown-unknown

# 生成绑定(在命令当前生成输出目录 out_wasm )
wasm-bindgen ./target/wasm32-unknown-unknown/debug/tiny.wasm --out-dir out_wasm --web

```
+ 启动与查看
```shell
# 查看当前根目录下，新增的 index.html 文件

# 用自带的 python3 启动一个简单的 web 服务器(默认端口 8000)
pyhton3 -m http.server

# 使用 safari / chrome / fixfox 打开地址 http://localhost:8000
```
### 2024-07-31 / rust 1.80.0 / iced 1.21.0 
+ 基础的消息嵌套的应用
+ 消息路由通过派生宏进行扩展（减少代码复用）
+ 中文字体加载与显示的用例
+ 多页切换逻辑