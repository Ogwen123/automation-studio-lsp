mod parse;
mod scope;
mod types;

use crate::parse::parse;
use crate::types::{Type, Variable};
use dashmap::DashMap;
use std::path::PathBuf;
use std::sync::RwLock;
use tower_lsp::jsonrpc::Error as JRPCError;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    CompletionOptions, DidChangeTextDocumentParams, DidChangeWatchedFilesParams,
    DidChangeWatchedFilesRegistrationOptions, DidOpenTextDocumentParams, FileSystemWatcher,
    GlobPattern, InitializeParams, InitializeResult, InitializedParams, Registration,
    ServerCapabilities,
};
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct Backend {
    client: Client,
    root: RwLock<Option<PathBuf>>,
    vars: DashMap<PathBuf, Vec<Variable>>,
    types: DashMap<PathBuf, Vec<Type>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let root = match params.root_uri {
            Some(uri) => match uri.to_file_path() {
                Ok(path) => path,
                Err(_) => {
                    eprintln!("Could not parse root URI to PathBuf");
                    return Err(JRPCError::internal_error());
                }
            },
            None => {
                eprintln!("Could not parse root URI to PathBuf");
                return Err(JRPCError::invalid_params("No root URI provided."));
            }
        };

        *self.root.write().unwrap() = Some(root);

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                completion_provider: Some(CompletionOptions::default()),

                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.index().await;

        let opts = DidChangeWatchedFilesRegistrationOptions {
            watchers: vec![
                FileSystemWatcher {
                    glob_pattern: GlobPattern::String("**/*.typ".into()),
                    kind: None,
                },
                FileSystemWatcher {
                    glob_pattern: GlobPattern::String("**/*.var".into()),
                    kind: None,
                },
            ],
        };
        let _ = self
            .client
            .register_capability(vec![Registration {
                id: "br-watch".into(),
                method: "workspace/didChangeWatchedFiles".into(),
                register_options: Some(serde_json::to_value(opts).unwrap()),
            }])
            .await;
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

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        todo!()
    }
}

impl Backend {
    // Walk the project to discover new files and parse them
    async fn index(&self) {
        let scopes = match parse(PathBuf::from("./test_code")) {
            Ok(res) => res,
            Err(err) => panic!("Could not parse .typ and .var files ({})", err.to_string()),
        };

        for scope in scopes {
            self.types.insert(scope.scope.clone(), scope.types);
            self.vars.insert(scope.scope, scope.vars);
        }
    }

    // Go through every file in the index and reparse them
    async fn update_index(&self) {}

    // Reparse the provided file, nothing will happen if the file is not in the index
    async fn update_scope(&self, scope: PathBuf) {}

    // Parse a new scope that has not been seen yet
    async fn index_scope(&self, scope: PathBuf) {}
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend {
        client,
        root: RwLock::new(None),
        vars: DashMap::new(),
        types: DashMap::new(),
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
