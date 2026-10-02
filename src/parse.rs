use std::cell::RefCell;
use std::fs::read_dir;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

use crate::Backend;

#[derive(Clone)]
struct Scope {
    scope: String,
    vars: Vec<String>,
    types: Vec<String>,
    children: Vec<RefCell<Scope>>,
}

impl Scope {
    fn new(scope: String) -> Self {
        Self {
            scope,
            vars: vec![],
            types: vec![],
            children: vec![],
        }
    }
}

fn map_project(path: &Path, scope: &RefCell<Scope>) -> Result<()> {
    if !path.is_dir() {
        return Ok(());
    }

    for item_res in read_dir(path)? {
        let item = item_res?;
        let item_path = item.path();

        if item_path.is_dir() {
            let child = RefCell::new(Scope::new(item_path.to_string_lossy().into_owned()));

            map_project(&item_path, &child)?;

            scope.borrow_mut().children.push(child);
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
                scope.borrow_mut().types.push(parse_typ())
            }
            if file_str.ends_with(".var") {
                scope.borrow_mut().types.push(parse_var())
            }
        }
    }

    Ok(())
}

pub fn parse(path: String, backend: &mut Backend) {
    let mut top_scope = Scope {
        scope: String::from("/"),
        vars: vec![],
        types: vec![],
        children: vec![],
    };

    map_project(&Path::new(&path.as_str()), &RefCell::new(top_scope));
}

fn parse_typ() -> String {
    String::new()
}

fn parse_var() -> String {
    String::new()
}
