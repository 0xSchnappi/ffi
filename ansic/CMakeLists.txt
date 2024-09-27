# 设置允许的cmake最低版本
cmake_minimum_required(VERSION 3.12)

# 设置目标名称
set(TARGET simplemath)

# 设置项目名称及使用的开发语言
project(${TARGET} LANGUAGES C)

# 设置C语言标准
set(CMAKE_C_STANDARD 23)

# 生成静态库，及相应的c代码
add_library(${TARGET} STATIC "src/simplemath.c")

# 设置头文件搜索目录
target_include_directories(${TARGET} PRIVATE "include")

# 设置目标生成目录
install(TARGETS ${TARGET} DESTINATION .)
