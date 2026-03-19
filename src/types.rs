use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Arguments for the `fetch` tool that converts a web page to Markdown.
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub(crate) struct FetchArguments {
    #[schemars(
        description = "The URL to fetch and convert to Markdown. Must be an http:// or https:// URL."
    )]
    pub url: String,
}
