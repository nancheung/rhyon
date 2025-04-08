# 使用官方 Rust 镜像作为基础镜像
FROM rust:latest AS builder

# 设置工作目录
WORKDIR /app

# 将项目的 Cargo 配置文件复制到容器中
COPY Cargo.toml Cargo.lock ./

# 预构建依赖以利用缓存
RUN mkdir temp_src && echo "fn main() {}" > temp_src/main.rs
RUN cargo build --release || true

# 复制项目代码到容器中
COPY . .

# 构建项目
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && apt-get clean
# 设置工作目录
WORKDIR /app

# 复制构建好的二进制文件
COPY --from=builder /app/target/release/rhyon ./

# 暴露应用运行的端口
EXPOSE 8080

# 设置启动命令
CMD ["./rhyon"]