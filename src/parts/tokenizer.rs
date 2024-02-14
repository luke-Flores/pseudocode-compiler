
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
}

impl DevelopingTokens{
    fn match_tokens(&mut self, input: &Vec<&str>){
        for (i, val) in input.iter().enumerate(){
            if val == &"(" {
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
                }
                else {
                    panic!("Function called without name, aborting");
                }
            }
            else if val == &"\""{
                if !self.in_string{
                    self.stream.push(Token{
                        id: TokenIds::StringBeg,
                        value: "\"".to_string(),
                    });

                }
                else {
                    self.in_string = true;

                }
            }

            else  if val == &")"{
                let useless = 0;
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
            ')' | '(' | '\"' => {
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
    };
    res.match_tokens(&preproccessed);
    return res.stream.clone();
}



