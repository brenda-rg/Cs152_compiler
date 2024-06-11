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

static mut VAR_NUM: i64 = 0;

fn create_temp() -> String {
    unsafe {
        VAR_NUM += 1;
        format!("_temp{}", VAR_NUM)
    }
}

static mut VAR_NUM3: i64 = 0;
static mut VAR_NUM2: i64 = 0;
static mut VAR_ELSE: i64 = 0;

fn create_beginif() -> String {
  unsafe {
    VAR_NUM3 += 1;
    format!(":iftrue{}",VAR_NUM3)
  }
}

fn create_endif() -> String {
  unsafe {
    format!(":endif{}",VAR_NUM3)
  }
}

fn create_else() -> String {
  unsafe {
    VAR_ELSE += 1;
    format!(":else{}",VAR_ELSE)
  }
}

fn create_begin() -> String {
  unsafe {
      VAR_NUM2 += 1;
      format!(":beginningloop{}", VAR_NUM2)
  }
}

fn create_end() -> String {
  unsafe {
      format!(":endloop{}", VAR_NUM2)
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

fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
let mut func_table: Vec<String> = vec![];
let mut ir_code: String = String::from("");
  loop {
      match parse_function(tokens, index,&mut func_table)? {
      None => {
          break;
      }
      Some(function_ir_code) => {
        ir_code += &function_ir_code;
      }
      }
  }
  let main = String::from("main");
  if !find_symbol(&func_table, &main) {
    return Err(format!("Error. no main function defined"));
  };

  return Ok(ir_code);
}

fn find_symbol(symbol_table: &Vec <String>, symbol: &String) -> bool {
  for symbol_in_table in symbol_table {
    if symbol_in_table.eq(symbol) {
      return true;
    }
  }
  return false;
}



fn parse_function(tokens: &Vec<Token>, index: &mut usize, func_table: &mut Vec<String>) -> Result<Option<String>, String> {

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

  let mut function_code:String = String::from("");
  let mut symbol_table: Vec<String> = vec![];
  let mut array_table: Vec<String> = vec![];
  let mut int_table: Vec<String> = vec![];
  let mut loop_table: Vec<String> = vec![];

  match next_result(tokens, index)? {
  Token::Ident(identifier_name) => {
    if find_symbol(&func_table, identifier_name) {
      return Err(format!("Error. found a duplicate function {identifier_name}"));
    };

    for symbol in func_table.iter() {
      println!("{symbol}\n");
      }

    func_table.push(identifier_name.clone());

    function_code += &format!("%func {identifier_name}(");
  }, 
  _  => {return Err(String::from("functions must have a function identifier"));} 
  }

  if !matches!( next_result(tokens, index)?, Token::LeftParen) {
      return Err(String::from("expected '(' in function"));
  }


  loop {
    match peek(tokens, *index) {
      None => {return Ok(None)}
      Some(token) => {
        match token {
        Token::RightParen => {
            break;
        }
          _ => {}
        }
      }
    }
      match parse_declaration(tokens, index, &mut symbol_table, func_table, &mut array_table, &mut int_table)? {
      None => {
        return Ok(None);
      }
      Some(decl) => {
        function_code += &format!("{decl}, ");
        match peek(tokens, *index) {
          None => {return Ok(None)}
          Some(token) => {
            match token {
            Token::Comma => {
              *index += 1;
              match peek_result(tokens, *index)? {
                Token::RightParen => {
                  return Err(String::from("Expected ')' to finish function declarations but got ',' "));
                }
                _ => {}
              }
            }
            _ => {}
          }
         }
        }
      }
      }
    }


  match next_result(tokens, index)? {
    Token::RightParen => {}, 
    _  => {return Err(String::from("expected ')' in function"));}
    }
    function_code += &format!(")\n");

  if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
      {return Err(String::from("expected '{' in function"));}
  }

  loop {
      match parse_statement(tokens, index, &mut symbol_table,func_table, &mut array_table, &mut loop_table)? {
      None => {
        break;
      }
      Some(statements_code) => {
        function_code += &statements_code;
      }
      }
  }

  if !matches!(next_result(tokens, index)?, Token::RightCurly) {
    return Err(String::from("expected '}' in function"));
  }
  function_code += "%endfunc\n";
  return Ok(Some(function_code));
}



fn parse_declaration(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<String>, _func_table: &mut Vec<String>, array_table: &mut Vec<String>, _loop_table: &mut Vec<String>) -> Result<Option<String>, String> {
  //let mut decl:String;
  //let mut symbol_table: Vec<String> = vec![];
  match next(tokens, index) {
    None => {
        return Ok(None);
    }
    Some(token) => {
      if !matches!(token, Token::Int) {
          return Err(String::from("declarations must begin with int"));}
      } 
    }
  let mut decl = String::from("%int");
  match peek(tokens, *index) {
    None => {
      return Ok(None);
    }
    Some(token) => {
      match token {
      //int [2] a;
      Token::LeftBracket => {
        let src: String;
        *index += 1;
        match next_result(tokens, index)? {
          Token::Num(num) => {
              if *num <= 0 {
                return Err(format!("Error. Declaring an array of size {num} which is <= 0"));
              }

            let e = Expression {
              code: String::from(""),
              name: format!("{num}"),
            };
            src = e.name;
          }
          _ => {return Err(String::from("expected [number] in declaration"));}
          };
          if !matches!(next_result(tokens, index)?, Token::RightBracket) {
            {return Err(String::from("expected ']' in the declaration"));}
          }
          let ident = match next_result(tokens, index)? {
            Token::Ident(ident) => {ident}

            _  => {return Err(String::from("expected '[num]' or an identifier in decalration"));}
            };

            if find_symbol(&symbol_table, ident) {
              return Err(format!("Error. found a duplicate variable {ident}"));
            };

            for symbol in symbol_table.iter() {
              println!("{symbol}\n");
              }

            symbol_table.push(ident.clone());
            array_table.push(ident.clone());


            decl += &format!("[] {ident}, {src}");
            return Ok(Some(decl))
        }
        _ => {}
      }
    }
  }
  //int a
  let ident = match next_result(tokens, index)? {
    Token::Ident(ident) => {ident},
    _  => {return Err(String::from("expected '[num]' or an identifier in decalration"));}
    };
    if find_symbol(&symbol_table, ident) {
      return Err(format!("Error. found a duplicate variable {ident}"));
    };

    for symbol in symbol_table.iter() {
      println!("{symbol}\n");
      }

    symbol_table.push(ident.clone());
  // int a => %int a
  decl += &format!(" {ident}");
  return Ok(Some(decl))
}

fn parse_var(tokens: &Vec<Token>, index: &mut usize,symbol_table: &mut Vec<String>, func_table: &mut Vec<String>, array_table: &mut Vec<String>, loop_table: &mut Vec<String>) -> Result<Expression, String> {
  let mut e = Expression {code: String::from(""), name: String::from(""),};
  let src:String;
  match next(tokens, index) {
    None => {return Err(String::from("expected identifier in var but saw none"));}
    Some(token) => {
      match token {
        Token::Ident(ident) => {
            e.code = String::from("");
            e.name = ident.clone();
          src = format!("{}", e.name);
        }
        _ => {return Err(String::from("expected identifier in var"));}
    }
  }
}
  match peek(tokens, *index) {
    None => {Ok(e)}
    Some(token) => {
      match token {
        Token::LeftBracket => {
          for symbol in array_table.iter() {
            println!("Array_table: {symbol}\n");
          }
          if !find_symbol(&array_table, &e.name) {
            return Err(format!("Error. type mismatch: using int as array in var: {}", e.name));
          };
          *index += 1;
          let e2 = parse_expression(tokens, index, symbol_table,func_table, array_table, loop_table)?;
          if !matches!(next_result(tokens, index)?, Token::RightBracket) {
            return Err(String::from("expected ']' in var"))
          }
          let src2 = e2.name;
          e.name = format!("[{src} + {src2}]",);
          e.code += &e2.code;
        }
        _ => {if find_symbol(&array_table, &e.name) {
          return Err(format!("Error. type mismatch: using array as int in var: {}", e.name));
        };}
      }
      //v += &format!("\n");
      return Ok(e);
    }
  }
}

fn parse_while_loop(tokens: &Vec<Token>, index: &mut usize,symbol_table: &mut Vec<String>, func_table: &mut Vec<String>, array_table: &mut Vec<String>, loop_table: &mut Vec<String>) -> Result<Option<String>, String> {
  match peek(tokens, *index) {
    None =>  {
      return Ok(None);
    }
    Some(token) => {
      if !matches!(token, Token::While) {
        return Err(String::from("while loops must begin with while keyword"));
      }
      
      let begin = create_begin();
      let end = create_end();
      let mut code = format!("{}\n", begin);
      loop_table.push(begin.clone());
      *index += 1;
      let expr = parse_bool_expr(tokens, index, symbol_table,func_table, array_table, loop_table)?;
      code += &expr.code;
      code += &format!("%branch_ifn {}, {}\n", expr.name, end);

      parse_bool_expr(tokens, index, symbol_table,func_table, array_table, loop_table)?;
      if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
        return Err(String::from("missing '{' in while loop"));
      }
      
      loop {
        match parse_statement(tokens, index,symbol_table,func_table, array_table, loop_table)? {
        None => {
            break;
        }
        Some(statement) => {
          code += &statement;
        }
        }
      }

      if !matches!(next_result(tokens, index)?, Token::RightCurly) {
        return Err(String::from("Missing '}' in while loop"));
      }

      code += &format!("%jmp {}\n", begin);
      code += &format!("{}\n", end);
      loop_table.push(end.clone());
      //let codenode = Some(code);

      return Ok(Some(code));
    }
  }
}

fn parse_if(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<String>, func_table: &mut Vec<String>, array_table: &mut Vec<String>, loop_table: &mut Vec<String>) -> Result<Option<String>, String> {
  match next(tokens, index) {
    None =>  {
      return Ok(None);
    }
    Some(token) => {
      if !matches!(token, Token::If) {
        return Err(String::from("If statements must begin with if keyword"));
      }

      let begin1 = create_beginif();
      let end1 = create_endif();
      let else1 = create_else();
      let mut code:String= String::from("");
      let expr = parse_bool_expr(tokens, index, symbol_table,func_table, array_table, loop_table)?;

      code += &expr.code;
      code += &format!("%branch_if {}, {}\n", expr.name, begin1);
      code += &format!("%jmp {}\n", else1);
      code += &format!("{}\n", begin1);
      
      if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
        return Err(String::from("missing '{' in if statement"));
      }
      
      loop {
        match parse_statement(tokens, index, symbol_table,func_table, array_table, loop_table)? {
        None => {
            break;
        }
        Some(y) => {
          code += &y;
        }
        }
      }

      if !matches!(next_result(tokens, index)?, Token::RightCurly) {
        return Err(String::from("Missing '}' in if statement"));
      }

      code += &format!("%jmp {}\n", end1);
      code += &format!("{}\n", else1);

      match peek(tokens, *index) {
        None => {return Ok(None)}
        Some(token) => {
          match token {
            Token::Else => {
              *index += 1;
              if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
                return Err(String::from("missing '{' in else statement"));
              }
              parse_statement(tokens, index, symbol_table,func_table, array_table, loop_table)?;
              if !matches!(next_result(tokens, index)?, Token::RightCurly) {
                return Err(String::from("missing '}' in else statement"));
              }
            }
            _ => {}
          }
          code += &format!("{}\n",end1);
          return Ok(Some(code))
        }
      }
    }
  }
}


fn parse_bool_expr(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<String>, func_table: &mut Vec<String>, array_table: &mut Vec<String>, loop_table: &mut Vec<String>) -> Result<Expression, String> {

  let mut expr = Expression {
    code: String::from(""),
    name: String::from(""),
  };

  match peek(tokens, *index) {
    None => {
      return Ok(expr);
    }
    Some(_) => {
      //let mut expr: String;
      let expr2 = parse_expression(tokens, index, symbol_table,func_table, array_table, loop_table)?;
      expr.code += &expr2.code;
      let opcode = match peek_result(tokens, *index)? {
        Token::Less => "%lt",
        Token::Greater => "%gt",
        Token::LessEqual => "%le",
        Token::GreaterEqual => "%ge",
        Token::Equality => "%eq",
        Token::NotEqual => "%neq",
        _ => {return Err(String::from("expected comparison symbol in bool expression"));}
      };
      *index += 1;
      //let mut expr2: String;
      let expr3 = parse_expression(tokens, index, symbol_table,func_table, array_table, loop_table)?;
      let temp = create_temp();
      let src1 = expr2.name;
      let src2 = expr3.name;
      expr.code += &format!("%int {temp}\n");
      expr.code += &expr3.code;
      expr.code += &format!("{opcode} {temp}, {src1}, {src2}\n");
      expr.name = temp;
      return Ok(expr);
    }
  }
}

fn parse_mult_expr(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<String>, func_table: &mut Vec<String>, array_table: &mut Vec<String>, loop_table: &mut Vec<String>) -> Result<Expression, String> {

  /* match peek(tokens, *index) {
    None => {
      return Ok(None);
    }
    Some(_) => { */
      let mut e = parse_term(tokens,index, symbol_table,func_table, array_table, loop_table)?;
      loop {
        match peek(tokens, *index) {
          None => {
            break;
          }
          Some(token) => {
            let opcode = match token {
              Token::Multiply => "%mult" ,
              Token::Divide => "%div",
              Token::Modulus => "%mod",
              _ => {break;}
            };
            *index += 1;
            let node = parse_term(tokens, index, symbol_table,func_table, array_table, loop_table)?;
            let temp = create_temp();
            let src1 = e.name;
            let src2 = node.name;
            e.code += &format!("%int {temp}\n");
            e.code += &node.code;
            e.code += &format!("{opcode} {temp}, {src1}, {src2}\n");
            e.name = temp;
          }
        }
      }
      return Ok(e)
    }
/*   }
} */

fn parse_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<String>, func_table: &mut Vec<String>, array_table: &mut Vec<String>, loop_table: &mut Vec<String>) -> Result<Option<String>, String> {
  match peek(tokens, *index) {
  None => {
      return Ok(None);
  }

  Some(token) => {
      let mut code:String= String::from("");
      match token {
      Token::RightCurly => {
        return Ok(None);
      }

      Token::Int => {
        match parse_declaration(tokens, index, symbol_table, func_table, array_table, loop_table)? {
          None => {
            return Ok(None)
          }
          Some(declaration) => {
            code += &declaration;
            code += &format!("\n");
          }
        }
      }
      // a = 1 + 1
      // a = 0
      Token::Ident(dest) => {
        if !find_symbol(&symbol_table, dest) {
          return Err(format!("Error. undeclared var: {dest}"));
        };

          let v = parse_var(tokens, index, symbol_table,func_table, array_table, loop_table)?;
          if !matches!(next_result(tokens, index)?, Token::Assign) {
              return Err(String::from("expected '=' assignment operator"));
          }
          let expression = parse_expression(tokens, index, symbol_table,func_table, array_table, loop_table)?;
          let src2 = expression.name;
          //a = 0
          let src = &v.name;
          let temp = create_temp();
          code += &v.code;
          code += &expression.code;
          //println!("{}, {}\n-------\n", src, src2);
          //println!("CODE: {}, {}\n-------\n", expression.code, v.code);
          code += &format!("%mov {src}, {src2}\n");
          code += &format!("%int {temp}\n");
          code += &format!("%mov {temp}, {src}\n");
      }

      Token::Return => {
          *index += 1;
          let e = parse_expression(tokens, index, symbol_table,func_table, array_table, loop_table)?;
          let src = e.name;
          code += &e.code;
          code += &format!("%ret {src}\n");
      }

      Token::Print => {
        *index += 1;
        let expression = parse_term(tokens, index, symbol_table,func_table, array_table, loop_table)?;
        //todo!()
        let name = expression.name;
        code += &expression.code;
        code += &format!("%out {name}\n");
      }

      Token::Read => {
        *index += 1;
        parse_term(tokens, index, symbol_table,func_table, array_table, loop_table)?;
        todo!()
      }

      Token::Break | Token::Continue => {
        *index += 1;
      }

      Token::While => {
        match parse_while_loop(tokens, index, symbol_table,func_table, array_table, loop_table)? {
          None => {
            //Ok(None);
          }
          Some(w) => {
            code += &w;
        }
        }
        return Ok(Some(code));
      }
      Token::If => {
        match parse_if(tokens, index, symbol_table,func_table, array_table, loop_table)? {
          None => {
            //Ok(None);
          }
          Some(x) => {
            code += &x;
          }
        }
        return Ok(Some(code));
      }

      _ => {
           return Err(String::from("invalid statement"));
      }

      }
      if !matches!(next_result(tokens, index)?, Token::Semicolon) {
          return Err(String::from("expected ';' closing statement"));
      }
      //code += "%endfunc\n"; // fix to actual output !!!!!!!!Mellohi708515*
      
      
      return Ok(Some(code));
  }

  }
}

struct Expression {
  code: String,
  name: String,
}

fn parse_expression(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<String>, func_table: &mut Vec<String>, array_table: &mut Vec<String>, loop_table: &mut Vec<String>) -> Result<Expression, String> {
  let mut e = parse_mult_expr(tokens, index, symbol_table,func_table, array_table, loop_table)?;
  loop {
    let opcode = match peek_result(tokens, *index)? {
      Token::Plus => "%add",
      Token::Subtract => "%sub",
      _ => { break; }
      };
      *index += 1;
      let m_expr = parse_mult_expr(tokens, index, symbol_table,func_table, array_table, loop_table)?;
      let temp = create_temp();
      let src1 = e.name;
      let src2 = m_expr.name;
      e.code += &format!("%int {temp}\n");
      e.code += &m_expr.code;
      e.code += &format!("{opcode} {temp}, {src1}, {src2}\n");
      e.name = temp;
  }
  return Ok(e);
}

fn parse_term(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<String>, func_table: &mut Vec<String>, array_table: &mut Vec<String>, loop_table: &mut Vec<String>) -> Result<Expression, String> {

  match next_result(tokens, index)? {
      Token::Num(num) => {
        let e = Expression {
          code: String::from(""),
          name: format!("{num}"),
        };
        return Ok(e);
      }
      Token::LeftParen => {
        let e = parse_expression(tokens, index, symbol_table,func_table, array_table, loop_table)?;
        match next_result(tokens, index)? {
          Token::RightParen => {}
          _ => {return Err(String::from("term missing closing ')' "));}
        }
        return Ok(e)
      }
      Token::Ident(ident) => {
        if !find_symbol(&symbol_table, ident) && !find_symbol(&func_table, ident) {
          return Err(format!("Error. undeclared var or function: {ident}"));
        };
        let  mut e = Expression {
          code: String::from(""),
          name: ident.clone(),
        };
           match peek_result(tokens, *index)? {
            //a[3];
            Token::LeftBracket => {
              for symbol in array_table.iter() {
                println!("Array_table: {symbol}\n");
              }
              if !find_symbol(&array_table, ident) {
                return Err(format!("Error. type mismatch: using int as array in var: {ident}"));
              };
              *index += 1;
              e = parse_expression(tokens, index, symbol_table, func_table, array_table, loop_table)?;
              match next_result(tokens, index)? {
                Token::RightBracket => {}
                _ => {return Err(String::from("term missing closing ']'"));}
              }
              let src = e.name;
              e.name = format!("[array+ {src}]");
              let temp = create_temp();
              e.code += &format!("%int {temp}\n");
              e.code += &format!("%mov {temp}, [array + {src}]\n");
              e.name = temp;
            }

            //add(a,b)
            Token::LeftParen => {
              *index += 1;
              loop {
                match peek_result(tokens, *index)? {
                  Token::RightParen => {
                    break;
                  }
                  _ => {}
                }
                let e2 = parse_expression(tokens, index, symbol_table,func_table, array_table, loop_table)?;
                e.code += &e2.code;
                //println!("{}--------------------------\n", e2.name);
                match peek_result(tokens, *index)? {
                  Token::Comma => {
                    *index += 1;
                    
                    let e3 = parse_expression(tokens, index, symbol_table,func_table, array_table, loop_table)?;
                    
                    e.code += &e3.code;
                    let temp = create_temp();
                    e.code += &format!("%int {temp}\n");
                    e.code += &format!("%call {temp}, {ident}({}", e2.name);
                    e.code += &format!(", {}", e3.name);
                    e.name = temp;
                    //println!("{}++++++++++++++++++++++++++\n", e3.code);
                    //println!("{}++++++++++++++++++++++++++\n", e3.name);
                  }
                  _ => {}
                }
              }
              match next_result(tokens, index)? {
                Token::RightParen => {e.code += &format!(")\n");}
                _ => {return Err(String::from("missing closing ')' in \"ident ( expr? )\" "));}
              }
            }
            _ => {
              for symbol in array_table.iter() {
                println!("Array_table2: {symbol}\n");
              }
              if find_symbol(&array_table, ident) {
                return Err(format!("Error. type mismatch: using array as int for var: {ident}"));
              };
            }
          }
          return Ok(e);
      }
  
      _ => {
          return Err(String::from("invalid term expression"));
      } 
  };
  
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
        //parse_statement(&tokens, &mut 0).unwrap();

        let tokens = lex("b = 1 / 2;").unwrap();
        //parse_statement(&tokens, &mut 0).unwrap();


        // test errors. missing semicolon
        let tokens = lex("b = 1 / 2").unwrap();
        //assert!(matches!(parse_statement(&tokens, &mut 0), Err(_)));

    }

}