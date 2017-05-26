extern crate regex;
#[macro_use] extern crate log;
extern crate env_logger;


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
    env_logger::init().unwrap();
    debug!("this is a debug {}", "message");
    let input = "(add 2 (subtract 4 2))";
    let mut tokens = tokenizer(&input);
    debug!("tokens length {}", tokens.len());

    let node = parser(&mut tokens);
    debug!("parsed node is {:?}", node);

    let vs = vec![
        (AstNode::Program{body: vec![]}, AstVisitor{enter: Some(Box::new(|astnode, option_parent|{println!("{:?} {:?}", astnode, option_parent);})), exit: None}),
        (AstNode::CallExpression{name: String::from(""), params: vec![]}, AstVisitor{enter: Some(Box::new(|astnode, option_parent|{println!("\n\t CallExpression: node:{:?} \n\t parent: {:?}", astnode, option_parent);})), exit: None}),
        (AstNode::NumberLiteral{value: String::from("")}, AstVisitor{enter: Some(Box::new(|astnode, option_parent|{println!("\n\t NumberLiteral node:{:?} \n\t parent: {:?}", astnode, option_parent);})), exit: None}),
        (AstNode::StringLiteral{value: String::from("")}, AstVisitor{enter: Some(Box::new(|astnode, option_parent|{println!("\n\t StringLiteral node:{:?} \n\t parent: {:?}", astnode, option_parent);})), exit: None})
    ];

    let visitor_contaner = VistorContainer::new(vs);
    traverse(&node, &visitor_contaner);
}

// todo: move this into struct Token,
// replace debug with debug
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
            debug!("{:?}, {:?}", TokenType::Paren, token);
        } else if c == ")".chars().nth(0).unwrap() {
            let token = Token {
                tpe: TokenType::Paren,
                value: ")".to_string(),
            };
            tokens.push(token.clone());
            counter += 1;
            debug!("{:?}, {:?}", TokenType::Paren, token);
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
            debug!("{:?}, {:?}", TokenType::Name, token);
        } else if num_reg.is_match(&c.to_string()) {
            let token = Token {
                tpe: TokenType::Number,
                value: acc!(c, counter, chars, num_reg),
            };
            tokens.push(token.clone());
            debug!("{:?}, {:?}", TokenType::Number, token);
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
            debug!("{:?}, {:?}", TokenType::Strin, token);
            counter += 1;//skip ending quote off
        } else if c.is_whitespace() {
            debug!("{:?}", "empty token");
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

fn parser(input: &mut Vec<Token>)->AstNode{
    fn walk(counter: &mut usize, input: &Vec<Token>)->AstNode{
        assert!(counter < &mut input.len());
        // counter is &mut unsize type, Index<usize> is required, so *counter return a usize type.
        // token has to be a mutable type since we're changing it
        // & is because rust complain move T out of context, this is something i cant figure out, in the documentation
        // index(t)->&T, so how this is possible?
        let mut token = &input[*counter];
        match token.tpe {
            TokenType::Strin => AstNode::StringLiteral{value: token.value.clone()},
            TokenType::Number => AstNode::NumberLiteral{value: token.value.clone()},
            TokenType::Paren if token.value == "(" => {
                *counter += 1; //mutable reference need * to make a change
                token = &input[*counter];
                if token.tpe == TokenType::Name {
                    let mut params:Vec<AstNode> = vec![];
                    let name = token.value.clone();
                    *counter +=1;
                    token = &input[*counter];
                    while token.tpe != TokenType::Paren || (token.tpe == TokenType::Paren && token.value != ")")  {
                        let node = walk(counter, input);
                        params.push(node);
                        token = &input[*counter];
                        // this code isn't as clean as the original one, too clumsy it seems
                        if token.value == ")" { // nested parenthsis ends
                            continue;
                        } else {
                            *counter +=1;
                            token=&input[*counter];
                        }
                    }
                    *counter += 1; //skip off the close ) parenthsis
                    AstNode::CallExpression{
                        name: name,
                        params: params
                    }
                }else{
                    panic!("unexpect token type after paren {:?}", token);
                }
            },
            _ => panic!("unexpect token type: {:?} at unmatch", token)
        }
        
    }

    let mut body: Vec<AstNode> = vec![];
    let mut counter:usize = 0;
    while counter < input.len() {
        let node = walk(&mut counter, input);
        body.push(node);
        counter+=1;
    }
    AstNode::Program {
        body: body
    }
}


#[derive(Debug)]
enum AstNode {
    Program {body: Vec<AstNode>},
    CallExpression{name: String, params: Vec<AstNode>},
    NumberLiteral{value: String},
    StringLiteral{value: String}
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

struct AstVisitor {
    enter: Option<Box<Fn(&AstNode, &Option<&AstNode>)>>, // trait is unsized, has to be boxed here
    exit: Option<Box<Fn(&AstNode, &Option<&AstNode>)>>,
}

// compare Enum without fields
fn variant_eq(a: &AstNode, b: &AstNode) -> bool {
    match (a, b) {
        (&AstNode::Program{..}, &AstNode::Program{..}) => true,
        (&AstNode::CallExpression{..}, &AstNode::CallExpression{..}) => true,
        (&AstNode::NumberLiteral{..}, &AstNode::NumberLiteral{..}) => true,
        (&AstNode::StringLiteral{..}, &AstNode::StringLiteral{..}) => true,
        _ => false,
    }
}


struct VistorContainer{
    data: Vec<(AstNode,AstVisitor)>
}

impl VistorContainer {
    fn new(data: Vec<(AstNode,AstVisitor)>) -> VistorContainer{
        VistorContainer{data: data}
    }

    fn get(&self, target: &AstNode)-> Option<&AstVisitor>{
        for data in &self.data {
            if variant_eq(&data.0, target) { // https://stackoverflow.com/questions/32554285/compare-enums-only-by-variant-not-value
                return Some(&data.1);
            }
        }
        return None;
    }
}

fn traverse(ast: &AstNode, visitors: &VistorContainer){
    // recursive closure, in fact it's quite tricky to do recursive lamda in rust.
    // so i have to mannuly wrap up sorrounding environment variables. 
    struct Env<'a> {
        visitors: &'a VistorContainer
    }

    fn traverse_array(env: &Env, array: &Vec<AstNode>, parent: &Option<&AstNode>) {
        for ref node in array {
            traverse_node(env, node, parent);
        }
    }

    fn traverse_node(env: &Env, node: &AstNode, parent: &Option<&AstNode>) -> (){
        let ovisitor = env.visitors.get(node);
        if let Some(vi) = ovisitor {
            vi.enter.as_ref().map(|f| f(node, parent));
        }

        match node {
            &AstNode::Program{ref body} => traverse_array(env, body, &Some(node)),
            &AstNode::CallExpression{ref params, ..} => traverse_array(env, params, &Some(node)),
            &AstNode::NumberLiteral{..} => (),
            &AstNode::StringLiteral{..} => (),
        }

        if let Some(vi) = ovisitor {
           vi.exit.as_ref().map(|f| f(node, parent));
        }
    };
    
    let env = Env{visitors};
    traverse_node(&env, ast, &None);
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
