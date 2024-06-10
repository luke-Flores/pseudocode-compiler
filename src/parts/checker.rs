/*
    pseudocode compiler - an implementation of the CSP pseudocode language
    Copyright (C) 2024  Luke Flores

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
/*
    The goal of this is to check the code for errors and ensure that at codegen that no errors will occur
    it will also return the list of variables and functions with their types
*/
use crate::parts::tokenizer;
//the three different basic types not including lists
pub enum TypeVariant{
    Num,
    String,
    Bool,
}

//what a type can be attached too
pub enum DataDesc{
    Funcion,
    Variable,
}

//displays the line the error occurs on and then says an error message
fn say_error(msg: &str, line_num: usize){
    eprintln!("error occured on line {}:", line_num);

}

//Structure describing a type
pub struct Type{
    typedesc: TypeVariant,
    //how many leveles of nesting in an array
    // 0 indicates single variable, 1 indicates a 1 dimensional array
    nested_level: u16,
    data_desc: DataDesc,
}

fn ensure_single_statements(stream: &Vec<tokenizer::Token>){
    let mut excess_statements: u32 = 0;
    let mut line_num: usize = 0;
    for token in stream.iter(){
        match token.id{
            tokenizer::TokenIds::Terminator=> {
                line_num+=1;
            }
            _ => excess_statements+=1,
        }
    }
}

pub fn check_code(stream: &Vec<tokenizer::Token>) -> Vec<Type>{
    let mut types: Vec<Type> = Vec::new();


    return types;
}
