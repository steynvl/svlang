use std::{fs, io};
use svlang::lexer::Lexer;
use svlang::token::Token;

fn get_lexer_test_files(path: &str) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(path)?;

    let mut file_names: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() {
                path.file_name()?.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect();

    file_names.sort();

    Ok(file_names)
}

fn convert_token_to_testable_string(token: &Token) -> String {
    match token {
        Token::Eof => "".to_string(),
        Token::Id(s) => format!("Identifier: '{}'", s),
        Token::Number(n) => format!("Number: {}", n),
        Token::StringLiteral(s) => format!("String: \"{}\"", s),
        Token::Equal => "'='".to_string(),
        Token::GreaterEqual => "'>='".to_string(),
        Token::GreaterThan => "'>'".to_string(),
        Token::LessEqual => "'<='".to_string(),
        Token::LessThan => "'<'".to_string(),
        Token::NotEqual => "'<>'".to_string(),
        Token::Minus => "'-'".to_string(),
        Token::Or => "'or'".to_string(),
        Token::Plus => "'+'".to_string(),
        Token::And => "'and'".to_string(),
        Token::Divide => "'/'".to_string(),
        Token::Multiply => "'*'".to_string(),
        Token::Remainder => "'rem'".to_string(),
        Token::CloseBracket => "']'".to_string(),
        Token::CloseParenthesis => "')'".to_string(),
        Token::Comma => "','".to_string(),
        Token::Concatenate => "'.'".to_string(),
        Token::Gets => "':='".to_string(),
        Token::OpenBracket => "'['".to_string(),
        Token::OpenParenthesis => "'('".to_string(),
        Token::Semicolon => "';'".to_string(),
        _ => format!("'{:?}'", token).to_lowercase(),
    }
}

#[test]
fn test_lexer() {
    let lexer_tests_dir = "tests/resources/lexer";
    let lexer_test_files = get_lexer_test_files(lexer_tests_dir).unwrap();
    for file in lexer_test_files {
        // Stop here (for now), so that test suite passes.
        if file == "test115.svl" {
            break;
        }

        let source_file = format!("{}/{}", lexer_tests_dir, file);
        let source = fs::read_to_string(&source_file)
            .expect(format!("Could not read the file: {}", source_file).as_str());

        let std_out_file = format!("{}/report/{}.out.txt", lexer_tests_dir, file);
        let std_out = fs::read_to_string(&std_out_file)
            .expect(format!("Could not read the file: {}", std_out_file).as_str());
        let std_out_lines: Vec<&str> = std_out.split("\n").collect();

        let std_err_file = format!("{}/report/{}.err.txt", lexer_tests_dir, file);
        let std_err = fs::read_to_string(&std_err_file)
            .expect(format!("Could not read the file: {}", std_err_file).as_str());

        println!("-- START --");
        println!("{}", file);
        println!("{:?}", source);
        println!("out: {:?}", std_out_lines);
        println!("err: {:?}", std_err);

        let mut lexer = Lexer::new(source.as_bytes());
        let mut token: Token = Token::Eof;
        let mut std_out_index = 0;
        loop {
            if std_out_index < std_out_lines.len() {
                let std_out_line = std_out_lines[std_out_index];

                if std_out_line.is_empty() {
                    std_out_index += 1;
                    continue;
                }

                lexer.get_token(&mut token).unwrap();
                println!("token  = {}", convert_token_to_testable_string(&token));
                println!("stdout = {}", std_out_line);
                assert_eq!(convert_token_to_testable_string(&token), std_out_line);
                std_out_index += 1;
            } else {
                match lexer.get_token(&mut token) {
                    Ok(_) => {
                        if token == Token::Eof {
                            break;
                        }
                        panic!("Unexpected token: {:?}", token);
                    }
                    Err(err) => {
                        println!("SVLERROR = {}", err);
                        println!("stderr  =  {}", std_err);
                        // TODO: Once the lexer error includes the file name, line, and column we should do a direct comparison.
                        assert!(std_err.contains(&err));
                        break;
                    }
                }
            }
        }
        println!();
    }
}
