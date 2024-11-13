# 使用rust最新的稳定版
FROM rust:1.82.0

# 把工作目录切换到app（相当于cd app）
# app文件夹将由Docker为我们创建，防止它不存在
WORKDIR /app

# 为链接配置安装所需的系统依赖
RUN apt update && apt install lld clang -y

# 将工作环境中的所有文件复制到Docker镜像中
COPY . .

# 开启sqlx的离线模式
ENV SQLX_OFFINE=true

# 开始编译二进制文件（使用release参数优化以提高速度）
RUN cargo build --release

# 当执行docker run时启动二进制文件
ENTRYPOINT ["./target/release/zero2prod"]