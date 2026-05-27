use crate::config::AgentKind;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::Value;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

#[derive(Debug, Deserialize)]
pub struct Target {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub target_type: String,
    pub url: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub web_socket_debugger_url: Option<String>,
}

pub async fn list_targets(port: u16) -> Result<Vec<Target>, String> {
    let url = format!("http://127.0.0.1:{}/json/list", port);
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("HTTP error listTargets: {}", e))?;

    let targets: Vec<Target> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse targets: {}", e))?;

    Ok(targets)
}

/// Find the main window target for the given agent kind.
pub fn find_main_target<'a>(targets: &'a [Target], kind: &AgentKind) -> Option<&'a Target> {
    match kind {
        AgentKind::Codex => {
            if let Some(main) = targets
                .iter()
                .find(|t| t.url == "app://-/index.html" && t.target_type == "page")
            {
                return Some(main);
            }
            targets
                .iter()
                .find(|t| t.target_type == "page" && t.title == "Codex")
        }
        AgentKind::Antigravity => {
            if let Some(main) = targets.iter().find(|t| {
                t.url.starts_with("https://127.0.0.1:")
                    && t.target_type == "page"
                    && t.title == "Antigravity"
            }) {
                return Some(main);
            }
            targets
                .iter()
                .find(|t| t.target_type == "page" && t.title.contains("Antigravity"))
        }
    }
}

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

async fn make_cdp_request(
    ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    method: &str,
    params: Value,
) -> Result<Value, String> {
    let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
    let payload = serde_json::json!({
        "id": id,
        "method": method,
        "params": params
    });

    ws_stream
        .send(Message::Text(payload.to_string().into()))
        .await
        .map_err(|e| format!("Failed to send CDP message: {}", e))?;

    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            if let Some(msg_result) = ws_stream.next().await {
                match msg_result {
                    Ok(Message::Text(text)) => {
                        if let Ok(response) = serde_json::from_str::<Value>(&text) {
                            if response.get("id").and_then(|i| i.as_u64()) == Some(id as u64) {
                                if let Some(err) = response.get("error") {
                                    return Err(err.to_string());
                                }
                                return Ok(response.get("result").cloned().unwrap_or(Value::Null));
                            }
                        }
                    }
                    Ok(_) => {}
                    Err(e) => return Err(format!("WebSocket read error: {}", e)),
                }
            } else {
                return Err("WebSocket connection closed".to_string());
            }
        }
    })
    .await
    .map_err(|_| format!("CDP request timeout (method: {})", method))?
}

pub async fn inject_theme(port: u16, kind: &AgentKind, script: &str) -> Result<String, String> {
    let targets = list_targets(port).await?;
    let target =
        find_main_target(&targets, kind).ok_or("Could not find Agent main window target")?;

    let ws_url = target
        .web_socket_debugger_url
        .as_ref()
        .ok_or("Target has no WebSocket URL")?;
    let (mut ws_stream, _): (WebSocketStream<MaybeTlsStream<TcpStream>>, _) =
        connect_async(ws_url.as_str())
            .await
            .map_err(|e| format!("WebSocket connect failed: {}", e))?;

    make_cdp_request(&mut ws_stream, "Page.enable", serde_json::json!({})).await?;
    make_cdp_request(
        &mut ws_stream,
        "Runtime.evaluate",
        serde_json::json!({ "expression": script }),
    )
    .await?;

    let result = make_cdp_request(
        &mut ws_stream,
        "Page.addScriptToEvaluateOnNewDocument",
        serde_json::json!({ "source": script }),
    )
    .await?;

    let identifier = result
        .get("identifier")
        .and_then(|v| v.as_str())
        .ok_or("Failed to get identifier")?
        .to_string();

    let _ = ws_stream.close(None).await;
    Ok(identifier)
}

pub async fn clear_theme(
    port: u16,
    kind: &AgentKind,
    identifier: Option<&str>,
) -> Result<(), String> {
    let targets = list_targets(port).await?;
    let target =
        find_main_target(&targets, kind).ok_or("Could not find Agent main window target")?;

    let ws_url = target
        .web_socket_debugger_url
        .as_ref()
        .ok_or("Target has no WebSocket URL")?;
    let (mut ws_stream, _): (WebSocketStream<MaybeTlsStream<TcpStream>>, _) =
        connect_async(ws_url.as_str())
            .await
            .map_err(|e| format!("WebSocket connect failed: {}", e))?;

    let clear_script = r#"
        (function() {
            const style = document.getElementById('agent-theme-style');
            if (style) style.remove();
            console.log('Agent theme cleared.');
        })();
    "#;

    make_cdp_request(&mut ws_stream, "Page.enable", serde_json::json!({})).await?;
    make_cdp_request(
        &mut ws_stream,
        "Runtime.evaluate",
        serde_json::json!({ "expression": clear_script }),
    )
    .await?;

    if let Some(id) = identifier {
        let _ = make_cdp_request(
            &mut ws_stream,
            "Page.removeScriptToEvaluateOnNewDocument",
            serde_json::json!({ "identifier": id }),
        )
        .await;
    }

    let _ = ws_stream.close(None).await;
    Ok(())
}

pub async fn reload_page(port: u16, kind: &AgentKind) -> Result<(), String> {
    let targets = list_targets(port).await?;
    let target =
        find_main_target(&targets, kind).ok_or("Could not find Agent main window target")?;

    let ws_url = target
        .web_socket_debugger_url
        .as_ref()
        .ok_or("Target has no WebSocket URL")?;
    let (mut ws_stream, _): (WebSocketStream<MaybeTlsStream<TcpStream>>, _) =
        connect_async(ws_url.as_str())
            .await
            .map_err(|e| format!("WebSocket connect failed: {}", e))?;

    make_cdp_request(&mut ws_stream, "Page.enable", serde_json::json!({})).await?;
    make_cdp_request(&mut ws_stream, "Page.reload", serde_json::json!({})).await?;

    let _ = ws_stream.close(None).await;
    Ok(())
}
