use std::fs;
use std::fs::read_dir;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

use crate::Backend;
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

fn get_str_data(item_path: PathBuf) -> Result<String> {
    let data = fs::read(item_path)?;
    
    // TODO: add support for windows-1252
    let str_data = match String::from_utf8(data) {
        Ok(res) => res,
        Err(err) => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Could not parse file content into a String. ({})", err),
            ));
        }
    };

    Ok(str_data)
}

fn bufcmp<T>(buf: &[char], other: T) -> bool where T: ToString {
    let s = other.to_string();

    buf.iter().copied().eq(s.chars())
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
                cur_scope.types.append(&mut parse_typ(get_str_data(item_path)?));
            } else if file_str.ends_with(".var") {
                println!("found {:?}", item_path);
                cur_scope.vars.append(&mut parse_var(get_str_data(item_path)?))
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

fn parse_typ(content: String) -> Vec<Type> {
    let mut entered_type_defs: bool = false;
    let mut in_type_def: bool = false;

    let mut buf: Vec<char> = Vec::new();
    for char in content.chars() {
        if char == ' ' || char == '\n' { buf.clear() }

        buf.push(char);

        if !entered_type_defs {
            if bufcmp(&buf, "TYPE") {
                println!("entering type");
                entered_type_defs = true;
            }
        } else {
            if bufcmp(&buf, "END_TYPE") {
                println!("exiting type");
                break;
            }
        }
    }

    vec![]
}

fn parse_var(file: String) -> Vec<Variable> {
    vec![]
}
