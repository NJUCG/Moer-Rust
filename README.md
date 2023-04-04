## Moer-Lite 渲染器框架的Rust实现

运行方法：

```shell
# 在 moer-lite-rs 目录下
$ cargo run --release -- model-dir-path
```
结果会输出为`model-dir-path/{output.filename}`.

目前实现了Lab1要求的与几个几何体的求交，以及BVH和Octree.

正确实现了area-lights、bunny、two-spotlights、cornell-box所需要的组件.

目前的BVH实现了SAH分割，但没有做树的平衡；不过得益于
计算细节的优化，目前效率已经相当高，在bunny的测试场景下效率优于
Moer-lite中的Embree加速结构.

## TODO
- BVH的SAH分割后树的平衡调整
- 引入Embree的Rust binding作为加速结构：[Embree](https://github.com/Twinklebear/embree-rs)