//! `egg` language for Starling expressions.

// nb: Some of the egg rewrites are taken from the egg test code, which is subject to the (MIT)
// licence of egg.  See https://github.com/egraphs-good/egg/blob/main/tests/math.rs

use super::Constant;
use egg::{rewrite as rw, DidMerge, Id, Language, Symbol};
use num_bigint::BigInt;
use once_cell::sync::OnceCell;

egg::define_language! {
    /// The egg language for PVC expressions.
    pub enum Term {
        // Arithmetic binary operations
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "*" = Mul([Id; 2]),
        "div" = Div([Id; 2]),
        "mod" = Modulus([Id; 2]),
        // Boolean binary operations
        "and" = And([Id; 2]),
        "or" = Or([Id; 2]),
        "implies" = Implies([Id; 2]),
        "iff" = Iff([Id; 2]),
        // Relational binary operations
        "<" = Less([Id; 2]),
        "<=" = LessEq([Id; 2]),
        "=" = Eq([Id; 2]),
        "<>" = NotEq([Id; 2]),
        ">=" = GreaterEq([Id; 2]),
        ">" = Greater([Id; 2]),
        // Prefix operations
        "-" = Minus(Id),
        "+" = Plus(Id),
        // Postfix operations
        "^" = Deref(Id),
        "not" = Not(Id),
        // Literals
        Literal(super::constant::Constant),
        Symbol(Symbol),
    }
}

/// Rewrite rules for e-graphs over PVC expressions.
static RULES: OnceCell<Vec<Rewrite>> = OnceCell::new();

fn init() -> Vec<Rewrite> {
    vec![
        // Commutativity
        rw!("commute-add"; "(+ ?x ?y)" => "(+ ?y ?x)"),
        rw!("commute-mul"; "(* ?x ?y)" => "(* ?y ?x)"),
        // Units
        rw!("add-0"; "(+ ?x 0)" => "?x"),
        rw!("sub-0"; "(- ?x 0)" => "?x"),
        rw!("mul-1"; "(* ?x 1)" => "?x"),
        rw!("or-false"; "(or ?x false)" => "?x"),
        rw!("and-true"; "(and ?x true)" => "?x"),
        // Zeroes
        rw!("mul-0"; "(* ?x 0)" => "0"),
        rw!("or-true"; "(or ?x true)" => "true"),
        rw!("and-false"; "(and ?x false)" => "false"),
        // Symmetry on equalities
        rw!("gt-lt"; "(> ?x ?y)" => "(< ?y ?x)"),
        rw!("ge-le"; "(>= ?x ?y)" => "(<= ?y ?x)"),
        rw!("eq-sym"; "(= ?x ?y)" => "(= ?y ?x)"),
        rw!("neq-sym"; "(<> ?x ?y)" => "(<> ?y ?x)"),
        // Reflexivity
        //
        // We don't have rules on > and >= because we reduce these to < and <=.
        rw!("sub-reflexive"; "(- ?x ?x)" => "0"),
        rw!("div-reflexive"; "(div ?x ?x)" => "1" if is_not_zero("?x")),
        rw!("mod-reflexive"; "(mod ?x ?x)" => "0"),
        rw!("eq-reflexive"; "(= ?x ?x)" => "true"),
        rw!("leq-reflexive"; "(<= ?x ?x)" => "true"),
        rw!("neq-reflexive"; "(<> ?x ?x)" => "false"),
        rw!("lt-reflexive"; "(< ?x ?x)" => "false"),
        // Other arithmetic identities
        rw!("add-minus"; "(+ ?x (- ?y))" => "(- ?x ?y)"),
        rw!("sub-minus"; "(- 0 ?x)" => "(- ?x)"),
        rw!("minus-intro"; "(* ?x -1)" => "(- ?x)"),
        rw!("plus-intro"; "?x" => "(+ ?x)"),
        rw!("minus-eliminate"; "(- ?x)" => "(* ?x -1)"),
        rw!("plus-eliminate"; "(+ ?x)" => "?x"),
        // Boolean negation
        rw!("not-true"; "(not true)" => "false"),
        rw!("not-false"; "(not false)" => "true"),
        // de Morgan's laws
        rw!("de-morgan-and"; "(not (and ?x ?y))" => "(or (not ?x) (not ?y))"),
        rw!("de-morgan-or"; "(not (or ?x ?y))" => "(and (not ?x) (not ?y))"),
        // Other Boolean or
        rw!("implies-definition"; "(implies ?x ?y)" => "(or (not ?x) ?y)"),
    ]
}

/// Performs optimising rewrites on an e-graph.
pub fn simp_egraph(expr: &Expr) -> Expr {
    let rules = RULES.get_or_init(init);

    let runner = egg::Runner::default().with_expr(expr).run(rules);
    let extractor = egg::Extractor::new(&runner.egraph, egg::AstSize);
    let (_best_cost, best_expr) = extractor.find_best(runner.roots[0]);

    best_expr
}

/// Constant folding analysis.
///
/// This is heavily based on the test code from egg.
#[derive(Default)]
struct ConstantFolding;

impl egg::Analysis<Term> for ConstantFolding {
    type Data = Option<(Constant, egg::PatternAst<Term>)>;

    fn make(egraph: &EGraph, enode: &Term) -> Self::Data {
        let x = |i: &Id| egraph[*i].data.as_ref().map(|d| d.0.clone());
        Some(match enode {
            Term::Literal(c) => (c.clone(), format!("{c}").parse().unwrap()),
            Term::Add([a, b]) => fold_op(|a: BigInt, b| a + b, "+", x(a)?, x(b)?)?,
            Term::Sub([a, b]) => fold_op(|a: BigInt, b| a - b, "-", x(a)?, x(b)?)?,
            Term::Mul([a, b]) => fold_op(|a: BigInt, b| a * b, "*", x(a)?, x(b)?)?,
            Term::Div([a, b]) => fold_div(x(a)?, x(b)?)?,
            Term::Less([a, b]) => fold_op(|a: BigInt, b| a < b, "<", x(a)?, x(b)?)?,
            Term::LessEq([a, b]) => fold_op(|a: BigInt, b| a <= b, "<=", x(a)?, x(b)?)?,
            Term::Eq([a, b]) => fold_op(|a: Constant, b| a == b, "<=", x(a)?, x(b)?)?,
            Term::NotEq([a, b]) => fold_op(|a: Constant, b| a != b, "<=", x(a)?, x(b)?)?,
            _ => return None,
        })
    }

    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        egg::merge_option(to, from, |a, b| {
            assert_eq!(a.0, b.0, "Merged non-equal constants");
            DidMerge(false, false)
        })
    }

    fn modify(egraph: &mut EGraph, id: Id) {
        let data = egraph[id].data.clone();
        if let Some((c, pat)) = data {
            if egraph.are_explanations_enabled() {
                egraph.union_instantiations(
                    &pat,
                    &format!("{c}").parse().unwrap(),
                    &egg::Subst::default(),
                    "constant_fold".to_string(),
                );
            } else {
                let added = egraph.add(Term::Literal(c));
                egraph.union(id, added);
            }
            // to not prune, comment this out
            egraph[id].nodes.retain(Language::is_leaf);

            #[cfg(debug_assertions)]
            egraph[id].assert_unique_leaves();
        }
    }
}

/// Performs constant folding on an integer division.
///
/// This fails, naturally, if `b` is zero.
fn fold_div(a: Constant, b: Constant) -> Option<(Constant, egg::PatternAst<Term>)> {
    if b.is_zero() {
        None
    } else {
        fold_op(|a: BigInt, b| a / b, "div", a, b)
    }
}

/// Boilerplate for constant-folding an integer operation `op` over `a` and `b` with symbol `sym`.
fn fold_op<I: TryFrom<Constant>, O: Into<Constant>>(
    op: fn(I, I) -> O,
    sym: &'static str,
    a: Constant,
    b: Constant,
) -> Option<(Constant, egg::PatternAst<Term>)> {
    let ast = format!("({sym} {} {})", &a, &b).parse().unwrap();
    Some((op(a.try_into().ok()?, b.try_into().ok()?).into(), ast))
}

type EGraph = egg::EGraph<Term, ConstantFolding>;
type Rewrite = egg::Rewrite<Term, ConstantFolding>;
type Expr = egg::RecExpr<Term>;

fn is_not_zero(var: &str) -> impl Fn(&mut EGraph, Id, &egg::Subst) -> bool {
    let var = var.parse().unwrap();
    move |egraph, _, subst| {
        if let Some(n) = &egraph[subst[var]].data {
            !(n.0.is_zero())
        } else {
            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    egg::test_fn! {
        simple_boolean,
        init(),
        "(not (and (> (+ 1 1) 3) (implies (> 3 2) (>= 3 2))))" => "true"
    }

    egg::test_fn! {
        simple_equality,
        init(),
        "(= (div (* 4 4) 8) (- 3 1))" => "true"
    }
}