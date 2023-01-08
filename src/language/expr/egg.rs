//! `egg` language for Starling expressions.

// nb: Some of the egg rewrites are taken from the egg test code, which is subject to the (MIT)
// licence of egg.  See https://github.com/egraphs-good/egg/blob/main/tests/math.rs

use egg::{rewrite as rw, DidMerge, Id, Language, Symbol};
use num_bigint::BigInt;
use once_cell::sync::OnceCell;

use super::Constant;

mod decode;
mod encode;

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
        // Terminals
        Constant(super::constant::Constant),
        Var(Symbol),
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
        // Zeroes
        rw!("mul-0"; "(* ?x 0)" => "0"),
        // Symmetry on equalities
        rw!("gt-lt"; "(> ?x ?y)" => "(< ?y ?x)"),
        rw!("ge-le"; "(>= ?x ?y)" => "(<= ?y ?x)"),
        rw!("neq-sym"; "(<> ?x ?y)" => "(<> ?y ?x)"),
        // Reflexivity
        //
        // We don't have rules on > and >= because we reduce these to < and <=.
        rw!("sub-reflexive"; "(- ?x ?x)" => "0"),
        rw!("div-reflexive"; "(div ?x ?x)" => "1" if is_not_zero("?x")),
        rw!("mod-reflexive"; "(mod ?x ?x)" => "0"),
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
        //
        // Equality
        //
        // We assume that there has been appropriate type checking to show that the terms are of the
        // appropriate type.
        //
        rw!("eq-reflexive"; "(= ?x ?x)" => "true"),
        rw!("eq-symmetric"; "(= ?x ?y)" => "(= ?y ?x)"),
        rw!("eq-transitive"; "(and (= ?x ?y) (= ?y ?z))" => "(= ?x ?z)"),
        rw!("eq-not"; "(= ?x (not ?y))" => "(<> ?x ?y)"),
        //
        // Equality against Boolean literals
        //
        //
        rw!("eq-true"; "(= ?x true)" => "?x"),
        rw!("eq-false"; "(= ?x false)" => "(not ?x)"),
        //
        // Disjunction
        //
        rw!("or-false"; "(or ?x false)" => "?x"),
        rw!("or-reflexive"; "(or ?x ?x)" => "?x"),
        rw!("or-true"; "(or ?x true)" => "true"),
        rw!("or-commute"; "(or ?x ?y)" => "(or ?y ?x)"),
        // Excluded middle
        rw!("or-saturate"; "(or ?x (not ?x))" => "true"),
        //
        // Conjunction
        //
        rw!("and-true"; "(and ?x true)" => "?x"),
        rw!("and-reflexive"; "(and ?x ?x)" => "?x"),
        rw!("and-false"; "(and ?x false)" => "false"),
        rw!("and-commute"; "(and ?x ?y)" => "(and ?y ?x)"),
        rw!("and-saturate"; "(and ?x (not ?x))" => "false"),
        //
        // Boolean negation
        //

        // Definition of not on Boolean literals:
        rw!("not-true"; "(not true)" => "false"),
        rw!("not-false"; "(not false)" => "true"),
        // This is a classical logic, so we have the law of double negation:
        rw!("not-not"; "(not (not ?x))" => "?x"),
        //
        // de Morgan's laws
        //
        rw!("de-morgan-and"; "(not (and ?x ?y))" => "(or (not ?x) (not ?y))"),
        rw!("de-morgan-or"; "(not (or ?x ?y))" => "(and (not ?x) (not ?y))"),
        //
        // Implication
        //
        rw!("implies-definition"; "(implies ?x ?y)" => "(or (not ?x) ?y)"),
        rw!("implies-reflexive"; "(implies ?x ?x)" => "true"),
        rw!("implies-antisymmetric"; "(and (implies ?x ?y) (implies ?y ?x))" => "(= ?x ?y)"),
        rw!("implies-transitive"; "(and (implies ?x ?y) (implies ?y ?z))" => "(implies ?x ?z)"),
        // As we're in classical logic, IFF is basically a strongly typed version of equality;
        // this rewrite rule makes that manifest.
        rw!("iff-definition"; "(iff ?x ?y)" => "(= ?x ?y)"),
    ]
}

/// Converts an expression to its e-graph form.
impl<M, V: Clone + Into<Symbol>> From<&super::Expr<M, V>> for Expr {
    fn from(value: &super::Expr<M, V>) -> Self {
        let mut result = Expr::default();
        let _ = encode::expr(&mut result, value);
        result
    }
}

/// Converts an expression from its e-graph form.
///
/// This assumes that the last node in the e-graph rec-expr is the top of the expression.
impl From<&Expr> for decode::Expr {
    fn from(e: &Expr) -> Self {
        let top_id = Id::from(e.as_ref().len() - 1);
        decode::expr(e, top_id)
    }
}

/// Performs optimising rewrites on an expression by turning it into an e-graph.
#[must_use]
pub fn simp(expr: &decode::Expr) -> decode::Expr {
    let egg_in = expr.into();
    let egg_out = simp_egraph(&egg_in);
    (&egg_out).into()
}

/// Performs optimising rewrites on an e-graph.
#[must_use]
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
            Term::Constant(c) => (c.clone(), format!("{c}").parse().unwrap()),
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
                let added = egraph.add(Term::Constant(c));
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

    /// Tests that the simplification in `simple_boolean` holds when we convert an expression to and
    /// from egg.
    #[test]
    fn simple_boolean_transcode() {
        use super::super::{
            bop::{Arith, Bool, Rel},
            Expr, Uop,
        };

        let expr: Expr<(), Symbol> = Expr::uop(
            Uop::Not,
            Expr::bop(
                Expr::bop(
                    Expr::bop(Expr::i64(1), Arith::Add, Expr::i64(1)),
                    Rel::Greater,
                    Expr::i64(3),
                ),
                Bool::And,
                Expr::bop(
                    Expr::bop(Expr::i64(3), Rel::Greater, Expr::i64(2)),
                    Bool::Implies,
                    Expr::bop(Expr::i64(3), Rel::GreaterEq, Expr::i64(2)),
                ),
            ),
        );

        assert_eq!(Expr::bool(true), simp(&expr));
    }

    egg::test_fn! {
        simple_equality,
        init(),
        "(= (div (* 4 4) 8) (- 3 1))" => "true"
    }

    egg::test_fn! {
        double_negate,
        init(),
        "(iff x (not (not x)))" => "true"
    }

    egg::test_fn! {
        vacuous_implication,
        init(),
        "(implies x (not x))" => "(not x)"
    }

    egg::test_fn! {
        impossible_boolean_equality,
        init(),
        "(= x (not x))" => "false"
    }
}
