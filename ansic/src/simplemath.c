/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-09-27 16:35:51
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-09-27 16:36:25
 * @FilePath: /ffi/ansic/src/simplemath.c
 * @Description: 
 * 
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved. 
 */
// simplemath.c

#include "simplemath.h"

int add(int left, int right) {
    return left + right;
}

int sub(int left, int right) {
    return left - right;
}

int mul(int left, int right) {
    return left * right;
}

int div(int left, int right) {
    return left / right;
}
