mod parse;
mod types;

use tower_lsp::LanguageServer;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    DidChangeTextDocumentParams, DidOpenTextDocumentParams, InitializeParams, InitializeResult,
};

struct Backend {
    vars: Vec<String>,
    types: Vec<String>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        todo!()
    }

    async fn shutdown(&self) -> Result<()> {
        todo!()
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        todo!()
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        todo!()
    }
}

fn main() {}
