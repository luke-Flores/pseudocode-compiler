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

use super::Compiler;
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

//Structure describing a type
pub struct Type{
    typedesc: TypeVariant,
    //how many leveles of nesting in an array
    // 0 indicates single variable, 1 indicates a 1 dimensional array
    nested_level: u16,
    data_desc: DataDesc,
}

impl Compiler{
    pub fn check_code(&mut self) {
       // self.check_data();
    }
    //asign variable and function types and ensure that variables are used at proper scopes
    /*
    fn check_data(&mut self){
        let mut var_list: Vec<((String, usize), Type)> = Vec::new();
        // keeps list of vars that would be in scope
        let mut active_vars: Vec<String> = Vec::new();
        //keeps list of how many variables are in each scope
        let mut vars_in_scope: Vec<u16> = vec![0];
        //keep track of vars_in_scope length as it would violate borrow checker rules to use .len()
        let mut num_scopes: usize = 0;
        //for errors
        let mut line_num: usize = 0;
        let mut iter = self.tokens.iter().enumerate();
        for (i, token) in iter{
            match token.id{
                tokenizer::TokenIds::VarDec => {
                    let mut new_var = true;
                    'varloop : for var in active_vars.iter(){
                        if var == &self.tokens[i].value{
                            new_var = false;
                            break 'varloop;
                        }
                    }
                    if new_var{
                        active_vars.push(self.tokens[i].value.clone());
                        vars_in_scope[num_scopes]+=1;
                        let mut typedesc : TypeVariant;
                        let mut nested_level: u16 = 0;
                        //currently doesn't work with expressions this is something needed to be
                        //fixed
                        'valueloop : for value in iter{
                            match value.1.id{
                                tokenizer::TokenIds::ArrayBeg => nested_level+=1,
                                tokenizer::TokenIds::Num => {
                                    typedesc = TypeVariant::Num;
                                    break 'valueloop;
                                },
                                tokenizer::TokenIds::StringBeg => {
                                    typedesc = TypeVariant::String;
                                    break 'valueloop;
                                },
                                tokenizer::TokenIds::Bool => {
                                    typedesc = TypeVariant::Bool;
                                    break 'valueloop;
                                },
                                _ => self.say_error("unexpected value appeared after variable declaration", line_num),
                            }
                        }

                        var_list.push(((self.tokens[i].value.clone(), line_num), Type{
                            typedesc,
                            nested_level,
                            data_desc: DataDesc::Variable,
                        }));
                    }
                },
                tokenizer::TokenIds::BlockBeg => {
                    vars_in_scope.push(0);
                    num_scopes+=1;
                }
                tokenizer::TokenIds::BlockEnd => {
                    for j in 0..vars_in_scope[num_scopes]{
                        active_vars.pop();
                    }
                    vars_in_scope.pop();
                    num_scopes-=1;
                }
                tokenizer::TokenIds::Terminator => {
                    line_num+=1;
                }
                tokenizer::TokenIds::VarName => {
                    let mut is_in_scope = false;
                    'varloop : for var in active_vars.iter(){
                        if var == &self.tokens[i].value{
                            is_in_scope = true;
                            break 'varloop;
                        }
                    }
                    if !is_in_scope{
                        self.say_error(&format!("variable: {}, was declared in a previous unusable scope and is undeclared in a usable scope", self.tokens[i].value)[..], line_num);
                    }
                }
                _ => (),
            }
        }
        self.var_list = var_list;
    }
    */
}
