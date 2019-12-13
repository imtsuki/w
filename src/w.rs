use std::collections::{HashMap, HashSet};

/// Represents an expression
#[derive(Debug)]
enum Exp {
    /// variable term
    Var(String),
    /// Literal term
    Lit(Lit),
    /// Apply term
    App(Box<Exp>, Box<Exp>),
    /// Abstraction term
    Abs(String, Box<Exp>),
    /// Let term
    Let(String, Box<Exp>, Box<Exp>),
}

/// Literal
#[derive(Debug)]
enum Lit {
    /// Int literal
    Int(i64),
    /// Bool literal
    Bool(bool),
    /// Str literal
    Str(String),
}

/// Type
/// monotype
#[derive(Clone, Debug)]
enum Type {
    /// Type variable
    Var(String),
    /// Int type
    Int,
    /// Bool type
    Bool,
    /// Str type
    Str,
    /// Function type
    Fun(Box<Type>, Box<Type>),
}

trait Types {
    /// Determine the free type variables of a type.
    fn ftv(&self) -> HashSet<String>;

    /// Apply a substitution.
    fn apply(&self, subst: &Subst) -> Self;
}

impl Types for Type {
    fn ftv(&self) -> HashSet<String> {
        use Type::*;
        match self {
            Var(n) => [n.clone()].iter().cloned().collect(),
            Int | Bool | Str => HashSet::new(),
            Fun(t1, t2) => t1.ftv().union(&t2.ftv()).cloned().collect(),
        }
    }

    fn apply(&self, subst: &Subst) -> Self {
        use Type::*;
        match self {
            Var(n) => match subst.get(n) {
                Some(t) => t.clone(),
                None => Var(n.clone()),
            },
            Fun(t1, t2) => Fun(Box::new(t1.apply(subst)), Box::new(t2.apply(subst))),
            t => t.clone(),
        }
    }
}

/// Substitution
type Subst = HashMap<String, Type>;
