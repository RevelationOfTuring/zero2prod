#!/bin/bash

# Shell将会打印出每个命令执行前的完整形式，并且在输出中添加+符号
set -x

# -e：如果脚本中的任何命令返回非零状态，则立即退出脚本。这有助于在脚本中尽早发现错误，并防止错误的命令影响后续的脚本执行;
# -o pipefail：如果管道中的任何命令返回非零状态，则整个管道命令失败，脚本也会立即退出。这意味着即使管道中的某个命令成功，只要有一个命令失败，整个管道就会被视为失败。
set -eo pipefail

# command -v psql: 内置命令，用于查找指定命令的完整路径。当您使用 command -v psql 时，它会检查系统的 PATH 环境变量中是否存在psql命令。如果有，返回其完整路径
# -x 是一个用于检查文件是否可执行的条件测试操作符。它通常用于 if 语句中，以检查某个文件是否具有执行权限。
if ! [ -x "$(command -v psql)" ]; then
    # 如果psql命令不存在于PATH中，或其没有执行权限
    echo >&2 "Error: psql is not installed." # 将命令的输出重定向到标准错误输出（stderr）
    # 终止脚本并返回一个非零的退出状态码（退出状态码为0表示脚本成功执行完毕）
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    exit 1
fi

# 检查是否已设置自定义用户名。如果未设置，默认为"postgres"
DB_USER=${POSTGRES:=postgres}
# 检查是否设置密码。如果未设置，默认为"password"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# 检查是否设置自定义数据库名称。如果未设置，默认为"newsletter"
DB_NAME="${POSTGRES_DB:=newsletter}"
# 检查是否设置数据库端口。如果未设置，默认为5432
DB_PORT="${POSTGRES_PORT:=5432}"

# 如果已经运行了docker中的postgres，则允许跳过docker run步骤
if [[ -z "$SKIP_DOCKER" ]]; then
    # 使用docker启动数据库
    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${POSTGRES_PASSWORD} \
        -e DB_NAME=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d postgres \
        postgres -N 1000 # 为了测试，增加了最大连接数

    # docker run的参数说明：
    # -e 或 --env 参数用于设置环境变量。在 Docker 容器中，环境变量可以用来传递配置信息，例如数据库连接字符串、API 密钥等；
    # -d 或 --detach 参数用于在后台运行容器。当你使用 -d 参数时，Docker 会在后台启动容器，并立即返回一个容器 ID。这意味着你可以在不等待容器启动完成的情况下，继续执行其他命令；
    # -p 用于将容器的端口映射到宿主机上。格式为 host_port:container_port，其中 host_port 是宿主机上的端口号，container_port 是容器内部的端口号
fi

# 保持对Postgres的轮询，直到它准备好接受命令
export PGPASSWORD=${DB_PASSWORD}
until psql -h "localhost" -U postgres -p 5432 -d postgres -c '\q'; do
    echo >&2 "Postgres is still unavailbale - sleeping"
    sleep 1
done

echo >&2 "Postgres is up and running on port ${DB_PORT}"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"

# 注： 使用以下命令迁移数据库
# SKIP_DOCKER=true ./scripts/init_db.sh