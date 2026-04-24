# tertrans

> 一个基于 GLM 大模型的终端翻译工具，将中文（或其他语言）快速翻译为英文，并自动复制到剪贴板。

---

## 功能特性

- **一行命令翻译**：直接在终端输入待翻译的文字，立即获得英文译文。
- **自动复制**：翻译结果自动写入系统剪贴板，支持 macOS、Linux（X11 / Wayland）。
- **自定义 API**：支持通过环境变量切换任意兼容 OpenAI Chat Completions 接口的后端。

---

## 依赖

- [Rust](https://www.rust-lang.org/) 工具链（建议 1.70+）
- [智谱 AI（BigModel）](https://open.bigmodel.cn/) 账号及 API Key
- 剪贴板工具（按系统选一）：
  - macOS：内置 `pbcopy`，无需额外安装
  - Linux Wayland：`wl-copy`（`wl-clipboard` 包）
  - Linux X11：`xclip`

---

## 安装

```bash
git clone https://github.com/Lin-Jiong-HDU/tertrans.git
cd tertrans
cargo build --release
```

编译完成后，二进制文件位于 `target/release/tertrans`。  
可将其复制到 `PATH` 中的目录以便全局调用：

```bash
# 示例（Linux / macOS）
sudo cp target/release/tertrans /usr/local/bin/
```

---

## 配置

在使用前，需要设置以下环境变量：

| 变量名          | 是否必填 | 说明                                                                                   |
|----------------|---------|--------------------------------------------------------------------------------------|
| `GLM_API_KEY`  | **必填** | 智谱 AI 的 API Key，在 [BigModel 控制台](https://open.bigmodel.cn/) 创建              |
| `GLM_BASE_URL` | 可选    | API 基础地址，默认为 `https://open.bigmodel.cn/api/paas/v4`，可替换为兼容接口的地址    |

推荐将变量写入 `~/.bashrc` 或 `~/.zshrc`：

```bash
export GLM_API_KEY="your_api_key_here"
```

---

## 使用方法

```bash
tertrans <要翻译的文字>
```

### 示例

```bash
$ tertrans 你好，世界
Hello, World

$ tertrans 今天天气很好
The weather is very nice today.

$ tertrans Hello world
Hello world
```

翻译结果同时打印到终端并自动写入剪贴板，可直接粘贴使用。

---

## 工作原理

1. 读取命令行参数作为待翻译文本。
2. 携带 `GLM_API_KEY` 调用智谱 AI 的 `glm-4-flash` 模型（Chat Completions 接口）。
3. 系统提示词要求模型仅输出翻译结果，不附加任何解释。
4. 将翻译结果输出到 `stdout`，并通过 `pbcopy` / `wl-copy` / `xclip` 写入剪贴板。

---

## 许可证

本项目采用 [MIT License](LICENSE) 授权。
