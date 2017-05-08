extern crate regex;

use regex::Regex;

// 这个 macro 定义的太爽了
macro_rules! do_eval {
    ( $( $x:expr ),*  ;$y: expr ) => {
        {
            $(
                $x;
            )*
            $y
        }
    };
}

macro_rules! acc {
    ($c: ident, $counter: ident, $chars: ident, $reg: ident) => {
        {
            let mut value = $c.to_string();
            while do_eval!($counter+=1, $c = $chars[$counter]; $reg.is_match(&$c.to_string())){
                value += &$c.to_string();
            }
            value
        }
    }
}



fn main() {
    let input = "(add 2 (subtract 4 2))";
    let tokens = tokenizer(&input);
    println!("tokens length {}", tokens.len());
}

// todo: move this into struct Token
// const input  = '(add 2 (subtract 4 2))';
fn tokenizer(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut counter = 0;
    let char_reg = Regex::new(r"(?i)[a-z]").unwrap();
    let num_reg = Regex::new(r"\d").unwrap();
    let chars: Vec<char> = input.chars().collect();

    while counter != chars.len() {
        let mut c = chars[counter];

        if c == "(".chars().nth(0).unwrap() {
            let token = Token {
                tpe: TokenType::Paren,
                value: "(".to_string(),
            };
            tokens.push(token.clone());
            counter += 1;
            println!("{:?}, {:?}", TokenType::Paren, token);
        } else if c == ")".chars().nth(0).unwrap() {
            let token = Token {
                tpe: TokenType::Paren,
                value: ")".to_string(),
            };
            tokens.push(token.clone());
            counter += 1;
            println!("{:?}, {:?}", TokenType::Paren, token);
        } else if char_reg.is_match(&c.to_string()) {
            // let mut value = c.to_string();
            // while do_eval!(counter+=1, c = chars[counter]; char_reg.is_match(&c.to_string())){
            //     value += &c.to_string();
            // }
            let token = Token {
                tpe: TokenType::Name,
                value: acc!(c, counter, chars, char_reg),
            };
            tokens.push(token.clone());
            println!("{:?}, {:?}", TokenType::Name, token);
        } else if num_reg.is_match(&c.to_string()) {
            let token = Token {
                tpe: TokenType::Number,
                value: acc!(c, counter, chars, num_reg),
            };
            tokens.push(token.clone());
            println!("{:?}, {:?}", TokenType::Number, token);
        } else if c == "\"".chars().nth(0).unwrap() {
            counter += 1;
            let mut value = c.to_string();
            while do_eval!(counter+=1, c = chars[counter]; c != "\"".chars().nth(0).unwrap()) {
                value += &c.to_string();
            }
            let token = Token {
                tpe: TokenType::Strin,
                value: value,
            };
            tokens.push(token.clone());
            println!("{:?}, {:?}", TokenType::Strin, token);
            counter += 1;//skip ending quote off
        } else if c.is_whitespace() {
            println!("{:?}", "empty token");
            counter += 1;
        } else {
            panic!("{:?}， {:?}", "sorry unexpect character", c);
        }
    }

    tokens
}


/*
const tokens = [
  { type: 'paren',  value: '('        },
  { type: 'name',   value: 'add'      },
  { type: 'number', value: '2'        },
  { type: 'paren',  value: '('        },
  { type: 'name',   value: 'subtract' },
  { type: 'number', value: '4'        },
  { type: 'number', value: '2'        },
  { type: 'paren',  value: ')'        },
  { type: 'paren',  value: ')'        }
];

const ast = {
  type: 'Program',
  body: [{
    type: 'CallExpression',
    name: 'add',
    params: [{
      type: 'NumberLiteral',
      value: '2'
    }, {
      type: 'CallExpression',
      name: 'subtract',
      params: [{
        type: 'NumberLiteral',
        value: '4'
      }, {
        type: 'NumberLiteral',
        value: '2'
      }]
    }]
  }]
};
*/

fn parser(input: Vec<Token>){

}

#[derive(Debug, Copy, Clone, PartialEq)]
enum TokenType {
    Paren,
    Name,
    Number,
    Strin,
}

#[derive(Debug)]
struct Token {
    tpe: TokenType,
    value: String,
}

impl Clone for Token {
    fn clone(&self) -> Token {
        Token {
            tpe: self.tpe,
            value: self.value.clone(),
        }
    }
}
// const input  = '(add 2 (subtract 4 2))';
// const output = 'add(2, subtract(4, 2));';
//
// const tokens = [
// { type: 'paren',  value: '('        },
// { type: 'name',   value: 'add'      },
// { type: 'number', value: '2'        },
// { type: 'paren',  value: '('        },
// { type: 'name',   value: 'subtract' },
// { type: 'number', value: '4'        },
// { type: 'number', value: '2'        },
// { type: 'paren',  value: ')'        },
// { type: 'paren',  value: ')'        }
// ];
//
#[cfg(test)]
mod tests {
    use super::{tokenizer, TokenType};


    #[test]
    fn test_tokenizer() {
        let input = "(add 2 (subtract 4 2))";
        let results = tokenizer(&input);
        assert_eq!(results.len(), 9);
        assert_eq!(results[1].tpe, TokenType::Name);
        assert_eq!(results[1].value, "add");
    }
}
