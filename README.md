## Moer-Lite 渲染器框架的Rust实现

运行方法：

```shell
# 在 moer-lite-rs 目录下
$ cargo run --release -- model-dir-path
```
结果会输出为`model-dir-path/{output.filename}`.

目前实现了Lab1要求的与几个几何体的求交，以及BVH和Octree.
正确实现了area-lights、bunny、two-spotlights所需要的组件.
