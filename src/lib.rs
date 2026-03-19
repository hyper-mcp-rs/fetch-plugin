mod pdk;
mod types;

use crate::{
    pdk::{http::http_request_with_retry, types::*},
    types::*,
};
use anyhow::{Result, anyhow};
use extism_pdk::*;
use htmd::HtmlToMarkdown;
use schemars::schema_for;
use serde_json::Value;

fn fetch(input: CallToolRequest) -> Result<CallToolResult> {
    let args: FetchArguments =
        match serde_json::from_value(Value::Object(input.request.arguments.unwrap_or_default())) {
            Ok(a) => a,
            Err(e) => return Ok(CallToolResult::error(format!("Invalid arguments: {e}"))),
        };

    // Create HTTP request
    let req = HttpRequest::new(args.url)
        .with_header("User-Agent", "fetch-tool/1.0")
        .with_method("GET");

    // Perform the request
    let res = match http_request_with_retry(&req) {
        Ok(res) => res,
        Err(e) => return Ok(CallToolResult::error(format!("Failed to get HTML: {e}"))),
    };

    // Convert response body to string
    let body = res.body();
    let html = String::from_utf8_lossy(body.as_slice());

    let converter = HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style"])
        .build();

    // Convert HTML to markdown
    Ok(match converter.convert(&html) {
        Ok(markdown) => CallToolResult {
            content: vec![ContentBlock::Text(TextContent {
                text: markdown,

                ..Default::default()
            })],
            ..Default::default()
        },
        Err(e) => CallToolResult::error(format!("Failed to convert HTML to markdown: {e}")),
    })
}

// ---------------------------------------------------------------------------
// Implemented handlers
// ---------------------------------------------------------------------------

pub(crate) fn list_tools(_input: ListToolsRequest) -> Result<ListToolsResult> {
    Ok(ListToolsResult {
        tools: vec![Tool {
            name: "fetch".into(),
            description: Some(
                "Fetches the contents of a URL and returns its contents converted to markdown"
                    .into(),
            ),
            input_schema: schema_for!(FetchArguments),
            annotations: Some(ToolAnnotations {
                read_only_hint: Some(true),
                open_world_hint: Some(true),
                ..Default::default()
            }),
            title: Some("Fetch URL".into()),

            ..Default::default()
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
