# 构建阶段
FROM rust:1.75-slim as builder

# 设置工作目录
WORKDIR /app

# 安装系统依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libmysqlclient-dev \
    && rm -rf /var/lib/apt/lists/*

# 复制Cargo配置文件
COPY Cargo.toml Cargo.lock ./

# 创建虚拟源文件以缓存依赖
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub mod app;" > src/lib.rs && \
    echo "pub mod config;" > src/config.rs && \
    echo "pub mod error;" > src/error.rs && \
    echo "pub mod state;" > src/state.rs

# 构建依赖
RUN cargo build --release && rm -rf src

# 复制源代码
COPY src ./src
COPY migrations ./migrations

# 构建应用
RUN touch src/main.rs && cargo build --release

# 运行阶段
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    libssl3 \
    libmysqlclient21 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 创建非root用户
RUN groupadd -r appuser && useradd -r -g appuser appuser

# 设置工作目录
WORKDIR /app

# 复制构建的二进制文件
COPY --from=builder /app/target/release/server .

# 创建必要的目录
RUN mkdir -p uploads && chown -R appuser:appuser /app

# 切换到非root用户
USER appuser

# 暴露端口
EXPOSE 8080

# 健康检查
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# 启动应用
CMD ["./server"]