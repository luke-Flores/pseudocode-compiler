use std::fs;
use std::env;
use cbpcc::parts::tokenizer;
use cbpcc::parts::tokenizer::tokenize;

fn main(){
    let mut argv = env::args();
    let argc = argv.len();

    if argc == 1{
        eprintln!("expected an option, please provide input. For help do `cbpcc --help`");
    }
    else if argc == 2{
       let test = match fs::read_to_string(argv.nth(1).unwrap()){
            Ok(n) => tokenize_stage(n),
            Err(e) => {
                eprintln!("problem opening or reading file, rust error msg: {}", e);
                return;
            },
        };

    }
}

fn tokenize_stage(input: String){
    let token_stream = tokenize(input);

    println!("contents: {:#?}", token_stream);
}
