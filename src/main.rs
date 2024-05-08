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
    
        Ok(data) => data,
        
        };
    
    
        // print out the lexer tokens parsed.
    
        println!("----------------------");
        println!("Finished Lexing the file {}", filename);
        println!("Expression:");
        println!("{code}");
        println!("Here are the Results:");
        println!("----------------------");
        for t in &tokens { //make sure '&' here because rust not gonna work
          println!("{:?}", t);
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
    return(false, code);
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

fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<String>, String> {
  match peek(tokens, *index) {
  None => {
      return Ok(None);
  }

  Some(token) => {
      let codenode: Option<String>;
      match token {

      Token::RightCurly => {
          return Ok(None);
      }

      Token::Int => {
          *index += 1;
          match next_result(tokens, index)? {
          Token::Ident(ident) => {
              let statement = format!("%int {}\n", ident);
              codenode = Some(statement);
          }

          _ => {
              return Err(String::from("expected identifier"));
          }

          }
      }

      Token::Ident(ident) => {
          *index += 1;
          if !matches!(next_result(tokens, index)?, Token::Assign) {
              return Err(String::from("expected '=' assignment operator"));
          }
          let expr = parse_expression(tokens, index)?;
          let code = format!("{}%mov {}, {}\n", expr.code, ident, expr.name);
          codenode = Some(code);
      }

      Token::Return => {
          *index += 1;
          let expr = parse_expression(tokens, index)?;
          let code = format!("{}%ret {}\n", expr.code, expr.name);
          codenode = Some(code);
      }

      Token::Print => {
          *index += 1;
          if !matches!(next_result(tokens, index)?, Token::LeftParen) {
              return Err(String::from("expect '(' closing statement"));
          }

          let expr = parse_expression(tokens, index)?;
          let code = format!("{}%out {}\n", expr.code, expr.name);
          if !matches!(next_result(tokens, index)?, Token::RightParen) {
              return Err(String::from("expect ')' closing statement"));
          }
          codenode = Some(code);
      }

      Token::Read => {
          *index += 1;
          if !matches!(next_result(tokens, index)?, Token::LeftParen) {
              return Err(String::from("expect '(' closing statement"));
          }

          let expr = parse_expression(tokens, index)?;
          let code = format!("{}%input {}\n", expr.code, expr.name);

          if !matches!(next_result(tokens, index)?, Token::RightParen) {
              return Err(String::from("expect ')' closing statement"));
          }
          codenode = Some(code);
      }

      _ => {
           return Err(String::from("invalid statement."));
      }

      }

      if !matches!(next_result(tokens, index)?, Token::Semicolon) {
          return Err(String::from("expect ';' closing statement"));
      }

      return Ok(codenode);
  }
  
  }
}


fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
  let mut expr = parse_term(tokens, index)?;
  loop {
    let opcode = match peek_result(tokens, *index)? {
    Token::Plus => "%add",
    Token::Subtract => "%sub",
    Token::Multiply => "%mult",
    Token::Divide => "%div",
    Token::Modulus => "%mod",

    _ => break,

    };

    *index += 1;
    let m_expr = parse_term(tokens, index)?;
    let t = create_temp();
    let instr = format!("%int {}\n{opcode} {}, {}, {}\n", t, t, expr.name, m_expr.name);
    expr.code += &m_expr.code;
    expr.code += &instr;
    expr.name = t;
  }
  return Ok(expr);
}

fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
  match next_result(tokens, index)? {

  Token::Ident(ident) => {
      let expr = Expression {
          code : String::from(""),
          name : ident.clone(),
      };
      return Ok(expr);
  }

  Token::Num(num) => {
      let expr = Expression {
          code : String::from(""),
          name : format!("{}", num),
      };
      return Ok(expr);
  }

  _ => {
      return Err(String::from("invalid expression"));
  }

  }

  loop {
    match peek(tokens, *index) {
        Some(&Token::Multiply) | Some(&Token::Divide) | Some(&Token::Modulus) => {
            let op = next_result(tokens, index)?;
            let next_expr = parse_factor(tokens, index)?;

            expr.code.push_str(&next_expr.code);
            expr.code.push_str(&format!("%{} {}, {}, {}\n", op, expr.name, expr.name, next_expr.name));

            expr.name = generate_temporary();
        }
        _ => break,
    }
  }

}

fn parse_function(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<String>, String> {
    
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
  let func_ident = match next_result(tokens, index)? {
  Token::Ident(func_ident) => func_ident,
  _  => {return Err(String::from("functions must have a function identifier"));}
  };

  if !matches!( next_result(tokens, index)?, Token::LeftParen) {
      return Err(String::from("expected '('"));
  }

  let mut code = format!("%func {}\n", func_ident);
  let mut params: Vec<String> = vec![];
  loop {
     match next_result(tokens, index)? {

     Token::RightParen => {
         break;
     }

     Token::Int => {
         match next_result(tokens, index)? {
         Token::Ident(param) => {
             params.push(param.clone());
             match peek_result(tokens, *index)? {
             Token::Comma => {
                 *index += 1;
             }
             Token::RightParen => {}
             _ => {
                 return Err(String::from("expected ',' or ')'"));
             }

             }
         }
         _ => {
              return Err(String::from("expected ident function parameter"));
         }

         }
     }

     _ => {
         return Err(String::from("expected 'int' keyword or ')' token"));
     }

     }
  }


  if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
      return Err(String::from("expected '{'"));
  }

  loop {
      match parse_statement(tokens, index)? {
      None => {
          break;
      }
      Some(statement) => {
          code += &statement;
      }
      }
  }

  code += "%endfunc\n\n";

  if !matches!(next_result(tokens, index)?, Token::RightCurly) {
    return Err(String::from("expected '}'"));
  }

  return Ok(Some(code));
}


// writing tests!
// testing shows robustness in software, and is good for spotting regressions
// to run a test, type "cargo test" in the terminal.
// Rust will then run all the functions annotated with the "#[test]" keyword.
#[cfg(test)]
mod tests {
    use crate::Token;
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
    }

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