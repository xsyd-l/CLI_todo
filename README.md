# CLI_todo

一个使用 Rust 编写的命令行待办事项工具。任务保存在运行命令时所在目录下的 `task.json` 文件中。

## 环境要求

- Rust 工具链（包含 Cargo）

## 构建

```bash
git clone https://github.com/xsyd-l/CLI_todo.git
cd CLI_todo
cargo build --release
```

构建完成后，可执行文件位于 `target/release` 目录中。

## 使用方法

在项目目录中使用 Cargo 运行：

```bash
cargo run -- --help
```

也可以构建后直接运行可执行文件。以下示例使用 `cargo run --`：

```bash
# 查看帮助和版本
cargo run -- --help
cargo run -- --version

# 添加任务（标题可以包含空格）
cargo run -- add "购买牛奶"

# 列出任务
cargo run -- list

# 将 ID 为 1 的任务标记为完成
cargo run -- done 1

# 删除 ID 为 1 的任务
cargo run -- remove 1
```

每个任务包含一个递增的数字 ID、标题、完成状态和创建时间。`list` 会输出任务数据的 JSON 内容。

## 数据文件

程序使用当前工作目录中的 `task.json` 保存任务。如果文件不存在，首次添加任务时会创建它。数据示例：

```json
[
  {
    "done": false,
    "title": "购买牛奶",
    "id": 1,
    "time": "2026-10-30 14:30:06.858365600 +08:00"
  }
]
```

## 许可证

[MIT](LICENSE)
