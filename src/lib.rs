#![no_std]

pub trait Lexer {
    fn lex<'src>(&self, source: &'src str) -> Option<&'src str>;
}

impl Lexer for &str {
    fn lex<'src>(&self, source: &'src str) -> Option<&'src str> {
        source.strip_prefix(self)
    }
}

pub struct Pred<F>(pub F);

impl<F: Fn(char) -> bool> Lexer for Pred<F> {
    fn lex<'src>(&self, source: &'src str) -> Option<&'src str> {
        source.strip_prefix(&self.0)
    }
}

impl<A: Lexer, B: Lexer> Lexer for (A, B) {
    fn lex<'src>(&self, source: &'src str) -> Option<&'src str> {
        let source = self.0.lex(source)?;
        self.1.lex(source)
    }
}

impl<A: Lexer, B: Lexer, C: Lexer> Lexer for (A, B, C) {
    fn lex<'src>(&self, source: &'src str) -> Option<&'src str> {
        let source = self.0.lex(source)?;
        let source = self.1.lex(source)?;
        self.2.lex(source)
    }
}

pub struct Or<A, B>(pub A, pub B);

impl<A: Lexer, B: Lexer> Lexer for Or<A, B> {
    fn lex<'src>(&self, source: &'src str) -> Option<&'src str> {
        self.0.lex(source).or_else(|| self.1.lex(source))
    }
}

pub struct Maybe<L>(pub L);

impl<L: Lexer> Lexer for Maybe<L> {
    fn lex<'src>(&self, source: &'src str) -> Option<&'src str> {
        Some(self.0.lex(source).unwrap_or(source))
    }
}

pub struct Repeat0<L>(pub L);

impl<L: Lexer> Lexer for Repeat0<L> {
    fn lex<'src>(&self, mut source: &'src str) -> Option<&'src str> {
        while let Some(s) = self.0.lex(source) {
            source = s;
        }
        Some(source)
    }
}

pub struct Repeat1<L>(pub L);

impl<L: Lexer> Lexer for Repeat1<L> {
    fn lex<'src>(&self, source: &'src str) -> Option<&'src str> {
        let mut source = self.0.lex(source)?;
        while let Some(s) = self.0.lex(source) {
            source = s;
        }
        Some(source)
    }
}
