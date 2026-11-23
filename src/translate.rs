//! 通过 OpenAI 兼容的 chat completions 接口翻译 OCR 文字。
//!
//! 请求体写入 0600 临时文件，再由 `curl --config -` 读取 API key，
//! 避免密钥和截图文字出现在进程参数中。
use crate::config;
use serde_json::{Value, json};
use std::{
    env, fs,
    io::Write,
    os::unix::fs::OpenOptionsExt,
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

const MAX_CHARS: usize = 12_000;

pub fn translate(text: &str) -> Result<String, String> {
    if text.trim().is_empty() {
        return Err("选区内没有可翻译的文字".into());
    }
    if text.chars().count() > MAX_CHARS {
        return Err("文字太多，请缩小选区".into());
    }
    let config = config::translate_config()?;
    let endpoint = endpoint(&config.url);
    let body = json!({
        "model": config.model,
        "temperature": 0.2,
        "messages": [
            {
                "role": "system",
                "content": format!(
                    "You are a translation engine. Translate from {} to {}. Preserve line breaks and formatting. Output only the translation.",
                    config.from, config.to
                )
            },
            { "role": "user", "content": text }
        ]
    });
    let request = request_text(&endpoint, &config.api_key, &body.to_string())?;
    parse_response(&request)
}

fn endpoint(base: &str) -> String {
    let base = base.trim_end_matches('/');
    if base.ends_with("/chat/completions") {
        base.to_owned()
    } else {
        format!("{base}/chat/completions")
    }
}

fn request_text(endpoint: &str, api_key: &str, body: &str) -> Result<String, String> {
    let path = env::temp_dir().join(format!(
        "babry-translate-{}-{}.json",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    let result = (|| {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&path)
            .map_err(|error| format!("无法创建翻译请求文件: {error}"))?;
        file.write_all(body.as_bytes())
            .map_err(|error| format!("无法写入翻译请求: {error}"))?;
        drop(file);

        let mut child = Command::new("curl")
            .args([
                "--silent",
                "--show-error",
                "--fail-with-body",
                "--connect-timeout",
                "10",
                "--max-time",
                "60",
                "--request",
                "POST",
                "--header",
                "Content-Type: application/json",
                "--config",
                "-",
                "--data-binary",
            ])
            .arg(format!("@{}", path.display()))
            .arg(endpoint)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|_| "翻译需要 curl，请先安装 curl".to_string())?;
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| "无法写入翻译请求".to_string())?;
        writeln!(
            stdin,
            "header = \"Authorization: Bearer {}\"",
            curl_quote(api_key)?
        )
        .map_err(|error| format!("无法写入翻译请求: {error}"))?;
        drop(stdin);
        let output = child
            .wait_with_output()
            .map_err(|error| format!("翻译请求失败: {error}"))?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Ok(value) = serde_json::from_str::<Value>(&stdout)
            && let Some(message) = value["error"]["message"].as_str()
        {
            return Err(format!("翻译服务返回错误: {message}"));
        }
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let detail = stderr.trim();
            return Err(if detail.is_empty() {
                format!("翻译请求失败，curl 退出码 {:?}", output.status.code())
            } else {
                format!("翻译请求失败: {detail}")
            });
        }
        Ok(stdout.into_owned())
    })();
    let _ = fs::remove_file(&path);
    result
}

fn curl_quote(value: &str) -> Result<String, String> {
    if value.chars().any(char::is_control) {
        return Err("翻译配置的 api_key 含有控制字符".into());
    }
    Ok(value.replace('\\', "\\\\").replace('"', "\\\""))
}

fn message_content(body: &str) -> Option<String> {
    let value: Value = serde_json::from_str(body).ok()?;
    let content = &value["choices"].get(0)?["message"]["content"];
    let text = if let Some(text) = content.as_str() {
        text.to_owned()
    } else {
        content
            .as_array()?
            .iter()
            .filter_map(|part| part["text"].as_str())
            .collect::<Vec<_>>()
            .join("")
    };
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_owned())
}

fn parse_response(body: &str) -> Result<String, String> {
    message_content(body).ok_or_else(|| {
        format!(
            "翻译服务没有返回内容: {}",
            body.trim().chars().take(300).collect::<String>()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_openai_compatible_endpoints() {
        assert_eq!(
            endpoint("https://api.openai.com/v1/"),
            "https://api.openai.com/v1/chat/completions"
        );
        assert_eq!(
            endpoint("https://example.com/v1/chat/completions"),
            "https://example.com/v1/chat/completions"
        );
    }

    #[test]
    fn reads_string_and_part_content() {
        assert_eq!(
            parse_response(r#"{"choices":[{"message":{"content":"你好"}}]}"#).unwrap(),
            "你好"
        );
        assert_eq!(
            parse_response(
                r#"{"choices":[{"message":{"content":[{"type":"text","text":"第一行"},{"type":"text","text":"第二行"}]}}]}"#
            )
            .unwrap(),
            "第一行第二行"
        );
        assert!(parse_response(r#"{"error":{"message":"bad key"}}"#).is_err());
    }
}
