# minigrep

一个使用 Rust 编写的简易命令行文本搜索工具，支持按行查找文本，并可通过环境变量启用大小写不敏感搜索。

## 功能

- 在指定文件中搜索包含关键词的行
- 默认进行大小写敏感匹配
- 支持通过 `IGNORE_CASE` 环境变量进行大小写不敏感匹配
- 使用 Rust 标准库实现，无第三方依赖
- 包含核心搜索逻辑的单元测试

## 环境要求

- Rust 工具链（建议使用最新稳定版）
- Cargo

项目使用 Rust 2024 edition。

## 构建

```bash
cargo build
```

构建完成后，可执行文件位于 `target/debug/minigrep`。

## 使用方法

```text
cargo run -- <关键词> <文件路径>
```

例如，搜索 `src/example.txt` 中包含 `nobody` 的行：

```bash
cargo run -- nobody src/example.txt
```

输出：

```text
I'm nobody! Who are you?
Are you nobody, too?
```

也可以先构建，再直接运行：

```bash
./target/debug/minigrep nobody src/example.txt
```

### 大小写不敏感搜索

设置 `IGNORE_CASE` 环境变量后，搜索将忽略大小写。变量的具体值不会被检查，只要该变量存在即可：

```bash
IGNORE_CASE=NOBODY cargo run -- nobody src/example.txt
```

Windows PowerShell：

```powershell
$env:IGNORE_CASE = "1"
cargo run -- nobody src/example.txt
```

## 参数

| 参数 | 说明 |
| --- | --- |
| `<关键词>` | 要搜索的文本，区分大小写时按原样匹配 |
| `<文件路径>` | 要读取的 UTF-8 文本文件路径 |
| `IGNORE_CASE` | 可选环境变量；存在时启用大小写不敏感搜索 |

如果缺少关键词或文件路径，程序会报告参数错误并以非零状态码退出；文件无法读取时也会报告错误。

## 项目结构

```text
.
├── Cargo.toml       # 项目配置
├── src/
│   ├── main.rs      # 命令行参数解析、文件读取和程序入口
│   ├── lib.rs       # 大小写敏感/不敏感的搜索函数及测试
│   └── example.txt  # 示例文本
└── README.md
```

核心 API：

- `search(query, contents)`：大小写敏感搜索
- `search_case_insensitive(query, contents)`：大小写不敏感搜索

两个函数都会返回匹配行的字符串切片，不会复制原始文本内容。

## 测试

运行全部测试：

```bash
cargo test
```

当前测试覆盖：

- 单个匹配结果
- 大小写敏感匹配
- 大小写不敏感匹配

## 许可证

当前项目未声明许可证。
