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
pub mod tokenizer;
pub mod checker;
use colored::Colorize;

pub struct Compiler{
    pub input: String,
    pub tokens: Vec<tokenizer::Token>,
}

impl Compiler{
    fn say_error(&mut self, msg: &str, line_num: usize){
        eprintln!("{}", msg);
        let lines: Vec<&str> = self.input.split('\n').collect();
        if line_num < lines.len(){
            eprintln!("error occured on line number {}: {}", line_num+1, lines[line_num].red());
            panic!();
        }
        else{
            panic!("line number panic occured on is greater than the number of lines. This is likely an error on the compiler\'s behalf please report this.");
        }
    }

}
