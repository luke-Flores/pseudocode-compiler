
#[derive(Debug, Clone)]
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
}


#[derive(Debug, Clone)]
pub struct Token{
    id: TokenIds,
    value: String,
}

struct DevelopingTokens{
    stream: Vec<Token>,
    //flags
    function_parameters: bool,
    in_string: bool,
    temp: Vec<String>,
    untokenized: u16,
}

impl DevelopingTokens{
    fn match_tokens(&mut self, input: &Vec<&str>){
        for (i, val) in input.iter().enumerate(){
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
                    if self.temp.len() != 0{
                        self.temp.push(" ".to_string());
                    }
                    self.temp.push(val.to_string());
                }
            }
            else if val == &"(" {
                if i != 0{
                    self.stream.push(Token{
                        id: TokenIds::FunctionName,
                        value: input.iter().nth(i-1).unwrap().to_string(),
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
            // there are empty string idk why
            else if val != &""{
                self.untokenized+=1;
            }
        }
    }
}

pub fn tokenize(input: String) -> Vec<Token>{
    // make mainpulating easier
    //let mut token_stream: Vec<Token> = Vec::new();
    let mut preproccessed: String = String::with_capacity(input.len());
    for letter in input.chars() {
        match letter {
            // split string at these values plus space
            ')' | '(' | '\"' | '\n' => {
                preproccessed.push(' ');
                preproccessed.push(letter);
                preproccessed.push(' ');
            },
            _ => preproccessed.push(letter),
        }
    }

    let preproccessed: Vec<&str> = preproccessed.split(' ').collect();
    //let token_stream: Vec<Token> = Vec::new();

    let mut res = DevelopingTokens{
        stream: Vec::new(),
        function_parameters: false,
        in_string: false,
        temp: Vec::new(),
        untokenized: 0,
    };
    res.match_tokens(&preproccessed);
    return res.stream.clone();
}



