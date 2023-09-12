use std::fs;
use std::{path::PathBuf, rc::Rc};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(tag = "kind")]
pub enum Term {
    Parameter(Parameter),
    Var(Var),
    Fn(Function),
    Let(Let),
    Call(Call),
    Str(Str),
    Binary(Binary),
    If(If),
    Bool(Bool),
    First(First),
    Second(Second),
    Print(Print),
    Int(Int),
}

pub type BoxTerm = Box<Rc<Term>>;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Location {
    pub start: usize,
    pub end: usize,
    pub filename: PathBuf,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct File {
    pub name: PathBuf,
    pub location: Location,
    pub expression: BoxTerm,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Parameter {
    pub text: String,
    pub location: Location,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Var {
    pub text: String,
    pub location: Location,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Function {
    pub parameters: Vec<Parameter>,
    pub location: Location,
    pub value: BoxTerm,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Call {
    pub callee: BoxTerm,
    pub arguments: Vec<BoxTerm>,
    pub location: Location,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Let {
    pub name: Parameter,
    pub value: BoxTerm,
    pub next: Option<BoxTerm>,
    pub location: Location,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Str {
    pub value: String,
    pub location: Location,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Int {
    pub value: i32,
    pub location: Location,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Bool {
    pub value: bool,
    pub location: Location,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Binary {
    pub lhs: BoxTerm,
    pub rhs: BoxTerm,
    pub op: BinaryOp,
    pub location: Location,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct If {
    pub condition: BoxTerm,
    pub then: BoxTerm,
    pub otherwise: BoxTerm,
    pub location: Location,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Tuple {
    pub first: BoxTerm,
    pub second: BoxTerm,
    pub location: Location,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct First {
    pub value: BoxTerm,
    pub location: Location,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Second {
    pub value: BoxTerm,
    pub location: Location,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Print {
    pub value: BoxTerm,
    pub location: Location,
}

macro_rules! into_boxed_term {
    ($variant:ident) => {
        impl Into<BoxTerm> for $variant {
            fn into(self) -> BoxTerm {
                Box::new(Rc::new(Term::$variant(self)))
            }
        }
    };
}

into_boxed_term!(Parameter);
into_boxed_term!(Var);
into_boxed_term!(Let);
into_boxed_term!(Call);
into_boxed_term!(Str);
into_boxed_term!(Binary);
into_boxed_term!(If);
into_boxed_term!(Bool);
into_boxed_term!(First);
into_boxed_term!(Second);
into_boxed_term!(Int);
into_boxed_term!(Print);

pub fn parse_ast_json_file_contents(ast: &str) -> Result<File, crate::error::Error> {
    Ok(serde_json::from_str(ast)?)
}

pub fn parse_json_file(ast_path: PathBuf) -> Result<File, crate::error::Error> {
    parse_ast_json_file_contents(&fs::read_to_string(ast_path)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_sum() {
        let ast_json = r#"
        {
            "name": "files/basic_sum.rinha",
            "expression": {
                "kind": "Let",
                "name": {
                    "text": "sum",
                    "location": {
                        "start": 4,
                        "end": 7,
                        "filename": "files/basic_sum.rinha"
                    }
                },
                "value": {
                    "kind": "Binary",
                    "op": "Add",
                    "lhs": {
                        "kind": "Int",
                        "value": 1,
                        "location": {
                            "start": 11,
                            "end": 12,
                            "filename": "files/basic_sum.rinha"
                        } 
                    },
                    "rhs": {
                        "kind": "Int",
                        "value": 1,
                        "location": {
                            "start": 15,
                            "end": 16,
                            "filename": "files/basic_sum.rinha"
                        }
                    },
                    "location": {
                        "start": 11,
                        "end": 16,
                        "filename": "files/basic_sum.rinha"
                    }
                },
                "next": {
                    "kind": "Print",
                    "value": {
                        "kind": "Var",
                        "text": "sum",
                        "location": {
                            "start": 27,
                            "end": 30,
                            "filename": "files/basic_sum.rinha"
                        }
                    },
                    "location": {
                        "start": 19,
                        "end": 31,
                        "filename": "files/basic_sum.rinha"
                    }
                },
                "location": {
                    "start": 0,
                    "end": 16,
                    "filename": "files/basic_sum.rinha"
                }
            },
            "location": {
                "start": 0,
                "end": 31,
                "filename": "files/basic_sum.rinha"
            }
        }
        "#;
        let file_path: PathBuf = "files/basic_sum.rinha".into();
        assert_eq!(
            parse_ast_json_file_contents(ast_json).unwrap(),
            File {
                name: file_path.clone(),
                expression: Let {
                    name: Parameter {
                        text: "sum".into(),
                        location: Location {
                            start: 4,
                            end: 7,
                            filename: file_path.clone()
                        }
                    },
                    value: Binary {
                        location: Location {
                            start: 11,
                            end: 16,
                            filename: file_path.clone()
                        },
                        op: BinaryOp::Add,
                        lhs: Int {
                            value: 1,
                            location: Location {
                                start: 11,
                                end: 12,
                                filename: file_path.clone(),
                            }
                        }
                        .into(),
                        rhs: Int {
                            value: 1,
                            location: Location {
                                start: 15,
                                end: 16,
                                filename: file_path.clone()
                            }
                        }
                        .into()
                    }
                    .into(),
                    location: Location {
                        start: 0,
                        end: 16,
                        filename: file_path.clone(),
                    },
                    next: Some(
                        Print {
                            location: Location {
                                start: 19,
                                end: 31,
                                filename: file_path.clone()
                            },
                            value: Var {
                                text: "sum".into(),
                                location: Location {
                                    start: 27,
                                    end: 30,
                                    filename: file_path.clone()
                                }
                            }
                            .into()
                        }
                        .into()
                    )
                }
                .into(),
                location: Location {
                    start: 0,
                    end: 31,
                    filename: file_path.clone()
                }
            }
        )
    }
}
