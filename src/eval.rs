use crate::parser::Expr;

#[derive(PartialEq, Debug)]
pub enum Object {
    Number(f64),
    Atom(String),
    Symbol(Symbol),
    Nil,
}

#[derive(PartialEq, Debug)]
pub enum Symbol {
    Variable(Expr),
    Function {
        name: String,
        def: fn(&Expr) -> Result<Object, EvalError>,
    },
}

impl Object {
    pub fn print(&self) -> String {
        match self {
            Object::Number(num) => num.to_string(),
            Object::Atom(s) => s.clone(),
            Object::Nil => "Nil".to_string(),
            Object::Symbol(sym) =>
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum EvalError {
    ExpectedNumber,
    ExpectedFunction,
    UnexpectedEndOfList,
}

pub fn evaluate(expr: &Expr) -> Result<Object, EvalError> {
    match expr {
        Expr::Number(num) => Ok(Object::Number(*num)),
        Expr::List(list) => evaluate_list(list),
        Expr::Nil => Ok(Object::Nil),
        Expr::Symbol(s) => Ok(Object::Atom(s.clone())),
    }
}

fn evaluate_list(list: &Vec<Expr>) -> Result<Object, EvalError> {
    // for now we only consider builtins
    let func = list.iter().next().ok_or(EvalError::UnexpectedEndOfList)?;
    if let Expr::Symbol(symbol) = func {
        match symbol.as_str() {
            // builtins
            "+" => builtin_add(&list[1..]),
            _ => todo!(),
        }
    } else {
        Err(EvalError::ExpectedFunction)
    }
}

fn builtin(symbol: &str) -> Option<fn(&Expr) -> Result<Object, EvalError>> {
    match symbol {
        "+" => Some(builtin_add),
    }
}

fn builtin_add(args: &[Expr]) -> Result<Object, EvalError> {
    let mut result: f64 = 0.0;

    for e in args {
        let num = match evaluate(e) {
            Ok(Object::Number(n)) => n,
            _ => return Err(EvalError::ExpectedNumber),
        };

        result += num;
    }

    Ok(Object::Number(result))
}
