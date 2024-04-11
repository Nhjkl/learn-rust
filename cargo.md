# release profile

- release profile:
  - 是预定义的
  - 可自定义：可使用不同的配置，对代码编译拥有更多的控制
- 每个profile的配置都独立与其他的profile

- Cargo 主要的两个profile
  - dev profile：适用于开发， `cargo build`
  - release profile：适用于发布， `cargo build --release`

# 文档注释 https://rustwiki.org/zh-CN/book/ch14-02-publishing-to-crates-io.html

`cargo doc --open`

# 使用 pub Use 导出方便使用的公共 API

- 问题：crate 的程序结构在开发时对于开发者很合理，但对于它的使用者不够方便

  - 开发者会把程序结构分为很多层，使用者想找到这种深层结构中的某个类型很费劲例如:
  - 麻烦： my_crate:some_module::another_module::UsefulType;
  - 方便： my_crate::UsefulType;

- 解决办法：
  - 不需要重新组织内部代码结构
  - 使用 pub use：可以重新导出，创建一个与内部私有结构不同的对外公共结构
