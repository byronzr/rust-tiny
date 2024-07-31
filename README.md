# TinyApp
这只是基于 iced 构建跨平台应用的学习总结与分享，

# 目录结构组织
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
└── Tiny_Macro                                  # 输助项目的导出宏
    ├── Cargo.toml
    └── src
        └── lib.rs
```

# 2024-07-31
基于 rust 1.80.0 与 iced 1.21.0 的项目组织
+ 基础的消息嵌套的应用
+ 消息路由通过派生宏进行扩展（减少代码复用）
+ 中文字体加载与显示的用例
+ 多页切换逻辑