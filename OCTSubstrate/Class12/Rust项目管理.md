内容：
* package, crate, module介绍
* 功能模块引入
* Workspace

为什么要做项目管理
* 组织大量代码
* 封装无需外部关心的实现细节
* 代码复用

## Rust项目管理的组件
* package: cargo工具用来构建、编译、测试的空间
* crate: 工具库(src/lib.rs)或可执行程序（src/main.rs）
  * rand
  * serde
  * diesel
* module: 在crate里组织代码，控制是否对其他模块可见

## 模块引入
* use
* pub use
* crate
* self
* super
* as

## Workspace
* 管理多个library, binary
* 共享cargo.lock和输出目录
* 依赖隔离
