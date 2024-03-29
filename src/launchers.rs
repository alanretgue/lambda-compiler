use std::collections::HashMap;
use std::io::ErrorKind;

use crate::{ast, parser, binding::Binding};

pub fn launch_pretty_print(str: &str) -> Result<String, (ErrorKind, String)>{
    let parsed = launch_parser(&str)?;
    Ok(format!("{:?}", parsed))
}

pub fn launch_binding(
    str: &str,
    hashmap: &mut Vec<HashMap<String, Box<ast::Expr>>>
    ) -> Result<Vec<HashMap<String, Box<ast::Expr>>>, (ErrorKind, String)> {

    let parsed = launch_parser(&str)?;
    
    Ok((*parsed).bind(hashmap))
}

pub fn launch_parser(str: &str) -> Result<Box<ast::Expr>, (ErrorKind, String)> {
    let mut errors = Vec::new();
    let parsed = parser::StatParser::new().parse(&mut errors, str);

    if errors.len() != 0 {
        return Err((ErrorKind::InvalidData, "An error occured during parsing".to_owned()));
    }
    return Ok(parsed.unwrap());
}

pub fn display_pretty_print(str: String) -> Result<u8, (ErrorKind, String)> {
    println!("{:?}", launch_pretty_print(&str)?);
    Ok(0)
}

pub fn display_binding(str: String) -> Result<u8, (ErrorKind, String)> {
    let mut binding_map: Vec<HashMap<String, Box<ast::Expr>>> = Vec::new();
    println!("{:?}", launch_binding(&str, &mut binding_map)?);
    Ok(0)
}
