
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
    meta_data: Option<String>,
}

#[derive(Debug)]
pub struct TokenStream{
    pub tokens: Vec<Token>,
}

pub fn preproccess<'a>( input: &'a String) -> Vec<Vec<&'a str>> {
    let lines: Vec<&str> = input.lines().collect();
    let mut res: Vec<Vec<&str>> = Vec::new();
    let mut splits: Vec<usize> = Vec::new();
    let mut temp_line: Vec<&str> = Vec::new();
    for mut line in lines{
        splits = Vec::new();
        // find indices of splits
        for (i, letter) in line.chars().enumerate(){
            match letter {
                ')' | '(' | '\"' => splits.push(i),
            }
        }
        if splits.len() > 0{
            let mut len_split = 0;
            let parts = line.split_at(splits[0]);
            temp_line.push(line.split_at(parts.0));
            temp_line.push(line.split_at(parts.1));
            splits.remove(0);
            len_split+=len(parts.0);
            for split_at in splits{
                let parts = temp_line[temp_line.len()].split_at(split_at-len_split);
                temp_line.remove(
                split_count+=1;
            }
        }

    }
    return res;
}

impl TokenStream {
    fn open_para(&self, mut i: usize) -> Token{
        if i!=0_usize {
            i-=1;
        }

        loop {
            break;
        }
        return Token{
            id: TokenIds::FunctionBeg,
            value: "(".to_string(),
            meta_data: None,
        };
    }

    pub fn tokenize(&mut self, mut input: String) -> Vec<Token>{
        for (i, val) in input.chars().enumerate(){
            if val == '('{
                self.tokens.push(self.open_para(i.clone()));
            }
        }


        return self.tokens.clone();
    }

}




