mod lexer;
mod utils;

fn main() {
    let argv = utils::get_argv();
    let init_file_string = utils::get_file(&argv[1]);

    let mut c_token = lexer::Tokeniser::new(init_file_string);
    let _tokenise = lexer::Tokeniser::tokenise(&mut c_token);
    println!("{:#?}", c_token);
}
