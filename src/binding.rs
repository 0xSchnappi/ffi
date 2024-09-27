/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-09-27 16:36:47
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-09-27 16:38:08
 * @FilePath: /ffi/src/binding.rs
 * @Description:
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */
// binding.rs

// 链接动态库
//#[link(name = "simplemath", kind = "dylib")]
// 链接静态库
#[link(name = "simplemath", kind = "static")]
extern "C" {
    pub fn add(left: isize, right: isize) -> isize;
    pub fn sub(left: isize, right: isize) -> isize;
    pub fn mul(left: isize, right: isize) -> isize;
    pub fn div(left: isize, right: isize) -> isize;
}
