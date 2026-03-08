use std::fmt::{self, Display, Formatter};

use chumsky::span::SimpleSpan;

use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct TypeCheckErr {
    pub kind: TypeCheckErrKind,
    pub span: SimpleSpan,
}

impl Display for TypeCheckErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason())
    }
}

impl TypeCheckErr {
    pub fn reason(&self) -> String {
        match &self.kind {
            TypeCheckErrKind::UndefinedIdentifier(ident) => format!("undefined ident: `{ident}`"),
            TypeCheckErrKind::UndefinedMethod(ident) => format!("undefined method: `{ident}`"),
            TypeCheckErrKind::TypeMissmatch { actual, expected } => {
                format!("expected: `{expected}`, actual: `{actual}`")
            }
            TypeCheckErrKind::Multiple(_) => unreachable!(),
        }
    }

    pub fn flatten(self) -> Self {
        if let TypeCheckErrKind::Multiple(errs) = self.kind {
            let mut new_errs = Vec::new();
            for e in errs {
                if let TypeCheckErrKind::Multiple(child_errs) = e.kind {
                    let mut child_errs = child_errs.into_iter().map(|e| e.flatten()).collect();
                    new_errs.append(&mut child_errs);
                } else {
                    new_errs.push(e);
                }
            }
            TypeCheckErr {
                kind: TypeCheckErrKind::Multiple(new_errs),
                span: self.span,
            }
        } else {
            self
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeCheckErrKind {
    UndefinedIdentifier(String),
    UndefinedMethod(String),
    TypeMissmatch { actual: Type, expected: Type },

    Multiple(Vec<TypeCheckErr>),
}

pub fn expect(expected: Type, actual: Type, span: SimpleSpan) -> Result<(), TypeCheckErr> {
    if expected == actual {
        Ok(())
    } else {
        Err(TypeCheckErr {
            span,
            kind: TypeCheckErrKind::TypeMissmatch { actual, expected },
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RunTimeErr {
    pub kind: RunTimeErrKind,
    pub span: SimpleSpan,
}

impl RunTimeErr {
    pub fn reason(&self) -> String {
        match &self.kind {
            RunTimeErrKind::OutputMissmatch { actual, expected } => {
                format!("expected: `{expected}`, actual: `{actual}`")
            }
            RunTimeErrKind::Multiple(_) => unreachable!(),
        }
    }

    pub fn flatten(self) -> Self {
        if let RunTimeErrKind::Multiple(errs) = self.kind {
            let mut new_errs = Vec::new();
            for e in errs {
                if let RunTimeErrKind::Multiple(child_errs) = e.kind {
                    let mut child_errs = child_errs.into_iter().map(|e| e.flatten()).collect();
                    new_errs.append(&mut child_errs);
                } else {
                    new_errs.push(e);
                }
            }
            RunTimeErr {
                kind: RunTimeErrKind::Multiple(new_errs),
                span: self.span,
            }
        } else {
            self
        }
    }
}

impl Display for RunTimeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RunTimeErrKind {
    OutputMissmatch { actual: String, expected: String },

    Multiple(Vec<RunTimeErr>),
}
