use crate::core::common::token::Token;

pub fn take_description(tokens: &[Token], index: &mut usize) -> Option<String> {
    if let Some(Token::Description(desc)) = tokens.get(*index) {
        *index += 1;
        Some(desc.clone())
    } else {
        None
    }
}
