#!/bin/bash

# prepare执行的与调用cargo build时所作的工作类似，它将这些查询结果保存在一个元数据文件(sql-data.json)中，
# 由sqlx自行检测，可以完全跳过查询从而执行离线构建
# 注：sqlx prepare 必须作为cargo的子命令来执行，--之后的参数会传给cargo自身。
# 由于所有的SQL查询都被放在以lib.rs为根的模块中，因此这里必须要指明命令作用于lib部分
cargo sqlx prepare -- --lib