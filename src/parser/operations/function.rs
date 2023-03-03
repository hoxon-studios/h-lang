use crate::parser::{
    tokens::{Block, Declaration, Definition, Expression, Function, Token, Value},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_function(&mut self) -> Result<(), String> {
        let Some(Token::Value(body)) = self.output.pop() else {
            return Err("Invalid operand".to_string());
        };
        let body = match &body {
            Value::Result(result) => match &**result {
                Expression::Block(body) => body.clone(),
                _ => Block {
                    body: vec![],
                    result: body.clone(),
                },
            },
            _ => Block {
                body: vec![],
                result: body.clone(),
            },
        };

        let Some(parameters) = self.output.pop() else {
            return Err("Invalid operand".to_string());
        };
        let parameters = match parameters {
            Token::Set(parameters) => parameters
                .iter()
                .map(|p| match p {
                    Token::Declaration(declaration) => declaration.clone(),
                    _ => panic!("Invalid operand"),
                })
                .collect::<Vec<Declaration>>(),
            Token::Value(Value::Unit) => vec![],
            Token::Declaration(declaration) => vec![declaration],
            _ => return Err("Invalid operand".to_string()),
        };
        let Some(Token::Value(Value::Label(label))) = self.output.pop() else {
            return Err("Invalid operand".to_string());
        };

        self.output
            .push(Token::Definition(Definition::Function(Function {
                export: false,
                label,
                parameters,
                body,
            })));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        tokens::{
            Addition, Assignment, Block, Declaration, Definition, Expression, Function, LabelType,
            Statement, Token, Value,
        },
        Parser,
    };

    #[test]
    fn it_parses_function_definition() {
        let code = "\
            fn some(x: usize, y: usize)
                a: usize = 3;
                a + x + y";
        // ACT
        let result = Parser::parse(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            vec![Token::Definition(Definition::Function(Function {
                export: false,
                label: "some",
                parameters: vec![
                    Declaration {
                        label: "x",
                        pointer: false,
                        _type: LabelType::Usize,
                    },
                    Declaration {
                        label: "y",
                        pointer: false,
                        _type: LabelType::Usize
                    }
                ],
                body: Block {
                    body: vec![Statement::Assignment(Assignment {
                        address: Box::new(Token::Declaration(Declaration {
                            label: "a",
                            pointer: false,
                            _type: LabelType::Usize
                        })),
                        value: Value::Constant("3")
                    })],
                    result: Value::Result(Box::new(Expression::Addition(Addition {
                        left: Value::Result(Box::new(Expression::Addition(Addition {
                            left: Value::Label("a"),
                            right: Value::Label("x")
                        }))),
                        right: Value::Label("y")
                    })))
                }
            }))]
        );
    }
}
