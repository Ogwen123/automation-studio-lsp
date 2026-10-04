mod parse;
mod scope;
mod types;

use crate::parse::parse;
use crate::types::{Type, Variable};
use dashmap::DashMap;
use std::path::{Path, PathBuf};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    CompletionOptions, DidChangeTextDocumentParams, DidOpenTextDocumentParams,
    HoverProviderCapability, InitializeParams, InitializeResult, ServerCapabilities,
};
use tower_lsp::{Client, LanguageServer, LspService, Server};

type Scope = Box<Path>;

struct Backend {
    client: Client,
    vars: DashMap<Scope, Vec<Variable>>,
    types: DashMap<Scope, Vec<Type>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        todo!()
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| {
        let mut backend = Backend {
            client,
            vars: DashMap::new(),
            types: DashMap::new(),
        };

        match parse(PathBuf::from("./test_code"), &mut backend) {
            Ok(_) => {},
            Err(err) => panic!("Could not parse .typ and .var files ({})", err.to_string())
        };

        backend
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
