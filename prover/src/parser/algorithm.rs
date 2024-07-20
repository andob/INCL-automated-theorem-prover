use std::any;
use std::rc::Rc;
use anyhow::{anyhow, Context, Result};
use crate::codeloc;
use crate::formula::Formula;
use crate::logic::Logic;
use crate::parser::models::{OperatorPrecedence, Token, TokenCategory, TokenType};
use crate::parser::token_types::TokenTypeID;

pub struct LogicalExpressionParser {}
impl LogicalExpressionParser
{
    pub fn parse(logic : &Rc<dyn Logic>, text : &String) -> Result<Formula>
    {
        return LogicalExpressionParserImpl::parse(logic, text);
    }
}

struct LogicalExpressionParserInput
{
    text : String,
    token_types : Vec<TokenType>,
    tokens : Vec<Token>,
}

struct LogicalExpressionParserState
{
    current_index : usize,
}

struct LogicalExpressionParserImpl<'a>
{
    input : &'a LogicalExpressionParserInput,
    state : &'a mut LogicalExpressionParserState,
}

const REPLACE_TABLE : [&str; 48] =
[
    "∀", " ∀", "∃", " ∃", "(", " ( ", ")", " ) ", ", ", ",", "◇", " ◇ ", "□", " □ ",
    "~", " ~ ", "¬", " ¬ ", "!", " ! ", "&", " & ", "∧", " ∧ ", "|", " | ", "∨", " ∨ ",
    "→", " → ", "⇒", " ⇒ ", "⊃", " ⊃ ", "⥽", " ⥽ ", "↔", " ↔ ", "⇔", " ⇔ ", "≡", " ≡ ",
    "ᶠ", " ᶠ ", "ᵖ", " ᵖ ", "ᐅ", " ᐅ "
];

impl <'a> LogicalExpressionParserImpl<'a>
{
    fn parse(logic : &Rc<dyn Logic>, text : &String) -> Result<Formula>
    {
        let token_types = TokenType::get_types().context(codeloc!())?;

        let mut prepared_text = text.clone();
        for i in (0..REPLACE_TABLE.len()).step_by(2)
        {
            prepared_text = prepared_text.replace(REPLACE_TABLE[i], REPLACE_TABLE[i+1]);
        }

        let tokens : Vec<Token> = prepared_text.split(" ")
            .map(|word| word.trim()).filter(|word| !word.is_empty())
            .flat_map(|word| Self::get_tokens(word, &token_types))
            .collect();

        if tokens.is_empty()
        {
            return Err(anyhow!("Empty formula {}", text));
        }

        let legal_syntax_in_this_logic = logic.get_parser_syntax();
        for token in &tokens
        {
            if !legal_syntax_in_this_logic.contains(&token.type_id)
            {
                return Err(anyhow!("Invalid syntax: {} operation is not available in {}!", token.type_id, logic.get_name()));
            }
        }

        let parser_input = LogicalExpressionParserInput { text:text.clone(), token_types, tokens };
        let mut parser_state = LogicalExpressionParserState { current_index:0 };

        let mut parser = LogicalExpressionParserImpl { input: &parser_input, state: &mut parser_state };

        return parser.next_expression(OperatorPrecedence::Lowest);
    }

    fn get_tokens(word : &str, token_types : &Vec<TokenType>) -> Vec<Token>
    {
        if word.is_empty()
        {
            return Vec::new();
        }

        let mut tokens : Vec<Token> = Vec::new();

        if let Some(full_match) = token_types.iter()
            .find(|token_type| token_type.regex.is_match(word))
        {
            tokens.push(Token { type_id: full_match.id, value: word.to_string() })
        }

        return tokens;
    }

    fn next_expression(&mut self, precedence : OperatorPrecedence) -> Result<Formula>
    {
        //the recursion here ensures that the first thing that gets called is factor, and then under it, in lowering priority, calls to expression
        let mut node =
            if precedence == OperatorPrecedence::Highest { self.next_factor().context(codeloc!())? }
            else { self.next_expression(precedence.incremented()).context(codeloc!())? };

        //then go along the tokens until we have eaten all the ones of the current precedence
        while self.current_token_type().precedence == precedence
        {
            if self.current_token_type().category == TokenCategory::BinaryOperation
            {
                let index_before_eat = self.state.current_index;
                self.eat(self.current_token_type().id).context(codeloc!())?;

                //infix means the (now previous) node is the first arg, next node is the next arg
                let left = node;
                let right = (
                    if precedence == OperatorPrecedence::Highest { self.next_factor() }
                    else { self.next_expression(precedence.incremented()) }
                ).context(codeloc!())?;

                let to_formula = self.token_type_at_index(index_before_eat).to_formula;
                let name = self.token_at_index(index_before_eat).value.clone();
                node = to_formula(name, vec![left, right]);
            }
            else if self.current_token_type().category == TokenCategory::UnaryOperation
            {
                let index_before_eat = self.state.current_index;
                self.eat(self.current_token_type().id).context(codeloc!())?;

                let to_formula = self.token_type_at_index(index_before_eat).to_formula;
                let name = self.token_at_index(index_before_eat).value.clone();
                node = to_formula(name, vec![node]);
            }
            else { break; }
        }

        return Ok(node);
    }

    fn next_factor(&mut self) -> Result<Formula>
    {
        //a factor is a node that could begin an expression

        if self.current_token_type().category == TokenCategory::Atomic
        {
            let index_before_eat = self.state.current_index;
            self.eat(self.current_token_type().id).context(codeloc!())?;

            let to_formula = self.token_type_at_index(index_before_eat).to_formula;
            let name = self.token_at_index(index_before_eat).value.clone();
            return Ok(to_formula(name, vec![]));
        }

        if self.current_token_type().category == TokenCategory::UnaryOperation
        {
            if self.state.current_index != self.input.tokens.len()-1
            {
                let index_before_eat = self.state.current_index;
                self.eat(self.current_token_type().id).context(codeloc!())?;

                let precedence = self.token_type_at_index(index_before_eat).precedence;

                let operand = (if precedence == OperatorPrecedence::Highest { self.next_factor() }
                else { self.next_expression(precedence.incremented()) }).context(codeloc!())?;

                let to_formula = self.token_type_at_index(index_before_eat).to_formula;
                let name = self.token_at_index(index_before_eat).value.clone();
                return Ok(to_formula(name, vec![operand]));
            }

            return Err(anyhow!("Expected an expression at word index {}, but the text ended", self.state.current_index));
        }

        if self.current_token_type().id == TokenTypeID::OpenParenthesis
        {
            if self.state.current_index != self.input.tokens.len()-1
            {
                self.eat(self.current_token_type().id).context(codeloc!())?;

                let node = self.next_expression(OperatorPrecedence::Lowest)?;
                self.eat(TokenTypeID::ClosedParenthesis).context(codeloc!())?;
                return Ok(node);
            }

            return Err(anyhow!("Expected an expression at word index {}, but the text ended", self.state.current_index));
        }

        return Err(anyhow!("Incorrectly placed token {} at word index {}", self.current_token(), self.state.current_index));
    }

    fn eat(&mut self, required_type_id : TokenTypeID) -> Result<()>
    {
        if self.current_token().type_id == required_type_id
        {
            if self.state.current_index < self.input.tokens.len()-1
            {
                self.state.current_index += 1;
            }

            return Ok(());
        }

        return Err(anyhow!("Required {}, but it was {} at word index {}",
                required_type_id, self.current_token_type().id, self.state.current_index));
    }

    fn current_token(&self) -> &Token
    {
        return self.token_at_index(self.state.current_index);
    }

    fn current_token_type(&self) -> &TokenType
    {
        return self.token_type_at_index(self.state.current_index);
    }

    fn token_at_index(&self, index : usize) -> &Token
    {
        return &self.input.tokens[index];
    }

    fn token_type_at_index(&self, index : usize) -> &TokenType
    {
        let id = &self.input.tokens[index].type_id;

        return &self.input.token_types.iter()
            .find(|token_type| token_type.id == *id).unwrap();
    }
}
