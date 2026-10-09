use std::fs::read_dir;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

use crate::{Backend};
use crate::types::{Type, Variable};

#[derive(Clone)]
pub struct ScopeData {
    pub scope: PathBuf,
    pub vars: Vec<Variable>,
    pub types: Vec<Type>,
}

impl ScopeData {
    fn new(scope: PathBuf) -> Self {
        Self {
            scope,
            vars: vec![],
            types: vec![],
        }
    }
}

fn map_project(path: PathBuf, scopes: &mut Vec<ScopeData>) -> Result<()> {
    if !path.is_dir() {
        return Ok(());
    }
    let mut cur_scope: ScopeData = ScopeData::new(path.clone());
    for item_res in read_dir(path)? {
        let item = item_res?;
        let item_path = item.path();

        if item_path.is_dir() {
            map_project(item_path, scopes)?;
        } else {
            let file_str = match item.file_name().into_string() {
                Ok(s) => s,
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Could not convert filename to string.",
                    ));
                }
            };

            if file_str.ends_with(".typ") {
                println!("found {:?}", item_path);
                cur_scope.types.append(&mut parse_typ());
            }
            if file_str.ends_with(".var") {
                println!("found {:?}", item_path);
                cur_scope.vars.append(&mut parse_var())
            }
        }
    }
    scopes.push(cur_scope);
    Ok(())
}

pub fn parse(path: PathBuf) -> Result<Vec<ScopeData>> {
    let mut scopes: Vec<ScopeData> = Vec::with_capacity(512);

    map_project(path, &mut scopes)?;

    Ok(scopes)
}

fn parse_typ() -> Vec<Type> {
    vec![]
}

fn parse_var() -> Vec<Variable> {
    vec![]
}