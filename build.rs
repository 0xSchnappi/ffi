/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-09-27 16:36:12
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-09-27 16:48:23
 * @FilePath: /ffi/build.rs
 * @Description:
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */
// build.rs

fn main() {
    // ## 事先做成CMakeLists.txt，利用cmake编译c代码并生成函数库
    // use cmake::Config;
    // let dst = Config::new("ansic").build();
    use cmake;
    let dst = cmake::build("ansic");

    // ## 生成cargo链接参数
    // 若build.rs有任何修改，则重新编译
    println!("cargo:rerun-if-changed=build.rs");
    // 搜索生成的函数库
    println!("cargo:rustc-link-search=native={}", dst.display());
    // 链接生成的函数库
    println!("cargo:rustc-link-lib=static=simplemath");
}
