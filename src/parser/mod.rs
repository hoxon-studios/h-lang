use self::{context::Context, cursor::*, operations::*, tokens::*};

pub mod context;
pub mod cursor;
pub mod operations;
pub mod resolve;
pub mod tokens;

#[derive(Debug)]
pub struct Parser<'a> {
    context: Context,
    output: Vec<Token<'a>>,
    operators: Vec<Operator>,
}
impl<'a> Parser<'a> {
    pub fn parse(code: &str) -> String {
        let mut parser = Parser {
            context: Context::init(),
            output: vec![],
            operators: vec![],
        };
        let mut cursor = code.clone();
        'outer: loop {
            cursor = skip_space(cursor);
            if let Some(code) = eat_empty(cursor) {
                parser.output.push(Token::Unit);
                cursor = code;
            } else if let Some((code, operator)) = eat_operator(cursor) {
                match &operator {
                    Operator::LeftParenthesis => {
                        parser.operators.push(operator);
                    }
                    Operator::RightParenthesis => {
                        while let Some(operator) = parser.operators.pop() {
                            match operator {
                                Operator::LeftParenthesis => break,
                                Operator::Operation(operation) => {
                                    parser.apply(operation);
                                }
                                Operator::RightParenthesis => {
                                    panic!("Open parenthesis missing")
                                }
                            }
                        }
                    }
                    Operator::Operation(operation) => loop {
                        let pop = if let Some(stack) = parser.operators.last() {
                            match stack {
                                Operator::LeftParenthesis => false,
                                Operator::RightParenthesis => panic!("Invalid operator"),
                                Operator::Operation(stack) => {
                                    let stack_precedence = stack.precedence();
                                    let current_precedence = operation.precedence();
                                    let left_associated = operation.left_associated();

                                    (stack_precedence > current_precedence)
                                        || (stack_precedence == current_precedence
                                            && left_associated)
                                }
                            }
                        } else {
                            false
                        };

                        if pop {
                            match parser.operators.pop().expect("Operator not found") {
                                Operator::LeftParenthesis => panic!("Invalid operator"),
                                Operator::RightParenthesis => panic!("Invalid operator"),
                                Operator::Operation(operation) => {
                                    parser.apply(operation);
                                }
                            }
                        } else {
                            parser.operators.push(operator.clone());
                            break;
                        }
                    },
                }

                cursor = code;
            } else if let Some((code, number)) = eat_number(cursor) {
                parser.output.push(Token::Constant(Constant(number)));
                cursor = code;
            } else if let Some((code, string_literal)) = eat_string(cursor) {
                parser
                    .output
                    .push(Token::String(StringLiteral(string_literal)));
                cursor = code;
            } else if let Some((code, id)) = eat_id(cursor) {
                parser.output.push(Token::Id(Id(id)));
                cursor = code;
            } else {
                break 'outer;
            }
        }

        while let Some(operator) = parser.operators.pop() {
            match operator {
                Operator::LeftParenthesis => panic!("Closing parenthesis missing"),
                Operator::RightParenthesis => panic!("Invalid operator"),
                Operator::Operation(operation) => {
                    parser.apply(operation);
                }
            }
        }

        return parser
            .output
            .iter()
            .map(|token| match token {
                Token::Result(Code(value)) => value.clone(),
                Token::Statement(Statement {
                    body: Code(body), ..
                }) => body.clone(),
                Token::Definition(Definition {
                    definition: Code(definition),
                    ..
                }) => definition.clone(),
                _ => panic!("Invalid token"),
            })
            .collect::<Vec<String>>()
            .join("\n\n");
    }

    pub fn apply(&mut self, operation: Operation) {
        match operation {
            Operation::Reference => self.parse_reference(),
            Operation::Visibility { export: true } => self.parse_export(),
            Operation::Visibility { export: false } => {}
            Operation::Let => self.parse_declaration(),
            Operation::Group => self.parse_group(),
            Operation::Sequence => self.parse_block(),
            Operation::Assign => self.parse_assignment(),
            Operation::Addition => self.parse_addition(),
            Operation::Call => self.parse_call(),
            Operation::Function => self.parse_function(),
            Operation::Dereference => self.parse_dereference(),
            Operation::If => self.parse_if_conditional(),
            Operation::Else => self.parse_else_conditional(),
            Operation::Loop => self.parse_loop(),
            Operation::Break => self.parse_break(),
            Operation::String => self.parse_string(),
            Operation::BitwiseOr => self.parse_bitwise_or(),
            Operation::Struct => self.parse_struct(),
            Operation::Navigate => self.parse_navigation(),
        }
    }
}

pub fn eat_empty(code: &str) -> Option<&str> {
    if let Some(code) = eat_token(skip_space(code), "(") {
        if let Some(code) = eat_token(skip_space(code), ")") {
            Some(code)
        } else {
            None
        }
    } else {
        None
    }
}
