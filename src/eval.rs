use crate::parser::Expr;

#[derive(PartialEq, Debug)]
pub enum Object {
    Atom(Atom),
    Symbol(Symbol),
    Nil,
}

impl Object {
    pub fn print(&self) -> String {
        match self {
            Object::Atom(Atom::Number(num)) => num.to_string(),
            Object::Atom(Atom::String(s)) => s.to_string(),
            Object::Nil => "Nil".to_string(),
            Object::Symbol(sym) => format!(
                "Symbol(name = '{}' value = '{}')",
                sym.name,
                sym.value.print()
            ),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Atom {
    Number(f64),
    String(String),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Symbol {
    name: String,
    value: SymbolVal,
}

#[derive(PartialEq, Debug, Clone)]
pub enum SymbolVal {
    Expr(Expr),
    Function(fn(&[Object]) -> Result<Object, EvalError>),
}

impl SymbolVal {
    pub fn print(&self) -> String {
        match self {
            Self::Expr(expr) => expr.print(),
            Self::Function(_) => "function".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum EvalError {
    ExpectedNumber,
    ExpectedFunction,
    UnexpectedEndOfList,
    ListDoesNotStartWithFunction,
    SymbolNotFound(String),
}

pub fn evaluate(expr: &Expr, scope: &[Symbol]) -> Result<Object, EvalError> {
    match expr {
        Expr::Number(num) => Ok(Object::Atom(Atom::Number(*num))),
        Expr::String(s) => Ok(Object::Atom(Atom::String(s.clone()))),
        Expr::List(list) => evaluate_list(list, scope),
        Expr::Nil => Ok(Object::Nil),
        Expr::Symbol(sym) => evaluate_symbol(sym, scope),
        Expr::Cons { car: _, cdr: _ } => todo!(),
    }
}

// Finds and returns the symbol in the symbol table
fn evaluate_symbol(symbol: &str, scope: &[Symbol]) -> Result<Object, EvalError> {
    let sym = scope
        .iter()
        .find(|s| s.name == symbol)
        .ok_or(EvalError::SymbolNotFound(symbol.to_string()))?;

    Ok(Object::Symbol((*sym).clone()))
}

fn evaluate_list(list: &[Expr], scope: &[Symbol]) -> Result<Object, EvalError> {
    let (symbol_expr, args) = list.split_first().ok_or(EvalError::UnexpectedEndOfList)?;

    // Get symbol - make sure it's a function definition
    let symbol = evaluate(symbol_expr, scope)?;
    if let Object::Symbol(Symbol { name: _, value }) = symbol {
        if let SymbolVal::Function(func) = value {
            let evaluated_args = evaluate_collection(args, scope)?;
            return func(&evaluated_args);
        }
    }

    return Err(EvalError::ListDoesNotStartWithFunction);
}

// Used for evaluating the args of a list before calling the function
fn evaluate_collection(args: &[Expr], scope: &[Symbol]) -> Result<Vec<Object>, EvalError> {
    let mut result: Vec<Object> = Vec::new();

    for expr in args {
        result.push(evaluate(expr, scope)?);
    }

    Ok(result)
}

pub fn create_builtins() -> Vec<Symbol> {
    let mut builtins: Vec<Symbol> = Vec::new();

    // add
    builtins.push(Symbol {
        name: "+".to_string(),
        value: SymbolVal::Function(builtin_add),
    });

    builtins
}

fn builtin_add(args: &[Object]) -> Result<Object, EvalError> {
    let mut sum: f64 = 0.0;

    for arg in args {
        if let Object::Atom(Atom::Number(num)) = arg {
            sum += num;
        } else {
            return Err(EvalError::ExpectedNumber);
        }
    }

    Ok(Object::Atom(Atom::Number(sum)))
}
