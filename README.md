# ASCII-Painting Backend

项目 [ASCII-Painting](https://github.com/stand114514/ASCII-Painting) 的后端实现，使用 Rust 的 Axum 框架。

API 文档请参考： [API 文档](https://apifox.com/apidoc/shared-93744361-5714-4750-a3fc-9190918b9cef/)。（最终认准原库持有者）

部分参数用环境变量配置，或者可以使用`.env`文件配置。

如果选择使用`.env`，项目根目录下应有`.env`文件，包含如下字段：

- `DATABASE_URL`：数据库连接字符串
- `SECRET_KEY`：JWT 加密密钥，暂未使用
- `RUST_BACKTRACE`：Rust 后端错误栈开关，建议设置为`full`，以便排查错误
- `PORT`: 程序运行的端口