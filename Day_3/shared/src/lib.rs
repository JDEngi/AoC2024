
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Instruction {
    NOP,
    MUL(i32, i32)
}

impl Instruction {
    fn build(tokens: Vec<Tokens>) -> Instruction {
        if tokens.len() < 7 {return Instruction::NOP}

        let mut token_iter = tokens.iter();
        let operator = token_iter.next().unwrap();
        token_iter.next().unwrap();
        let a = token_iter.next().unwrap();
        token_iter.next().unwrap();
        let b = token_iter.next().unwrap();

        let Tokens::Number(a) = a;

        match operator {
            Tokens::String(op_str) => {
                match op_str.as_str() {
                    "MUL" => Instruction::MUL(a, b),
                    _ => Instruction::NOP
                }
            }
            _ => {}
        }        
    }
}

impl Instruction {
    fn execute(&self) -> Option<i32> {
        match self {
            Instruction::NOP => None,
            Instruction::MUL(a, b) => Some(a * b)
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Tokens {
    String(String),
    Number(i32),
    OpenBracket,
    CloseBracket,
    Comma,
    Other
}

enum LexerState {
    Start,
    Operator,
    OpenBracket,
    CloseBracket,
    Number,
    Comma,
    End
}

struct Parser;
enum ParserState {
    START,
    STRING,
    NUMBER,
    SYMBOL,
    END
}

impl ParserState {
    fn get_state(ch: char) -> ParserState {
        match ch {
            '0'..='9' => ParserState::NUMBER,
            'A'..'z' => ParserState::STRING,
            ch if "!@#$%^&*-=_+()[]{};'\\:\"|,./<>?`~ \n\t".contains(ch) => ParserState::SYMBOL,
            '\0' => ParserState::END,
            _ => panic!("Parser not equiped to handle character: {:?}", ch)
        }  
    }
}


impl Parser {
    fn parse(input_str: &str) -> Vec<Tokens>{
        let mut state = ParserState::START;
        let mut result: Vec<Tokens> = vec![];

        let mut accumulator: Vec<char> = vec![];

        for c in input_str.chars() {
            state = match state {
                ParserState::START => {
                    accumulator.push(c);
                    ParserState::get_state(c)         
                },
                ParserState::STRING => {
                    let next_state = ParserState::get_state(c);
                    match next_state {
                        ParserState::STRING => {
                            accumulator.push(c);
                        }

                        _ => {
                            result.push(Tokens::String(accumulator.iter().collect()));
                            accumulator.clear();
                            accumulator.push(c)
                        }
                    }

                    next_state
                },
                ParserState::NUMBER => {
                    let next_state = ParserState::get_state(c);
                    match next_state {
                        ParserState::NUMBER => {
                            accumulator.push(c);
                        },
                        _ => {
                            result.push(Tokens::Number(accumulator.iter().fold(0, |acc, x| acc * 10 + x.to_digit(10).unwrap() as i32)));
                            accumulator.clear();
                            accumulator.push(c)
                        }
                    }

                    next_state
                },
                ParserState::SYMBOL => {
                    let next_state = ParserState::get_state(c);

                    let symbol = accumulator.first().unwrap();
                    match symbol {
                        '(' => {
                            result.push(Tokens::OpenBracket);
                        }
                        ')' => {
                            result.push(Tokens::CloseBracket);
                        }
                        ',' => {
                            result.push(Tokens::Comma);
                        }
                        _ => result.push(Tokens::Other)
                    }

                    accumulator.clear();
                    accumulator.push(c);

                    next_state
                },
                ParserState::END => break,
            }
        }

        result
    }
}



pub fn parse_input(input: &str) -> Vec<Instruction> {
    let tokens = Parser::parse(input);
    let mut lexer_state = LexerState::Start;
    let mut result: Vec<Instruction> = vec![];

    let mut accumulator: Vec<Tokens>

    for token in tokens {
        lexer_state = match lexer_state {
            LexerState::Start => {
                match token {
                    
                }
            },
            LexerState::Operator => todo!(),
            LexerState::OpenBracket => todo!(),
            LexerState::CloseBracket => todo!(),
            LexerState::Number => todo!(),
            LexerState::Comma => todo!(),
            LexerState::End => todo!(),
        }
    }

    vec![]
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_instructions() {
        assert_eq!(Instruction::NOP.execute(), None);
        assert_eq!(Instruction::MUL(3, 4).execute(), Some(12));
        assert_eq!(Instruction::MUL(-3, 4).execute(), Some(-12));
    }

    #[test]
    fn test_lexing() {
        let data = read_to_string("../data/test1.txt").unwrap();
        let data = parse_input(data.as_str());
        let test_data = vec![
            Instruction::MUL(2, 4),
            Instruction::MUL(5, 5),
            Instruction::MUL(11, 8),
            Instruction::MUL(8, 5),
        ];

        assert_eq!(data, test_data, "Parser output does not match expected output");
    }

    #[test]
    fn test_parsing() {
        let data = "MUL(1,2)\0";
        let tokens = Parser::parse(data);
        let test_data = vec![
            Tokens::String("MUL".to_string()), 
            Tokens::OpenBracket, 
            Tokens::Number(1),
            Tokens::Comma,
            Tokens::Number(2),
            Tokens::CloseBracket
        ];

        assert_eq!(tokens, test_data, "Parsing failed");
    }
}
