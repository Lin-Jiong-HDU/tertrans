use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("用法: tertrans <要翻译的文字 | 文本文件路径>");
        std::process::exit(1);
    }

    // If a single argument is a valid file path, read its contents
    let text = if args.len() == 1 && Path::new(&args[0]).is_file() {
        std::fs::read_to_string(&args[0]).unwrap_or_else(|e| {
            eprintln!("读取文件失败 {}: {}", args[0], e);
            std::process::exit(1);
        })
    } else {
        args.join(" ")
    };

    let api_key = env::var("GLM_API_KEY").expect("请设置环境变量 GLM_API_KEY");
    let base_url =
        env::var("GLM_BASE_URL").unwrap_or_else(|_| "https://open.bigmodel.cn/api/paas/v4".into());

    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(format!("{}/chat/completions", base_url))
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "glm-4-flash",
            "messages": [{
                "role": "system",
                "content": "你是一个翻译器。自动检测输入语言：如果是中文，翻译为英文；如果是英文，翻译为中文。只输出翻译结果，不要解释、不要注释、不要多余内容。"
            }, {
                "role": "user",
                "content": text
            }],
            "temperature": 0.1
        }))
        .send()
        .expect("请求失败");

    if !resp.status().is_success() {
        eprintln!("API 错误: {}", resp.status());
        eprintln!("{}", resp.text().unwrap_or_default());
        std::process::exit(1);
    }

    let body: serde_json::Value = resp.json().expect("解析响应失败");
    let result = body["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("翻译失败");

    let trimmed = result.trim();
    println!("{}", trimmed);

    // macOS: pbcopy, Linux: xclip, Wayland: wl-copy
    let (cmd, args) = if cfg!(target_os = "macos") {
        ("pbcopy", &[][..])
    } else {
        match std::process::Command::new("wl-copy").arg("").stderr(std::process::Stdio::null()).status() {
            Ok(_) => ("wl-copy", &[][..]),
            Err(_) => ("xclip", &["-selection", "clipboard"][..]),
        }
    };
    let mut child = std::process::Command::new(cmd)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    use std::io::Write;
    child.stdin.as_mut().unwrap().write_all(trimmed.as_bytes()).unwrap();
    child.wait().unwrap();
}
