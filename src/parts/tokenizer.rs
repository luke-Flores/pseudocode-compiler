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
#[derive(Debug, PartialEq, Copy, Clone)]
enum TokenIds{
    FunctionName,
    FunctionDef,
    FunctionBeg,
    FunctionEnd,
    Operand,
    ParamSeperator,
    ParanBeg,
    ParanEnd,
    BlockBeg,
    BlockEnd,
    If,
    Else,
    Times,
    Until,
    Asigment,
    ArrayBeg,
    ArrayEnd,
    IndBeg,
    IndEnd,
    ElemSeperator,
    VarDec,
    VarName,
    For,
    Each,
    In,
    Procedure,
    Return,
    StringBeg,
    StringEnd,
    Stringval,
    Terminator,
    Num,
}


#[derive(Debug)]
pub struct Token{
    id: TokenIds,
    value: String,
}

enum ParaBracketValues{
    ooo,
    function,
    array_def,
    array_index,
}

struct DevelopingTokens{
    stream: Vec<Token>,
    // i would off load variables and functions to codegen time but variable declarations needs to happen during tokenization
    // these are  types of String because using &str was a pain
    vars: Vec<String>,
    temp: Vec<String>,
    funcs: Vec<String>,
    //flags
    //each element represents a paranthese or bracket and its corresponding value
    bracket_para: Vec<ParaBracketValues>,
    in_string: bool,
    untokenized: u16,
    neg_num: bool,
    skip: u8,
    block_count: u16,
}

impl DevelopingTokens{
    fn match_tokens(&mut self, input: &Vec<&str>){
        'tokenloop : for (i, val) in input.iter().enumerate(){
            if self.skip != 0{
                // reset neg_num
                self.neg_num=false;
                self.skip-=1;
                continue;
            }
            else if self.in_string{
                if val == &"\""{
                    self.in_string = false;
                    let mut string: String = String::new();
                    for temp_string in self.temp.iter(){
                        string.push_str(temp_string);
                    }
                    self.stream.push(Token{
                        id: TokenIds::Stringval,
                        value: string,
                    });
                    self.stream.push(Token{
                        id: TokenIds::StringEnd,
                        value: "\"".to_string(),
                    });


                    self.temp = Vec::new();
                }
                else{
                    if self.temp.len() != 0 && self.temp[self.temp.len() -1] != " "{
                        self.temp.push(" ".to_string());
                    }
                    self.temp.push(val.to_string());
                }
                continue 'tokenloop;
            }
            match val {
                &"(" => {
                    let mut is_func = false;
                    if i != 0{
                        for func in self.funcs.iter(){
                            if input[i-1] == func{
                                is_func = true;
                                break;
                            }
                        }
                        if is_func{
                            self.stream.push(Token{
                                id: TokenIds::FunctionName,
                                value: input[i-1].to_string(),
                            });
                            self.stream.push(Token{
                                id: TokenIds::FunctionBeg,
                                value: "(".to_string(),
                            });
                            self.bracket_para.push(ParaBracketValues::function);
                            self.untokenized-=1;
                        }
                    }
                    if !is_func{
                        self.stream.push(Token{
                            id: TokenIds::ParanBeg,
                            value: "(".to_string(),
                        });
                        self.bracket_para.push(ParaBracketValues::ooo);
                    }
                }
                &"[" => {
                    let mut is_index = false;
                    if self.stream.len() > 0{
                        if self.stream[self.stream.len()-1].id == TokenIds::VarName{
                            is_index = true;
                        }
                    }
                    if is_index{
                        self.stream.push(Token{
                            id: TokenIds::IndBeg,
                            value: "[".to_string(),
                        });
                        self.bracket_para.push(ParaBracketValues::array_index);
                    }
                    else{
                        self.stream.push(Token{
                            id: TokenIds::ArrayBeg,
                            value: "[".to_string(),
                        });
                        self.bracket_para.push(ParaBracketValues::array_def);
                    }
                }
                &"]" => {
                    if self.bracket_para.len() > 0{
                        match self.bracket_para[self.bracket_para.len()-1]{
                            ParaBracketValues::array_def => {
                                self.stream.push(Token{
                                    id: TokenIds::ArrayEnd,
                                    value: "]".to_string(),
                                });
                                self.bracket_para.pop();
                            }
                            ParaBracketValues::array_index => {
                                self.stream.push(Token{
                                    id: TokenIds::IndEnd,
                                    value: "]".to_string(),
                                });
                                self.bracket_para.pop();
                            }
                            _ => panic!("] appeared while an unclosed paranthese is still present"),
                        }
                    }
                    else{
                        panic!("] appeared without a corresponding [ to go with it");
                    }
                }
                &"\"" => {
                    self.stream.push(Token{
                        id: TokenIds::StringBeg,
                        value: "\"".to_string(),
                    });
                    self.in_string = true;
                }
                &"IF" => {
                    self.stream.push(Token{
                        id:TokenIds::If,
                        value: "IF".to_string(),
                    });
                }
                &"ELSE" => {
                    self.stream.push(Token{
                        id: TokenIds::Else,
                        value: "ELSE".to_string(),
                    });
                }
                &"REPEAT" => {
                    let mut ok = false;
                    if input.len() > i+2{
                        if input[i+1].parse::<f64>().is_ok(){
                            if input[i+2] == "TIMES"{
                                self.stream.push(Token{
                                    id: TokenIds::Times,
                                    value: "TIMES".to_string(),
                                });
                                self.stream.push(Token{
                                    id: TokenIds::Num,
                                    value: input[i+1].to_string(),
                                });
                                ok = true;
                                self.skip+=2;
                            }
                            else{
                                panic!("REPEAT was followed by a number but TIMES did not follow");
                            }
                        }
                    }
                    if input.len() > i+1{
                        if input[i+1] == "UNTIL"{
                            self.stream.push(Token{
                                id: TokenIds::Until,
                                value: "UNTIL".to_string(),
                            });
                            self.skip+=1;
                            ok = true;
                        }
                    }
                    if !ok{
                        panic!("REPEAT found but was followed by neither a number or the keyword UNTIL");
                    }
                }
                &"{" =>{
                    self.stream.push(Token{
                        id: TokenIds::BlockBeg,
                        value: "{".to_string(),
                    });
                    self.block_count+=1;
                }
                &"}" => {
                    if self.block_count > 0{
                        self.stream.push(Token{
                            id: TokenIds::BlockEnd,
                            value: "}".to_string(),
                        });
                        self.block_count-=1;
                    }
                    else{
                        panic!("`}}` occured without a corresponding `{{`");
                    }
                }
                &")" => {
                    if self.bracket_para.len() > 0{
                        match self.bracket_para[self.bracket_para.len() -1]{
                            ParaBracketValues::ooo =>{
                                self.stream.push(Token{
                                    id: TokenIds::ParanEnd,
                                    value: ")".to_string(),
                                });
                                self.bracket_para.pop();
                            },
                            ParaBracketValues::function =>{
                                self.stream.push(Token{
                                    id: TokenIds::FunctionEnd,
                                    value: ")".to_string(),
                                });
                                self.bracket_para.pop();
                            },
                            _ => panic!(") occured in side of an unclosed array or array index"),
                        }
                    }
                    else{
                        panic!("Paranthese closed without a corresponding opening paranthese");
                    }
                }
                &"<-" => {
                    if i > 0{
                        self.stream.push(Token{
                            id: TokenIds::VarDec,
                            value: input[i-1].to_string(),
                        });
                        self.stream.push(Token{
                            id: TokenIds::Asigment,
                            value: "<-".to_string(),
                        });
                        self.untokenized-=1;
                        self.vars.push(input[i-1].to_string());
                    }
                    else {
                        panic!("<- used without a variable name");
                    }
                }
                &"-" =>{
                    if self.stream.len() > 0{
                        let prev_token = self.stream[self.stream.len()-1].id;
                        if prev_token == TokenIds::Num || prev_token == TokenIds::VarName{
                            self.stream.push(Token{
                                id: TokenIds::Operand,
                                value: "-".to_string(),
                            });
                        }
                        else{
                            self.neg_num = true;
                            continue 'tokenloop;
                        }
                    }
                    else{
                        self.neg_num = true;
                        continue 'tokenloop;
                    }
                }
                &"+" | &"*" | &"/" | &"MOD" | &"=" |  &"OR" | &"AND" |  &"NOT" => {
                    self.stream.push(Token{
                        id: TokenIds::Operand,
                        value: val.to_string(),
                    });
                }
                &"!" => {
                    match input.iter().nth(i+1){
                        Some(&"=") =>{
                            self.stream.push(Token{
                                id: TokenIds::Operand,
                                value: "!=".to_string(),
                            });
                            self.skip+=1;
                        },
                        _ => panic!("Expected = after ! but did not get any value or a different value"),
                    }
                }
                &"," => {
                    if self.bracket_para.len() > 0{
                        match self.bracket_para[self.bracket_para.len()-1]{
                            ParaBracketValues::function => {
                                self.stream.push(Token{
                                    id: TokenIds::ParamSeperator,
                                    value: ",".to_string(),
                                });
                            },
                            ParaBracketValues::array_def => {
                                self.stream.push(Token{
                                    id: TokenIds::ElemSeperator,
                                    value: ",".to_string(),
                                });
                            },
                            _ => {
                                panic!("Comma appeared while not in a function parameter statement");
                            },
                        }
                    }
                    else {
                        panic!("comma appeared while not in a function parameter statement");
                    }
                }
                &">" | &"<" => {
                    match input.iter().nth(i+1){
                        Some(&"=") =>{
                            self.stream.push(Token{
                                id: TokenIds::Operand,
                                value: format!("{}=",val),
                            });
                            self.skip+=1;
                        },
                        _ => {
                            self.stream.push(Token{
                                id: TokenIds::Operand,
                                value: val.to_string(),
                            });
                        },
                    }
                }
                &"\n" => {
                    if self.untokenized != 0{
                        panic!("Failed to tokenize a statement.");
                    }
                    else if self.bracket_para.len() > 0{
                        panic!("Terminator apeared inside of a paranthese or bracket statement");
                    }
                    else{
                        // dont't push a terminator if the previous token was a terminator or if its
                        // the first element just so its easier down the road
                        if self.stream.len() > 0{
                            if self.stream[self.stream.len()-1].id != TokenIds::Terminator{
                                self.stream.push(Token{
                                    id: TokenIds::Terminator,
                                    value: "\n".to_string(),
                                });
                            }
                        }

                    }
                }
                _ =>{
                    //dont count white space
                    if val == &" " || val == &"\t"{
                        continue 'tokenloop;
                    }
                    //check if its a number
                    else if val.parse::<f64>().is_ok(){
                        let mut number: String = val.to_string();
                        if self.neg_num{
                            number.insert(0, '-');
                        }
                        self.stream.push(Token{
                            id: TokenIds::Num,
                            value: number,
                        });
                        continue 'tokenloop;
                    }
                    // check if the val is equal to a variable name
                    for var in self.vars.iter(){
                        if val == var{
                            self.stream.push(Token{
                                id: TokenIds::VarName,
                                value: val.to_string(),
                            });
                            self.neg_num = false;
                            continue 'tokenloop;

                        }
                    }
                    self.untokenized+=1;
                }
            }
            // reset the neg_num flag so that every number doesnt become negative
            self.neg_num = false;
        }
        if self.block_count > 0{
            panic!("File ended but there are still unclosed block openers({{)");
        }
    }
}

fn preproccess(preproccessed: &mut String, input: String){
    let mut chars = input.chars();
    let mut i = 0;
    while let Some(letter) = chars.next(){
        match letter {
            // split string at these values plus space
            ')' | '('| '<' | '\"' | '\n' | '>' | '-'  | '*' | '+' | '/' | '=' | '!' | ',' | '}' | '{' | '[' | ']'=> {
                //prevent useless empty strings forming
                if i > 0{
                    if preproccessed.chars().nth(preproccessed.len()-1).unwrap() != ' '{
                        preproccessed.push(' ');
                    }
                }
                preproccessed.push(letter);
                // if it is <- then concatenate it
                if letter == '<'{
                    match input.chars().nth(i+1){
                        Some('-') => {
                            preproccessed.push('-');
                            // skip over the '-'
                            i+=1;
                            chars.next();
                        },
                        _ => (),
                    }
                }
                preproccessed.push(' ');
            },
            ' ' => {
                if i > 0{
                    if preproccessed.chars().nth(preproccessed.len()-1).unwrap() != ' '{
                        preproccessed.push(' ');
                    }
                }
            }
            _ => preproccessed.push(letter),
        }
        i+=1;
    }

}

fn replace_empty_strings(preproccessed: &mut Vec<&str>){
    let mut i = 0;
    while i< preproccessed.len(){
        if preproccessed[i] == ""{
            preproccessed[i]=" ";
        }
        else{
            i+=1;
        }
    }
}

pub fn tokenize(input: String) -> Vec<Token>{
    // make mainpulating easier
    let mut preproccessed: String = String::with_capacity(input.len());
    preproccess(&mut preproccessed, input.clone());

    let mut preproccessed: Vec<&str> = preproccessed.split(' ').collect();

    //replace empty strings so they don't cause problems
    replace_empty_strings(&mut preproccessed);

    let mut res = DevelopingTokens{
        stream: Vec::new(),
        bracket_para: Vec::new(),
        in_string: false,
        temp: Vec::new(),
        untokenized: 0,
        neg_num: false,
        vars: Vec::new(),
        funcs: vec!["DISPLAY".to_string(), "INPUT".to_string(), "RANDOM".to_string(), "INSERT".to_string()],
        skip: 0,
        block_count: 0,
    };
    res.match_tokens(&preproccessed);
    return res.stream;
}
