#!/bin/bash

# 用命令行执行，不要用脚本
# 添加迁移（创建表）
export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter
sqlx migrate add create_subscriptions_table