use crate::parser::token_types::TokenTypeID;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum OperatorNotations
{
    BookNotations,
    CommonMathNotations,
    ComputerScienceNotations,
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
                    _ => { ' ' }
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
                    _ => { ' ' }
                }
            }

            OperatorNotations::ComputerScienceNotations =>
            {
                match token_type_id
                {
                    TokenTypeID::Non => { '!' }
                    TokenTypeID::And => { '&' }
                    TokenTypeID::Or => { '|' }
                    TokenTypeID::Imply => { '→' }
                    TokenTypeID::BiImply => { '≡' }
                    _ => { ' ' }
                }
            }
        }
    }
}
