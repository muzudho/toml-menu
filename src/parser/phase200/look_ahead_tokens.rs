//! Look-ahead tokens.  
//! 先読みトークン。  

use crate::parser::phase200::LookAheadTokens;
use crate::parser::phase200::Token;

impl LookAheadTokens {
    pub fn from_old(tokens: (Option<&Token>, Option<&Token>, Option<&Token>)) -> Self {
        LookAheadTokens {
            current: if let Some(t) = tokens.0 {
                Some(t.clone())
            } else {
                None
            },
            one_ahead: if let Some(t) = tokens.1 {
                Some(t.clone())
            } else {
                None
            },
            two_ahead: if let Some(t) = tokens.2 {
                Some(t.clone())
            } else {
                None
            },
        }
    }

    pub fn to_old(&self) -> (Option<&Token>, Option<&Token>, Option<&Token>) {
        let c = if let Some(t) = &self.current {
            Some(t)
        } else {
            None
        };
        let o = if let Some(t) = &self.one_ahead {
            Some(t)
        } else {
            None
        };
        let t = if let Some(t) = &self.two_ahead {
            Some(t)
        } else {
            None
        };
        (c, o, t)
    }
}