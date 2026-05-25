use crate::eval::{Atom, EvalError, Object, Symbol, SymbolVal};

pub fn create_builtins() -> Vec<Symbol> {
    let mut builtins: Vec<Symbol> = Vec::new();

    // add
    builtins.push(Symbol {
        name: "+".to_string(),
        value: SymbolVal::Function(builtin_add),
    });

    builtins.push(Symbol {
        name: "-".to_string(),
        value: SymbolVal::Function(builtin_sub),
    });

    builtins.push(Symbol {
        name: "*".to_string(),
        value: SymbolVal::Function(builtin_mult),
    });

    builtins
}

fn builtin_add(args: &[Object]) -> Result<Object, EvalError> {
    let sum = args
        .iter()
        .map(|e| e.as_number())
        .collect::<Result<Vec<f64>, EvalError>>()?
        .iter()
        .sum::<f64>();

    Ok(Object::Atom(Atom::Number(sum)))
}

fn builtin_sub(args: &[Object]) -> Result<Object, EvalError> {
    let diff = args
        .iter()
        .map(|e| e.as_number())
        .collect::<Result<Vec<f64>, EvalError>>()?
        .into_iter()
        .reduce(|diff, e| diff - e)
        .ok_or(EvalError::UnexpectedEndOfList)?;

    Ok(Object::Atom(Atom::Number(diff)))
}

fn builtin_mult(args: &[Object]) -> Result<Object, EvalError> {
    let product = args
        .iter()
        .map(|e| e.as_number())
        .collect::<Result<Vec<f64>, EvalError>>()?
        .into_iter()
        .reduce(|acc, e| acc * e)
        .ok_or(EvalError::UnexpectedEndOfList)?;

    Ok(Object::Atom(Atom::Number(product)))
}
