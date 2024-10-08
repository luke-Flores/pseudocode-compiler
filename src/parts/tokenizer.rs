use super::Compiler;

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
pub enum TokenIds{
    FunctionName,
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
    ForVar,
    Procedure,
    ProcBeg,
    ProcEnd,
    ParamName,
    Return,
    StringBeg,
    StringEnd,
    Stringval,
    Terminator,
    Bool,
    Num,
}


#[derive(Debug)]
pub struct Token{
    pub id: TokenIds,
    pub value: String,
}

enum ParaBracketValues{
    OOO,
    Function,
    ArrayDef,
    ArrayIndex,
}

struct DevelopingTokens{
    stream: Vec<Token>,
    // i would off load variables and functions to codegen time but variable declarations needs to happen during tokenization
    vars: Vec<Vec<String>>,
    funcs: Vec<String>,
    //flags
    //each element represents a paranthese or bracket and its corresponding value
    bracket_para: Vec<ParaBracketValues>,
    in_proc: bool,
    untokenized: u16,
    neg_num: bool,
    skip: u8,
    line_num: usize,
}

impl DevelopingTokens{
    fn match_tokens(&mut self, input: &Vec<String>) -> Option<(&str, usize)>{
        self.vars.push(Vec::new());
        'tokenloop : for (i, val) in input.iter().enumerate(){
            if self.skip != 0{
                // reset neg_num
                self.neg_num=false;
                self.skip-=1;
                continue;
            }
            else if self.in_proc{
                if val == &","{
                    //There is guarenteed a Procedure token and a procbeg token before
                    if self.stream[self.stream.len()-1].id == TokenIds::ParamName{
                        self.stream.push(Token{
                            id: TokenIds::ElemSeperator,
                            value: ",".to_string(),
                        });
                    }
                    else {
                        return Some(("\',\' appeared in a procedure defintion but was not precceeded by a parameter variable name", self.line_num));
                    }
                }
                else if val == &")"{
                    //guarenteed a token before it like procbeg
                    if self.stream[self.stream.len()-1].id != TokenIds::ElemSeperator{
                        self.stream.push(Token{
                            id: TokenIds::ProcEnd,
                            value: ")".to_string(),
                        });
                        self.in_proc = false;
                        'blockloop : for value in input[i+1..input.len()].iter(){
                            self.skip+=1;
                            // this is neccesary to start the function but keep parameters in the
                            // scope of the function and not declare them in an outer scope
                            if value == "{"{
                                self.stream.push(Token{
                                    id: TokenIds::BlockBeg,
                                    value: "{".to_string(),
                                });
                                break 'blockloop;
                            }
                            else if value == &"\n"{
                                self.line_num+=1;
                            }
                            else if value != &""{
                                return Some(("Procedure statement ended with a ) but a block opener ({) did not follow", self.line_num));
                            }
                        }
                    }
                    else{
                        return Some(("\',\' appeared but only a closing statement followed it after", self.line_num));
                    }
                }
                else {
                    self.stream.push(Token{
                        id: TokenIds::ParamName,
                        value: val.to_string(),
                    });
                    // !
                    self.vars.last_mut().unwrap().push(val.to_string())
                }
                continue 'tokenloop;
            }
            match &val[..] {
                "(" => {
                    if i > 0 && self.funcs.contains(&input[i-1]){
                        self.stream.push(Token{
                            id: TokenIds::FunctionName,
                            //already checked that i is greater than 0
                            value: input[i-1].to_string(),
                        });
                        self.stream.push(Token{
                            id: TokenIds::FunctionBeg,
                            value: "(".to_string(),
                        });
                        self.bracket_para.push(ParaBracketValues::Function);
                        //an untokenized function statement should have appeared
                        self.untokenized-=1;

                    }
                    else {
                        self.stream.push(Token{
                            id: TokenIds::ParanBeg,
                            value: "(".to_string(),
                        });
                        self.bracket_para.push(ParaBracketValues::OOO);
                    }
                }
                "[" => {
                    if self.stream.len() > 0 && (self.stream.last().unwrap().id == TokenIds::VarName || self.stream.last().unwrap().id == TokenIds::IndEnd){
                        self.stream.push(Token{
                            id: TokenIds::IndBeg,
                            value: "[".to_string(),
                        });
                        self.bracket_para.push(ParaBracketValues::ArrayIndex);
                    }
                    else{
                        self.stream.push(Token{
                            id: TokenIds::ArrayBeg,
                            value: "[".to_string(),
                        });
                        self.bracket_para.push(ParaBracketValues::ArrayDef);
                    }
                }
                "true" => {
                    self.stream.push(Token{
                        id: TokenIds::Bool,
                        value: "true".to_string(),
                    });
                }
                "false" => {
                    self.stream.push(Token{
                        id: TokenIds::Bool,
                        value: "false".to_string(),
                    });
                }
                "]" => {
                    match self.bracket_para.last(){
                        Some(ParaBracketValues::ArrayDef) => {
                            self.stream.push(Token{
                                id: TokenIds::ArrayEnd,
                                value: "]".to_string(),
                            });
                            self.bracket_para.pop();
                        }
                        Some(ParaBracketValues::ArrayIndex) => {
                            self.stream.push(Token{
                                id: TokenIds::IndEnd,
                                value: "]".to_string(),
                            });
                            self.bracket_para.pop();
                        }
                        _ => return Some(("] appeared while an unclosed paranthese is still present", self.line_num)),
                    }
                }
                "\"" => {
                    self.stream.push(Token{
                        id: TokenIds::StringBeg,
                        value: "\"".to_string(),
                    });
                    if input.len() > i+1{
                        self.stream.push(Token{
                            id: TokenIds::Stringval,
                            value: input[i+1].to_string(),
                        });
                        if input.len() > i+2{
                            if input[i+2] == "\""{
                                self.stream.push(Token{
                                    id: TokenIds::StringEnd,
                                    value: "\"".to_string(),
                                });
                            }
                            else{
                                return Some(("Somehow there was a non quotation mark character after the end of a string, please report this", self.line_num));
                            }
                        }
                        else{
                            return Some(("Quotation mark occured without a closing quotation mark", self.line_num));
                        }
                    }
                    else{
                        return Some(("Quotation mark occured without a corresponding one", self.line_num));
                    }
                    self.skip+=2;
                }
                "PROCEDURE" => {
                    if input.len() > i+2{
                        self.stream.push(Token{
                            id: TokenIds::Procedure,
                            value: input[i+1].to_string(),
                        });
                        self.funcs.push(input[i+1].to_string());
                        if input[i+2] == "("{
                            self.stream.push(Token{
                                id: TokenIds::ProcBeg,
                                value: "(".to_string(),
                            });
                        }
                        else{
                            return Some(("PROCEDURE keyword and a name for the procedure followed however a character other than a opening paranthese followed", self.line_num));
                        }
                        //push new block. This needs to be done now as the parameters follow a
                        //the stack as if they were declared after the block begining
                        self.vars.push(Vec::new());
                        //self.block_count+=1;
                        self.in_proc = true;
                        self.skip+=2
                    }
                    else{
                        return Some(("PROCEDURE keyword appeared but not enough words followed (try adding a function name and opening paranthese)", self.line_num));
                    }
                }
                "RETURN" => {
                    self.stream.push(Token{
                        id: TokenIds::Return,
                        value: "RETURN".to_string(),
                    });
                }
                "IF" => {
                    self.stream.push(Token{
                        id: TokenIds::If,
                        value: "IF".to_string(),
                    });
                }
                "ELSE" => {
                    self.stream.push(Token{
                        id: TokenIds::Else,
                        value: "ELSE".to_string(),
                    });
                }
                "FOR" => {
                    //4th string won't be checked here as it could be multiple things but something
                    //must be present so it is required
                    if input.len() > i+4{
                        self.skip+=3;
                        if input[i+1] == "EACH"{
                            self.stream.push(Token{
                                id: TokenIds::For,
                                value: "FOR EACH".to_string(),
                            });
                            self.stream.push(Token{
                                id: TokenIds::ForVar,
                                value: input[i+2].to_string(),
                            });
                            self.vars.last_mut().unwrap().push(input[i+2].to_string());
                            if input[i+3] != "IN"{
                                return Some(("FOR EACH statement appeared without IN following", self.line_num));
                            }
                        }
                        else{
                            return Some(("FOR appeared without a subsequent EACH following", self.line_num));
                        }
                    }
                    else{
                        return Some(("FOR appeared without enough strings after", self.line_num));
                    }
                }
                "REPEAT" => {
                    let mut ok = false;
                    if input.len() > i+2{
                        if input[i+2] == "TIMES"{
                            if input[i+1].parse::<f64>().is_ok() || self.vars.last().unwrap().contains(&input[i+1]){
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
                                return Some(("REPEAT n TIMES statement occured but n isn\'t a number or variable", self.line_num));
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
                        return Some(("REPEAT found but was followed by neither a number or the keyword UNTIL", self.line_num));
                    }
                }
                "{" =>{
                    self.stream.push(Token{
                        id: TokenIds::BlockBeg,
                        value: "{".to_string(),
                    });
                    //self.block_count+=1;
                    self.vars.push(Vec::new());
                }
                "}" => {
                    if self.vars.len() > 0{
                        self.stream.push(Token{
                            id: TokenIds::BlockEnd,
                            value: "}".to_string(),
                        });
                        self.vars.pop();
                    }
                    else{
                        return Some(("`}}` occured without a corresponding `{{`", self.line_num));
                    }
                }
                ")" => {
                    match self.bracket_para.last(){
                        Some(ParaBracketValues::OOO) =>{
                            self.stream.push(Token{
                                id: TokenIds::ParanEnd,
                                value: ")".to_string(),
                            });
                            self.bracket_para.pop();
                        },
                        Some(ParaBracketValues::Function) =>{
                            self.stream.push(Token{
                                id: TokenIds::FunctionEnd,
                                value: ")".to_string(),
                            });
                            self.bracket_para.pop();
                        },
                        _ => return Some((") occured inside of an unclosed array or array index", self.line_num)),
                    }
                }
                "<-" => {
                    if !(i > 0 && (self.vars.last().unwrap().contains(&input[i-1]))){
                        self.stream.push(Token{
                            id: TokenIds::VarDec,
                            value: input[i-1].to_string(),
                        });
                        //The previous statement should have been untokenized
                        self.untokenized-=1;
                        self.vars.last_mut().unwrap().push(input[i-1].to_string());
                    }
                    else if i == 0{
                        return Some(("<- used without a variable name", self.line_num));
                    }
                    self.stream.push(Token{
                        id: TokenIds::Asigment,
                        value: "<-".to_string(),
                    });

                }
                "-" =>{
                    if self.stream.len() > 0{
                        match self.stream.last().unwrap().id{
                            TokenIds::Num | TokenIds::VarName => {
                                self.stream.push(Token{
                                    id: TokenIds::Operand,
                                    value: "-".to_string(),
                                });
                            }
                            _ => {
                                self.neg_num = true;
                                continue 'tokenloop;
                            }
                        }
                    }
                    else{
                        self.neg_num = true;
                        continue 'tokenloop;
                    }
                }
                "+" | "*" | "/" | "MOD" | "=" |  "OR" | "AND" |  "NOT" => {
                    self.stream.push(Token{
                        id: TokenIds::Operand,
                        value: val.to_string(),
                    });
                }
                "!" => {
                    if input.get(i+1) == Some(&"=".to_string()){
                        self.stream.push(Token{
                            id: TokenIds::Operand,
                            value: "!=".to_string(),
                        });
                        self.skip+=1;
                    }
                    else{
                        return Some(("Expected \'=\' after an \'!\' but instead a different character or no character followed", self.line_num));
                    }
                }
                "," => {
                    match self.bracket_para.last(){
                        Some(ParaBracketValues::Function) => {
                            self.stream.push(Token{
                                id: TokenIds::ParamSeperator,
                                value: ",".to_string(),
                            });
                        },
                        Some(ParaBracketValues::ArrayDef) => {
                            self.stream.push(Token{
                                id: TokenIds::ElemSeperator,
                                value: ",".to_string(),
                            });
                        },
                        _ => {
                            return Some(("Comma appeared while not in a function parameter statement", self.line_num));
                        },
                    }
                }
                ">" | "<" => {
                    if input.get(i+1) == Some(&String::from("=")){
                        self.stream.push(Token{
                            id: TokenIds::Operand,
                            value: format!("{}=",val),
                        });
                        self.skip+=1;
                    }
                    else{
                        self.stream.push(Token{
                            id: TokenIds::Operand,
                            value: val.to_string(),
                        });
                    }
                }
                "\n" => {
                    if self.untokenized != 0{
                        return Some(("Failed to understand something in this line. Common mistakes include variables accessed from out of scope or misspellings.", self.line_num));
                    }
                    else if self.bracket_para.len() > 0{
                        return Some(("Terminator apeared inside of a paranthese or bracket statement", self.line_num));
                    }
                    else{
                        self.line_num+=1;
                        if self.stream.len() > 0 && (self.stream.last().unwrap().id != TokenIds::Terminator){
                            self.stream.push(Token{
                                id: TokenIds::Terminator,
                                value: "\n".to_string(),
                            });
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
                    for scope in self.vars.iter(){
                        if scope.contains(val){
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
        if self.vars.len() > 1{
            return Some(("File ended but there are still unclosed block openers(curly braces)", self.line_num));
        }
        return None;
    }
}

fn preproccess(preproccessed: &mut Vec<String>, input: String){
    let mut skip: isize = 0;
    let mut tmp: String = String::new();
    preproccessed.push(String::new());
    for (i, letter) in input.chars().enumerate(){
        if skip > 0{
            skip-=1;
            continue;
        }
        match letter{
            ')' | '('| '\n' | '>' | '<'  | '*' | '+' | '/' | '=' | '!' | ',' | '}' | '{' | '[' | ']'=> {
                preproccessed.push(tmp);
                preproccessed.push(letter.to_string());
                tmp = String::new();
                //preproclen+=2;
            }
            '-' => {
                if preproccessed.last() == Some(&String::from("<")){
                    preproccessed.last_mut().unwrap().push('-');
                }
                else{
                    preproccessed.push(tmp);
                    preproccessed.push(String::from("-"));
                    tmp = String::new();
                }
            }
            '\"' => {
                preproccessed.push(tmp);
                preproccessed.push("\"".to_string());
                tmp = String::new();
                let mut j = i+1;
                'strloop: loop{
                    match input.get(j..j+1){
                        Some("\"") => {
                            preproccessed.push(tmp);
                            tmp = String::new();
                            preproccessed.push("\"".to_string());
                            skip+=1;
                            break 'strloop;
                        },
                        None => break 'strloop,
                        Some(n) => tmp.push(n.as_bytes()[0] as char),
                    }
                    j+=1;
                    skip+=1;
                }
            }
            ' ' => {
                preproccessed.push(tmp);
                tmp = String::new();
                preproccessed.push(String::new());
                //preproclen+=1;
            }
            _ => tmp.push(letter),
        }
    }
}

fn replace_empty_strings(preproccessed: &mut Vec<String>){
    let mut i = 0;
    while i< preproccessed.len(){
        if preproccessed[i] == ""{
            preproccessed.remove(i);
        }
        else{
            i+=1;
        }
    }
}

impl Compiler {
    pub fn tokenize(&mut self){
        // make mainpulating easier
        let mut preproccessed: Vec<String> = Vec::new();
        preproccess(&mut preproccessed, self.input.clone());

        //replace empty strings so they don't cause problems
        replace_empty_strings(&mut preproccessed);

        let mut res = DevelopingTokens{
            stream: Vec::new(),
            bracket_para: Vec::new(),
            in_proc: false,
            untokenized: 0,
            neg_num: false,
            vars: Vec::new(),
            funcs: vec!["DISPLAY".to_string(), "INPUT".to_string(), "RANDOM".to_string(), "INSERT".to_string(), "REMOVE".to_string(), "APPEND".to_string(), "LENGTH".to_string()],
            skip: 0,
            line_num: 0,
        };
        // return types specifies erorr None is no error Some is an error.
        // I can't do the say error method in the match tokens method because it is part of another
        // object
        match res.match_tokens(&preproccessed){
            Some(error) => self.say_error(error.0, error.1),
            None => (),
        }
        self.tokens = res.stream;
    }
}
