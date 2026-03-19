mod pdk;

use crate::pdk::types::*;
use anyhow::{Result, anyhow};
use extism_pdk::*;
use htmd::HtmlToMarkdown;
use serde_json::json;
use std::collections::BTreeMap;

fn fetch(input: CallToolRequest) -> Result<CallToolResult> {
    let args = input.request.arguments.unwrap_or_default();
    let url = args
        .get("url")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Please provide a url"))?;

    // Create HTTP request
    let mut req = HttpRequest {
        url: url.to_string(),
        headers: BTreeMap::new(),
        method: Some("GET".to_string()),
    };

    // Add a user agent header to be polite
    req.headers
        .insert("User-Agent".to_string(), "fetch-tool/1.0".to_string());

    // Perform the request
    let res = http::request::<()>(&req, None)?;

    // Convert response body to string
    let body = res.body();
    let html = String::from_utf8_lossy(body.as_slice());

    let converter = HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style"])
        .build();

    // Convert HTML to markdown
    match converter.convert(&html) {
        Ok(markdown) => Ok(CallToolResult {
            content: vec![ContentBlock::Text(TextContent {
                text: markdown,

                ..Default::default()
            })],
            ..Default::default()
        }),
        Err(e) => Ok(CallToolResult::error(format!(
            "Failed to convert HTML to markdown: {}",
            e
        ))),
    }
}

// ---------------------------------------------------------------------------
// Implemented handlers
// ---------------------------------------------------------------------------

pub(crate) fn list_tools(_input: ListToolsRequest) -> Result<ListToolsResult> {
    let input_schema: schemars::Schema = serde_json::from_value(json!({
        "type": "object",
        "properties": {
            "url": {
                "type": "string",
                "description": "The URL to fetch",
            },
        },
        "required": ["url"],
    }))?;

    Ok(ListToolsResult {
        tools: vec![Tool {
            name: "fetch".into(),
            description: Some(
                "Enables to open and access arbitrary text URLs. Fetches the contents of a URL and returns its contents converted to markdown".into(),
            ),
            input_schema,
            annotations: Some(ToolAnnotations {
                read_only_hint: Some(true),
                open_world_hint: Some(true),
                ..Default::default()
            }),
            output_schema: None,
            title: Some("Fetch URL".into()),
        }],
    })
}

pub(crate) fn call_tool(input: CallToolRequest) -> Result<CallToolResult> {
    match input.request.name.as_str() {
        "fetch" => fetch(input),
        _ => Ok(CallToolResult::error(format!(
            "Unknown tool: {}",
            input.request.name
        ))),
    }
}

// ---------------------------------------------------------------------------
// Unimplemented handlers (default stubs required by the V2 plugin interface)
// ---------------------------------------------------------------------------

pub(crate) fn complete(_input: CompleteRequest) -> Result<CompleteResult> {
    Ok(CompleteResult::default())
}

pub(crate) fn get_prompt(_input: GetPromptRequest) -> Result<GetPromptResult> {
    Err(anyhow!("get_prompt not implemented"))
}

pub(crate) fn list_prompts(_input: ListPromptsRequest) -> Result<ListPromptsResult> {
    Ok(ListPromptsResult::default())
}

pub(crate) fn list_resource_templates(
    _input: ListResourceTemplatesRequest,
) -> Result<ListResourceTemplatesResult> {
    Ok(ListResourceTemplatesResult::default())
}

pub(crate) fn list_resources(_input: ListResourcesRequest) -> Result<ListResourcesResult> {
    Ok(ListResourcesResult::default())
}

pub(crate) fn on_roots_list_changed(_input: PluginNotificationContext) -> Result<()> {
    Ok(())
}

pub(crate) fn read_resource(_input: ReadResourceRequest) -> Result<ReadResourceResult> {
    Err(anyhow!("read_resource not implemented"))
}
