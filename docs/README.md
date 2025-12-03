# 项目文档

本目录包含了 Astra AI API 项目的详细文档和开发指南。

## 📋 文档目录

### 🌐 多语言数据转换模式
- **[完整实现文档](multilingual-data-conversion-pattern.md)**
  - 详细的实现方案说明
  - 数据结构设计和转换函数
  - 最佳实践和性能优化建议
  - 扩展模式和应用场景

- **[代码模板](i18n-conversion-template.rs)**
  - 可直接复制使用的代码模板
  - 完整的函数实现和注释
  - API Handler 使用示例
  - 数据库迁移脚本

- **[快速参考卡片](i18n-conversion-cheatsheet.md)**
  - 核心模式和关键步骤
  - 常用代码片段和函数
  - JSON 数据格式规范
  - 快速上手指南

## 🎯 核心特性

### 多语言转换模式
项目实现了完整的多语言数据转换解决方案：

- **类型安全**: 使用 SQLx 的 JSON 类型映射
- **智能回退**: 指定语言 → 英文 → 默认值的回退机制
- **高性能**: 批量转换和优化的 JSON 解析
- **易扩展**: 支持新语言和复杂嵌套结构
- **错误处理**: 完善的默认值和异常处理

### 应用场景
- 产品信息的国际化显示
- 动态内容的多语言支持
- 配置项的本地化管理
- 用户界面的文本转换

## 🚀 快速开始

1. **查看模式文档**: 阅读 `multilingual-data-conversion-pattern.md` 了解完整方案
2. **复制代码模板**: 使用 `i18n-conversion-template.rs` 中的模板代码
3. **参考快速卡片**: 查看 `i18n-conversion-cheatsheet.md` 快速实现

## 📝 使用示例

```rust
// 在 API Handler 中应用多语言转换
pub async fn get_items(
    auth_user: AuthUser,  // 包含语言偏好
) -> Result<impl IntoResponse> {
    // 1. 获取数据库数据（包含 JSON 多语言字段）
    let (records, _) = repo::get_items(...).await?;

    // 2. 应用多语言转换
    let items = convert_user_power_records(records, &auth_user.lang);

    // 3. 返回响应
    Ok(Json(ApiResponse::success(items)))
}
```

## 🔧 技术栈

- **Rust**: 类型安全的系统编程语言
- **SQLx**: 异步、类型安全的数据库访问库
- **Serde**: 序列化和反序列化库
- **Axum**: 高性能的 Web 框架
- **MySQL**: 关系型数据库（支持 JSON 字段）

## 📚 其他资源

### 相关 RFC 和标准
- [RFC 7231](https://tools.ietf.org/html/rfc7231) - HTTP/1.1 Semantics and Content
- [JSON Schema](https://json-schema.org/) - JSON 数据结构验证

### 社区和工具
- [SQLx 文档](https://github.com/launchbadge/sqlx) - 异步数据库访问库
- [Serde 文档](https://serde.rs/) - 序列化框架
- [Axum 文档](https://github.com/tokio-rs/axum) - Web 框架

## 🤝 贡献指南

如果你有新的文档或改进建议：

1. 创建新的文档文件
2. 更新此 README.md 的索引
3. 提交 Pull Request
4. 确保文档格式和内容的一致性

## 📞 支持

如有问题或需要帮助，请：

1. 查看相关文档
2. 搜索项目的 Issues
3. 创建新的 Issue 描述问题
4. 联系开发团队

---

💡 **提示**: 所有文档都经过实践验证，可以直接应用到项目中！