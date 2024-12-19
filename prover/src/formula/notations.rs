use strum_macros::{Display, EnumIter};
use crate::parser::token_types::TokenTypeID;

#[derive(Eq, PartialEq, Hash, Clone, Copy, EnumIter, Display)]
pub enum OperatorNotations
{
    BookNotations,
    CommonMathNotations,
    SoftwareDevelopmentNotations,
    LogicNGNotations,
}

impl OperatorNotations
{
    pub fn get_operator_character(&self, token_type_id : TokenTypeID) -> char
    {
        return match self
        {
            OperatorNotations::BookNotations =>
            {
                match token_type_id
                {
                    TokenTypeID::Non => { '¬' }
                    TokenTypeID::And => { '∧' }
                    TokenTypeID::Or => { '∨' }
                    TokenTypeID::Imply => { '⊃' }
                    TokenTypeID::BiImply => { '≡' }
                    TokenTypeID::StrictImply => { '⥽' }
                    TokenTypeID::Conditional => { 'ᐅ' }
                    TokenTypeID::Possible => { '◇' }
                    TokenTypeID::Necessary => { '□' }
                    TokenTypeID::InPast => { 'ᵖ' }
                    TokenTypeID::InFuture => { 'ᶠ' }
                    TokenTypeID::Exists => { '∃' }
                    TokenTypeID::ForAll => { '∀' }
                    TokenTypeID::DefinitelyExists => { '𝔈' }
                    TokenTypeID::Equals => { '=' }
                    TokenTypeID::AtomicWithoutArgs => { ' '}
                    TokenTypeID::AtomicWithArgs => { ' ' }
                    TokenTypeID::OpenParenthesis => { '(' }
                    TokenTypeID::ClosedParenthesis => { ')' }
                }
            }

            OperatorNotations::CommonMathNotations =>
            {
                match token_type_id
                {
                    TokenTypeID::Non => { '¬' }
                    TokenTypeID::And => { '∧' }
                    TokenTypeID::Or => { '∨' }
                    TokenTypeID::Imply => { '→' }
                    TokenTypeID::BiImply => { '↔' }
                    TokenTypeID::StrictImply => { '⥽' }
                    TokenTypeID::Conditional => { 'ᐅ' }
                    TokenTypeID::Possible => { '◇' }
                    TokenTypeID::Necessary => { '□' }
                    TokenTypeID::InPast => { 'ᵖ' }
                    TokenTypeID::InFuture => { 'ᶠ' }
                    TokenTypeID::Exists => { '∃' }
                    TokenTypeID::ForAll => { '∀' }
                    TokenTypeID::DefinitelyExists => { '𝔈' }
                    TokenTypeID::Equals => { '=' }
                    TokenTypeID::AtomicWithoutArgs => { ' '}
                    TokenTypeID::AtomicWithArgs => { ' ' }
                    TokenTypeID::OpenParenthesis => { '(' }
                    TokenTypeID::ClosedParenthesis => { ')' }
                }
            }

            OperatorNotations::SoftwareDevelopmentNotations =>
            {
                match token_type_id
                {
                    TokenTypeID::Non => { '!' }
                    TokenTypeID::And => { '&' }
                    TokenTypeID::Or => { '|' }
                    TokenTypeID::Imply => { '→' }
                    TokenTypeID::BiImply => { '≡' }
                    TokenTypeID::StrictImply => { '⥽' }
                    TokenTypeID::Conditional => { 'ᐅ' }
                    TokenTypeID::Possible => { '◇' }
                    TokenTypeID::Necessary => { '□' }
                    TokenTypeID::InPast => { 'ᵖ' }
                    TokenTypeID::InFuture => { 'ᶠ' }
                    TokenTypeID::Exists => { '∃' }
                    TokenTypeID::ForAll => { '∀' }
                    TokenTypeID::DefinitelyExists => { '𝔈' }
                    TokenTypeID::Equals => { '=' }
                    TokenTypeID::AtomicWithoutArgs => { ' '}
                    TokenTypeID::AtomicWithArgs => { ' ' }
                    TokenTypeID::OpenParenthesis => { '(' }
                    TokenTypeID::ClosedParenthesis => { ')' }
                }
            }

            OperatorNotations::LogicNGNotations =>
            {
                match token_type_id
                {
                    TokenTypeID::Non => { '~' }
                    TokenTypeID::And => { '&' }
                    TokenTypeID::Or => { '|' }
                    TokenTypeID::Imply => { '→' }
                    TokenTypeID::BiImply => { '≡' }
                    TokenTypeID::StrictImply => { '⥽' }
                    TokenTypeID::Conditional => { 'ᐅ' }
                    TokenTypeID::Possible => { '◇' }
                    TokenTypeID::Necessary => { '□' }
                    TokenTypeID::InPast => { 'ᵖ' }
                    TokenTypeID::InFuture => { 'ᶠ' }
                    TokenTypeID::Exists => { '∃' }
                    TokenTypeID::ForAll => { '∀' }
                    TokenTypeID::DefinitelyExists => { '𝔈' }
                    TokenTypeID::Equals => { '=' }
                    TokenTypeID::AtomicWithoutArgs => { ' '}
                    TokenTypeID::AtomicWithArgs => { ' ' }
                    TokenTypeID::OpenParenthesis => { '(' }
                    TokenTypeID::ClosedParenthesis => { ')' }
                }
            }
        }
    }
}
