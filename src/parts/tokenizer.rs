/*
    CBPCC - an implementation of the CSP pseudocode language
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
    FunctionBeg,
    FunctionEnd,
    FunctionName,
    Operand,
    ParamSeperator,
    MonoOperand,
    BlockBeg,
    BlockEnd,
    If,
    Else,
    Repeat,
    Times,
    Until,
    Asigment,
    ArrayBeg,
    ArrayEnd,
    Data,
    VarDec,
    VarName,
    ArrayName,
    For,
    Each,
    In,
    FunctionDef,
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

struct DevelopingTokens{
    stream: Vec<Token>,
    // i would off load variables to codegen time but variable declarations needs to happen during
    // tokenization
    vars: Vec<String>,
    temp: Vec<String>,
    //flags
    function_parameters: bool,
    in_string: bool,
    untokenized: u16,
    neg_num: bool,
}

impl DevelopingTokens{
    fn match_tokens(&mut self, input: &Vec<&str>){
        'tokenloop : for (i, val) in input.iter().enumerate(){
            if self.in_string{
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
            }
            else if val == &"(" {
                if i != 0{
                    self.stream.push(Token{
                        id: TokenIds::FunctionName,
                        value: input[i-1].to_string(),
                    });
                    self.stream.push(Token{
                        id: TokenIds::FunctionBeg,
                        value: "(".to_string(),
                    });
                    self.function_parameters = true;
                    self.untokenized-=1;
                }
                else {
                    panic!("Function called without name, aborting");
                }
            }
            else if val == &"\""{
                self.stream.push(Token{
                    id: TokenIds::StringBeg,
                    value: "\"".to_string(),
                });
                self.in_string = true;
            }

            else  if val == &")"{
                self.stream.push(Token{
                    id: TokenIds::FunctionEnd,
                    value: ")".to_string(),
                });
                self.function_parameters = false;
            }
            else if val == &"<"{
                if i < input.len()-2{
                    if i > 0{
                        if input[i+1] == "-"{
                            self.stream.push(Token{
                                id: TokenIds::VarDec,
                                value: input[i-1].to_string(),
                            });
                            self.vars.push(input[i-1].to_string());
                            self.stream.push(Token{
                                id: TokenIds::Asigment,
                                value: "<-".to_string(),
                            });
                            // skip the '-'
                            input.iter().next();
                            self.untokenized-=1;
                        }
                    }
                    else {
                        panic!("<- operator occured without a variable");
                    }
                }
                else{
                    panic!("asignment operator is incomplete");
                }
            }
            else if val == &"-"{
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
            else if val == &"+" || val == &"*" || val == &"/" || val == &"MOD"{
                self.stream.push(Token{
                    id: TokenIds::Operand,
                    value: val.to_string(),
                });
            }
            else if val == &"\n"{
                if self.untokenized != 0{
                    panic!("Failed to tokenize a statement.");
                }
                else if self.function_parameters{
                    panic!("Terminator apeared inside of a function declaration");
                }
                else{
                    self.stream.push(Token{
                        id: TokenIds::Terminator,
                        value: "\n".to_string(),
                    });
                }
            }
            else if val.parse::<f64>().is_ok(){
                let mut number: String = val.to_string();
                if self.neg_num{
                    number.insert(0, '-');
                }
                self.stream.push(Token{
                    id: TokenIds::Num,
                    value: number,
                });
            }
            //dont count whitespace
            else if val != &" " && val != &"\t"{
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
            // reset the neg_num flag so that every number doesnt become negative
            self.neg_num = false;
        }
        // add a terminator at the end of every file
        self.stream.push(Token{
            id: TokenIds::Terminator,
            value: "".to_string(),
        });
    }
}

pub fn tokenize(input: String) -> Vec<Token>{
    // make mainpulating easier
    let mut preproccessed: String = String::with_capacity(input.len());
    for letter in input.chars() {
        match letter {
            // split string at these values plus space
            ')' | '(' | '\"' | '\n' | '>' | '<' | '-' | '*' | '+' | '/'=> {
                //prevent useless empty strings forming
                if preproccessed.len() > 0{
                    if preproccessed.chars().nth(preproccessed.len()-1).unwrap() != ' '{
                        preproccessed.push(' ');
                    }
                }
                preproccessed.push(letter);
                preproccessed.push(' ');
            },
            _ => preproccessed.push(letter),
        }
    }

    let mut preproccessed: Vec<&str> = preproccessed.split(' ').collect();

    //replace empty strings so they don't cause problems
    {
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

    let mut res = DevelopingTokens{
        stream: Vec::new(),
        function_parameters: false,
        in_string: false,
        temp: Vec::new(),
        untokenized: 0,
        neg_num: false,
        vars: Vec::new(),
    };
    res.match_tokens(&preproccessed);
    return res.stream;
}



