## Moer-Lite 渲染器框架的Rust实现

运行方法：

```shell
# 在根目录下
$ cargo run --release -- model-dir-path
```
结果会输出为`model-dir-path/{output.filename}`.

目前实现了Lab1要求的与几个几何体的求交，以及BVH和Octree,

其中BVH实现的性能很高，可以将`src/function_layer/acceleration/bvh.rs`
中的`MAX_PRIMS_IN_NODE`改为1获取最佳性能。
