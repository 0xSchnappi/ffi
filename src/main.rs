/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-09-27 16:34:31
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-09-27 16:38:24
 * @FilePath: /ffi/src/main.rs
 * @Description:
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */

mod binding;

fn main() {
    let left = 10isize;
    let right = 3isize;

    println!("{} + {} = {}", left, right, unsafe {
        binding::add(left, right)
    });
    println!("{} - {} = {}", left, right, unsafe {
        binding::sub(left, right)
    });
    println!("{} * {} = {}", left, right, unsafe {
        binding::mul(left, right)
    });
    println!("{} / {} = {}", left, right, unsafe {
        binding::div(left, right)
    });
}
