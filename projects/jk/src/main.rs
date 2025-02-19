mod lexer;
mod utils;

fn main() {
    let argv = utils::get_argv();
    let init_file_string = utils::get_file(&argv[1]);

    let mut c_token = lexer::Tokens::new(init_file_string);
    let _tokenise = lexer::Tokens::tokenise(&mut c_token);
    println!("{:#?}", c_token);
}
