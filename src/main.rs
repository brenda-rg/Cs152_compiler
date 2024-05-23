use std::{env, fs};

mod interpreter;

fn main() {
    // get commandline arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide an input file.");
        return;
    }

    if args.len() > 2 {
        println!("Too many commandline arguments.");
        return;
    }

    // read the entire file.
    let filename = &args[1];
    let result = fs::read_to_string(filename);
    let code = match result {
        Err(error) => {
            println!("**Error. File \"{}\": {}", filename, error);
            return;
        }

        Ok(code) => {
            code
        }
    };

    // Start Here!!
 
    
    let tokens = match lex(&code) {
        Err(error_message) => {
            println!("**Error**");
            println!("----------------------");
            println!("{}", error_message);
            println!("----------------------");
            return;
        }
    
        Ok(tokens) => tokens,
        
        };
    

        // print out the lexer tokens parsed.
    
        /* println!("----------------------");
        println!("Finished Lexing the file {}", filename);
        println!("Expression:");
        println!("{code}");
        println!("Here are the Results:");
        println!("----------------------");
        for t in &tokens { //make sure '&' here because rust not gonna work
          println!("{:?}", t);
        } */

        let mut index: usize = 0;
        match parse_program(&tokens, &mut index) {
          Ok(generated_code) => {
            println!("Program Parsed Successfully.");
            println!("-------------------------------");
            println!("Generated Code:");
            println!("-------------------------------");
            println!("{generated_code}");
            println!("-------------------------------");
            interpreter::execute_ir(&generated_code);
          }
          Err(message) => {
            println!("**Error**");
            println!("----------------------");
            if tokens.len() == 0 {
                println!("No code has been provided.");
            } else {
                println!("Error: {message}");
                println!("----------------------");
            }
          }

        }
}
    
    // Creating an Enum within Rust.
    // Documentation: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
    // Enums are a way of saying a value is one of a possible set of values.
    // Unlike C, Rust enums can have values associated with that particular enum value.
    // for example, a Num has a 'i32' value associated with it, 
    // but Plus, Subtract, Multiply, etc. have no values associated with it.
    #[derive(Debug, Clone)]
enum Token {
  NotToken,
  Plus,
  Subtract,
  Multiply,
  Divide,
  Modulus,
  Assign,
  //
  Less,
  LessEqual,
  Greater,
  GreaterEqual,
  Equality,
  NotEqual,
  LeftParen,
  RightParen,
  LeftCurly,
  RightCurly,
  LeftBracket,
  RightBracket,
  Comma,
  Semicolon,
  //
  Num(i32), //saying that you can have a token of type number and can also store the numerical value
  Ident(String),
  If,
  While,
  Read, 
  Func,
  Return,
  Int,
  Print,
  Else,
  Break,
  Continue,
}

// In Rust, you can model the function behavior using the type system.
// https://doc.rust-lang.org/std/result/
// Result < Vec<Token>, String>
// means that this function can either return:
// - A list of tokens as a Vec<Token>
// - Or an error message represented as a string
// If there is an error, it will return an error
// If successful, it will return Vec<Token>
// A Result is an enum like this:
// enum Result {
//     Ok(the_result),
//     Err(the_error),
// }


// This is a lexer that parses numbers/identifiers and math operations
fn lex(mut code: &str) -> Result<Vec<Token>, String> { //takes in a string and returns a vector
  let mut tokens: Vec<Token> = vec![];
  while code.len() > 0 {
    let (success, token, rest) = lex_number(code);
    if success {
      code = rest; 
      tokens.push(token);
      continue;
    } 
 
    let (success, rest) = lex_space(code);
    if success {
      code = rest;
      continue;
    }

    if code.starts_with("+") {
      code = &code[1..];
      tokens.push(Token::Plus);
      continue;
    }

    let (success, rest) = lex_comment(code);
    if success {
      code = rest;
      continue;
    }

    if code.starts_with("-") {
      code = &code[1..];
      tokens.push(Token::Subtract);
      continue;
    }

    if code.starts_with("*") {
      code = &code[1..];
      tokens.push(Token::Multiply);
      continue;
    }

    if code.starts_with("/") {
      code = &code[1..];
      tokens.push(Token::Divide);
      continue;
    }

    if code.starts_with("%") {
      code = &code[1..];
      tokens.push(Token::Modulus);
      continue;
    }

    if code.starts_with(",") {
      code = &code[1..];
      tokens.push(Token::Comma);
      continue;
    }

    if code.starts_with("(") {
      code = &code[1..];
      tokens.push(Token::LeftParen);
      continue;
    }

    if code.starts_with(")") {
      code = &code[1..];
      tokens.push(Token::RightParen);
      continue;
    }

    if code.starts_with("[") {
      code = &code[1..];
      tokens.push(Token::LeftBracket);
      continue;
    }

    if code.starts_with("]") {
      code = &code[1..];
      tokens.push(Token::RightBracket);
      continue;
    }

    if code.starts_with("{") {
      code = &code[1..];
      tokens.push(Token::LeftCurly);
      continue;
    }

    if code.starts_with("}") {
      code = &code[1..];
      tokens.push(Token::RightCurly);
      continue;
    }

    if code.starts_with(";") {
      code = &code[1..];
      tokens.push(Token::Semicolon);
      continue;
    }

    let (success, token, rest) = lex_equality(code);
    if success {
      code = rest; 
      tokens.push(token);
      continue;
    } 
    

    let (success, token, rest) = lex_identifier(code);
    if success {
      code = rest;
      tokens.push(token);
      continue;
    }

    let symbol = unrecognized_symbol(code);
    return Err(format!("Unidentified symbol {symbol}"));

  }

  return Ok(tokens);
}

fn lex_space(code: &str) -> (bool, &str) {
  for letter in code.chars() {
    if letter.is_whitespace() {
      return (true, &code[1..]);
    } else {
      return (false, code);
    }
  }
  return (false, code);
}

fn lex_comment(code: &str) -> (bool, &str) {
  enum StateMachine {
    Start,
    Filter,
  }
    let mut success = false;
    let mut state = StateMachine::Start;
    let mut index = 0;
    for letter in code.chars() {
      match state {
      StateMachine::Start => {
        if letter == '#' {
          state = StateMachine::Filter;
          success = true;
          index += 1;
        }
        else {
          return(false, code);
        }
      }
      StateMachine::Filter => {
        if letter != '\n' {
          state = StateMachine::Filter;
          success = true;
          index += 1;
        }
        else {
          return(true, &code[index..]);
        }
      }
      }
    }
    if success == true {
      return (true, &code[index..]);
    } else {
      return (false, "");
    }
}

// lex numbers.
fn lex_number(code: &str) -> (bool, Token, &str) {
  enum StateMachine {
    Start,
    Number,
  }

  let mut success = false;
  let mut state = StateMachine::Start;
  let mut index = 0;
  for letter in code.chars() {
    match state {
    StateMachine::Start => {
      if letter >= '0' && letter <= '9' {
        state = StateMachine::Number;
        success = true;
        index += 1;
      } else {
        return (false, Token::NotToken, "");
      }
    }

    StateMachine::Number => {
      if letter >= '0' && letter <= '9' {
        state = StateMachine::Number;
        success = true;
        index += 1;
      }
      else if (letter >= 'a' && letter <= 'z') || (letter >= 'A' && letter <= 'Z'){
        return(false, Token::NotToken, "");
      } else {
        let num = code[..index].parse::<i32>().unwrap();
        return (true, Token::Num(num), &code[index..]);
      }
    }

    }
  }

  if success == true {
    let num: i32 = code.parse::<i32>().unwrap();
    return (true, Token::Num(num), "");
  } else {
    return (false, Token::NotToken, "");
  }
}

// lex identifiers.
fn lex_identifier(code: &str) -> (bool, Token, &str) {
  enum StateMachine {
    Start,
    Ident,
  }

  let mut success = false;
  let mut state = StateMachine::Start;
  let mut index = 0;
  for letter in code.chars() {
    match state {
    StateMachine::Start => {
      if (letter >= 'a' && letter <= 'z') || (letter >= 'A' && letter <= 'Z'){
        state = StateMachine::Ident;
        success = true;
        index += 1;
      } else {
        return (false, Token::NotToken, "");
      }
    }

    StateMachine::Ident => {
      if (letter >= 'A' && letter <= 'Z') || (letter >= 'a' && letter <= 'z') || (letter >= '0' && letter <= '9') || letter == '_' {
        state = StateMachine::Ident;
        success = true;
        index += 1;
      } else {
        let token = &code[..index];
        return (true, create_identifier(token), &code[index..]);
      }
    }

    }
  }

  if success == true {
    return (true, create_identifier(code), "");
  } else {
    return (false, Token::NotToken, "");
  }
}

// lex equalities.
fn lex_equality(code: &str) -> (bool, Token, &str) {
  enum StateMachine {
    Start,
    Sign,
    Sign2,
  }

  let mut success = false;
  let mut state = StateMachine::Start;
  let mut index = 0;
  for letter in code.chars() {
    match state {
    StateMachine::Start => {
      if letter == '<' || letter == '>' || letter == '=' || letter == '!' {
        state = StateMachine::Sign;
        success = true;
        index += 1;
      } else {
        return (false, Token::NotToken, "");
      }
    }

    StateMachine::Sign => {
      if letter == '=' {
        state = StateMachine::Sign2;
        success = true;
        index += 1;
      } else {
        let token = &code[..index];
        if token == "!" {
          return (false, Token::NotToken, "");
        }
        return (true, create_sign(token), &code[index..]);
      }
    }
    StateMachine::Sign2 => {
      let token = &code[..index];
      return (true, create_sign(token), &code[index..]);
    }
  }
  }
  if success == true {
    return (true, create_sign(code), "");
  } else {
    return (false, Token::NotToken, "");
  }
}

fn unrecognized_symbol(code: &str) -> &str {
  enum StateMachine {
    Start,
    Symbol,
  }

  let mut state_machine = StateMachine::Start;
  let mut index = 0;
  for letter in code.chars() {
    match state_machine {
    StateMachine::Start => {
      state_machine = StateMachine::Symbol;
      index += 1;
    } 
    
    StateMachine::Symbol => {
      if letter.is_whitespace() {
        return &code[..index];
      } else {
        index += 1;
      }
    }

    }
  }
  return &code[..index];
} 

fn create_identifier(code: &str) -> Token {
  match code {
  "func" => Token::Func,
  "return" => Token::Return,
  "int" => Token::Int,

  // todo: implement all keywords...
  "print" => Token::Print,
  "else" => Token::Else,
  "break" => Token::Break,
  "continue" => Token::Continue,

  // ... all keywords...
  "read" => Token::Read,
  "while" => Token::While,
  "if" => Token::If,
  _ => Token::Ident(String::from(code)), //if its anything else return identifier
  }
}

fn create_sign(code: &str) -> Token {
  match code {
    "<" => Token::Less,
    ">" => Token::Greater,
    "<=" => Token::LessEqual,
    ">=" => Token::GreaterEqual,
    "!=" => Token::NotEqual,
    "==" => Token::Equality,
    "=" => Token::Assign,
    _ => Token::NotToken,
  }
}
//check for token but returns none instead of error
fn peek<'a>(tokens: &'a Vec<Token>, index: usize) -> Option<&'a Token> {
  if index < tokens.len() {
      return Some(&tokens[index])
  } else {
      return None
  }
}

//same as peek but returns error
fn peek_result<'a>(tokens: &'a Vec<Token>, index: usize) -> Result<&'a Token, String> {
  if index < tokens.len() {
      return Ok(&tokens[index])
  } else {
      return Err(String::from("expected a token, but got nothing"))
  }
}
//returns index to current token and then increments the current index
fn next<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Option<&'a Token> {
  if *index < tokens.len() {
      let ret = *index;
      *index += 1;
      return Some(&tokens[ret])
  } else {
      return None
  }
}

fn next_result<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<&'a Token, String> {
  if *index < tokens.len() {
      let ret = *index;
      *index += 1;
      return Ok(&tokens[ret])
  } else {
      return Err(String::from("expected a token, but got nothing"))
  }
}

fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  loop {
      match parse_function(tokens, index)? {
      None => {
          break;
      }
      Some(_) => {}
      }
  }

  return Ok(());
}

fn parse_function(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<()>, String> {

  //func keyword (must)
  match next(tokens, index) {
  None => {
      return Ok(None);
  }
  Some(token) => {
      if !matches!(token, Token::Func) {
          return Err(String::from("functions must begin with func"));
      }
  } 
  }

  //ident (must)
  match next_result(tokens, index)? {
  Token::Ident(_) => {}, //if ident is next continue
  _  => {return Err(String::from("functions must have a function identifier"));} //else error
  }

  //'(' (must)
  if !matches!( next_result(tokens, index)?, Token::LeftParen) {
      return Err(String::from("expected '(' in function"));
  }

  //loop to check for declarations
  loop {
    match peek(tokens, *index) {
      None => {return Ok(None)}
      Some(token) => {
        // if ')' then no delcarations --> exit loop
        match token {
        Token::RightParen => {
            break;
        }
          _ => {}
        }
      }
    }
      // else parse declaration
      match parse_declaration(tokens, index)? {
      //no more declarations to parse --> exit loop
      None => {
        return Ok(None);
      }
      Some(_) => {

        match peek(tokens, *index) {
          None => {return Ok(None)}
          Some(token) => {
          // check for comma
            match token {
            Token::Comma => {
              *index += 1;
              match peek_result(tokens, *index)? {
                //return error if ')' after a ','
                Token::RightParen => {
                  return Err(String::from("Expected ')' to finish function declarations but got ',' "));
                }
                _ => {} // continue if anything else
              }
            }
            _ => {}
          }
         }
        }
      } //continue parsing
      }
    }


  match next_result(tokens, index)? {
    Token::RightParen => {}, //if ident is next continue
    _  => {return Err(String::from("expected ')' in function"));}//else error
    }

  if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
      {return Err(String::from("expected '{' in function"));}
  }

  loop {
      match parse_statement(tokens, index)? {
      None => {
        break;
      }
      Some(()) => {}
      }
  }

  if !matches!(next_result(tokens, index)?, Token::RightCurly) {
    return Err(String::from("expected '}' in function"));
  }
  return Ok(Some(()));
}


fn parse_declaration(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<()>, String> {

  match next(tokens, index) {
    None => {
        return Ok(None);
    }
    Some(token) => {
      if !matches!(token, Token::Int) {
          return Err(String::from("declarations must begin with int"));}
      } 
    }
  // check for int in declaration. Next check '[' or Ident
  match peek(tokens, *index) {
    None => {
      return Ok(None);
    }
    Some(token) => {
      match token {

      Token::LeftBracket => {
        *index += 1;
        match next_result(tokens, index)? {
          Token::Num(_) => {}
          _ => {return Err(String::from("expected [number] in declaration"));}
        }
        if !matches!(next_result(tokens, index)?, Token::RightBracket) {
          {return Err(String::from("expected ']' in the declaration"));}
        }
        }
        _ => {}
      }
    }
  }
  match next_result(tokens, index)? {
    Token::Ident(_) => {},
    _  => {return Err(String::from("expected '[num]' or an identifier in decalration"));}
    };
  return Ok(Some(()))
}

fn parse_var(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<()>, String> {
  match next(tokens, index) {
    None => {
      return Ok(None);
    }
    Some(token) => {
      if !matches!(token, Token::Ident(_)) {
        return Err(String::from("expected identifier in var"));
      }
    }
  }
  match peek(tokens, *index) {
    None => {
      return Ok(None);
    }
    Some(token) => {
      match token {
        Token::LeftBracket => {
          *index += 1;
          parse_expression(tokens, index)?;
          if !matches!(next_result(tokens, index)?, Token::RightBracket) {
            return Err(String::from("expected ']' in var"))
          }
        }
        _ => {}
      }

      return Ok(Some(()));
    }
  }
}

fn parse_while_loop(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<()>, String> {
  match peek(tokens, *index) {
    None =>  {
      return Ok(None);
    }
    Some(token) => {
      if !matches!(token, Token::While) {
        return Err(String::from("while loops must begin with while keyword"));
      }
      *index += 1;
      parse_bool_expr(tokens, index)?;
      if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
        return Err(String::from("missing '{' in while loop"));
      }
      
      loop {
        match parse_statement(tokens, index)? {
        None => {
            break;
        }
        Some(()) => {}
        }
      }

      if !matches!(next_result(tokens, index)?, Token::RightCurly) {
        return Err(String::from("Missing '}' in while loop"));
      }
      return Ok(Some(()));
    }
  }
}

fn parse_if(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<()>, String> {
  match next(tokens, index) {
    None =>  {
      return Ok(None);
    }
    Some(token) => {
      if !matches!(token, Token::If) {
        return Err(String::from("If statements must begin with if keyword"));
      }
      
      parse_bool_expr(tokens, index)?;
      if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
        return Err(String::from("missing '{' in if statement"));
      }
      
      loop {
        match parse_statement(tokens, index)? {
        None => {
            break;
        }
        Some(()) => {}
        }
      }

      if !matches!(next_result(tokens, index)?, Token::RightCurly) {
        return Err(String::from("Missing '}' in if statement"));
      }

      match peek(tokens, *index) {
        None => {return Ok(None)}
        Some(token) => {
          match token {
            Token::Else => {
              *index += 1;
              if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
                return Err(String::from("missing '{' in else statement"));
              }
              parse_statement(tokens, index)?;
              if !matches!(next_result(tokens, index)?, Token::RightCurly) {
                return Err(String::from("missing '}' in else statement"));
              }
            }
            _ => {}
          }
          return Ok(Some(()))
        }
      }
    }
  }
}


fn parse_bool_expr(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<()>, String> {

  match peek(tokens, *index) {
    None => {
      return Ok(None);
    }
    Some(_) => {
      parse_expression(tokens, index)?;
      match peek_result(tokens, *index)? {
        Token::Less => {},
        Token::Greater => {},
        Token::LessEqual => {},
        Token::GreaterEqual => {},
        Token::Equality => {},
        Token::NotEqual => {},
        _ => {return Err(String::from("expected comparison symbol in bool expression"));}
      }
      *index += 1;
      parse_expression(tokens, index)?;
      return Ok(Some(()));
    }
  }
}

fn parse_mult_expr(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<()>, String> {

  match peek(tokens, *index) {
    None => {
      return Ok(None);
    }
    Some(_) => {
      parse_term(tokens,index)?;
      loop {
        match peek(tokens, *index) {
          None => {
            break;
          }
          Some(token) => {
            match token {
              Token::Multiply => {
                *index += 1;
              },
              Token::Divide => {
                *index += 1;
              },
              Token::Modulus => {
                *index += 1;
              },
              _ => {break;}
            }
            parse_term(tokens, index)?;
          }
        }
      }
      return Ok(Some(()))
    }
  }
}

fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<String>, String> {
  match peek(tokens, *index) {
  None => {
      return Ok(None);
  }

  Some(token) => {
      let code: String;
      match token {
      Token::RightCurly => {
        return Ok(None);
      }

      Token::Int => {
          parse_declaration(tokens, index)?;
      }

      Token::Ident(_) => {
          parse_var(tokens, index)?;
          
          if !matches!(next_result(tokens, index)?, Token::Assign) {
              return Err(String::from("expected '=' assignment operator"));
          }
          parse_expression(tokens, index)?;
      }

      Token::Return => {
          *index += 1;
          parse_expression(tokens, index)?;
      }

      Token::Print | Token::Read => {
        *index += 1;
        parse_term(tokens, index)?;
      }

      Token::Break | Token::Continue => {
        *index += 1;
      }

      Token::While => {
        parse_while_loop(tokens, index)?;
        return Ok(Some(()));
      }
      Token::If => {
        parse_if(tokens, index)?;
        return Ok(Some(()));
      }

      _ => {
           return Err(String::from("invalid statement"));
      }

      }
      if !matches!(next_result(tokens, index)?, Token::Semicolon) {
          return Err(String::from("expected ';' closing statement"));
      }

      return Ok(Some(()));
  }

  }
}

fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  let mut expr = parse_mult_expr(tokens, index)?;
  loop {
    let opcode = match peek_result(tokens, *index)? {
    Token::Plus => "%add",
    Token::Subtract => "%sub",
    _ => { break; }
    };
    
    *index += 1;
    let m_expr = parse_mult_expr(tokens, index)?;
    let t = create_temp();
    let instr = format!("%int {}\n{opcode} {}, {}, {}\n", t, t, expr.name, m_expr.name);
    expr.code += &m_expr.code;
    expr.code += &instr;
    expr.code = t;
  }
  return Ok(expr);
}

fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match next_result(tokens, index)? {
      Token::Num(_) => {
        return Ok(());
      }
      Token::LeftParen => {
        parse_expression(tokens, index)?;
        match next_result(tokens, index)? {
          Token::RightParen => {}
          _ => {return Err(String::from("term missing closing ')' "));}
        }
        return Ok(())
      }
      Token::Ident(_) => {
           match peek_result(tokens, *index)? {
            Token::LeftBracket => {
              *index += 1;
              parse_expression(tokens, index)?;
              match next_result(tokens, index)? {
                Token::RightBracket => {}
                _ => {return Err(String::from("term missing closing ']'"));}
              }
            }
            Token::LeftParen => {
              *index += 1;
              loop {
                match peek_result(tokens, *index)? {
                  Token::RightParen => {
                    break;
                  }
                  _ => {}
                }
                parse_expression(tokens, index)?;
                match peek_result(tokens, *index)? {
                  Token::Comma => {
                    *index += 1;
                    parse_expression(tokens, index)?;
                  }
                  _ => {}
                }
              }
              match next_result(tokens, index)? {
                Token::RightParen => {}
                _ => {return Err(String::from("missing closing ')' in \"ident ( expr? )\" "));}
              }
            }
            _ => {}
          }
          return Ok(());
      }
  
      _ => {
          return Err(String::from("invalid term expression"));
      } 
  }
  
}


// writing tests!
// testing shows robustness in software, and is good for spotting regressions
// to run a test, type "cargo test" in the terminal.
// Rust will then run all the functions annotated with the "#[test]" keyword.
#[cfg(test)]
mod tests {
    /* use crate::Token;
    use crate::lex;

    #[test]
    fn lexer_test() {
        // test that lexer works on correct cases
        let toks = lex("1 + 2 + 3").unwrap();
        assert!(toks.len() == 5);
        assert!(matches!(toks[0], Token::Num(1)));
        assert!(matches!(toks[1], Token::Plus));
        assert!(matches!(toks[2], Token::Num(2)));
        assert!(matches!(toks[3], Token::Plus));
        assert!(matches!(toks[4], Token::Num(3)));

        let toks = lex("3 + 215 +-").unwrap();
        assert!(toks.len() == 5);
        assert!(matches!(toks[0], Token::Num(3)));
        assert!(matches!(toks[1], Token::Plus));
        assert!(matches!(toks[2], Token::Num(215)));
        assert!(matches!(toks[3], Token::Plus));
        assert!(matches!(toks[4], Token::Subtract));

        // test that the lexer catches invalid tokens
        assert!(matches!(lex("^^^"), Err(_)));
    } */
    use crate::lex;
    use crate::parse_statement;

    #[test]
    fn test_statements() {

        // test that valid statements are correct.
        let tokens = lex("a = 1 + 2;").unwrap();
        parse_statement(&tokens, &mut 0).unwrap();

        let tokens = lex("b = 1 / 2;").unwrap();
        parse_statement(&tokens, &mut 0).unwrap();


        // test errors. missing semicolon
        let tokens = lex("b = 1 / 2").unwrap();
        assert!(matches!(parse_statement(&tokens, &mut 0), Err(_)));

    }

}