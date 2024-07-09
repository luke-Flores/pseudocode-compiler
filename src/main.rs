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
use std::fs;
use std::env;
//use cbpcc::parts::Compiler;
//use cbpcc::parts::checker::check_code;

fn main(){
    let mut argv = env::args();
    let argc = argv.len();

    if argc == 1{
        eprintln!("expected an option, please provide input. For help do `cbpcc --help`");
    }
    else if argc == 2{
       let test = match fs::read_to_string(argv.nth(1).unwrap()){
            Ok(n) => compile_code(n),
            Err(e) => {
                eprintln!("problem opening or reading file, rust error msg: {}", e);
                return;
            },
        };

    }
}

fn compile_code(input: String){
    let mut program_code = cbpcc::parts::Compiler{
        input,
        tokens: Vec::new(),
        var_list: Vec::new(),
    };
    program_code.tokenize();
    println!("{:#?}", program_code.tokens);
    //program_code.check_code();

}
