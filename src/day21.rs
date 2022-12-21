use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    rc::Rc,
};

pub enum Expression {
    Num(Option<u64>),
    Add(Rc<Expression>, Rc<Expression>),
    Sub(Rc<Expression>, Rc<Expression>),
    Mul(Rc<Expression>, Rc<Expression>),
    Div(Rc<Expression>, Rc<Expression>),
}

impl Expression {
    fn get_terms(&self) -> Option<(Rc<Expression>, Rc<Expression>)> {
        match self {
            Self::Num(_) => None,
            Self::Add(a, b) => Some((a.clone(), b.clone())),
            Self::Sub(a, b) => Some((a.clone(), b.clone())),
            Self::Mul(a, b) => Some((a.clone(), b.clone())),
            Self::Div(a, b) => Some((a.clone(), b.clone())),
        }
    }

    fn eval(&self) -> Option<u64> {
        match self {
            Self::Num(n) => *n,
            _ => {
                let (a, b) = self.get_terms().unwrap();
                let a = a.eval();
                let b = b.eval();
                match (a, b) {
                    (Some(a), Some(b)) => match self {
                        Self::Add(_, _) => Some(a + b),
                        Self::Sub(_, _) => Some(a - b),
                        Self::Mul(_, _) => Some(a * b),
                        Self::Div(_, _) => Some(a / b),
                        _ => unreachable!(),
                    },
                    _ => None,
                }
            }
        }
    }

    fn solve_for(mut lhs: Rc<Expression>, mut rhs: Rc<Expression>) -> Option<u64> {
        if let Some((a, b)) = lhs.get_terms() {
            let ax = a.eval();
            let bx = b.eval();

            if let Some(x) = ax {
                let x = Rc::new(Expression::Num(Some(x)));
                rhs = match &*lhs {
                    Self::Add(_, _) => Rc::new(Expression::Sub(rhs, x)),
                    Self::Sub(_, _) => Rc::new(Expression::Sub(x, rhs)),
                    Self::Mul(_, _) => Rc::new(Expression::Div(rhs, x)),
                    Self::Div(_, _) => Rc::new(Expression::Mul(x, rhs)),
                    _ => unreachable!(),
                };
                lhs = b;
            } else if let Some(x) = bx {
                let x = Rc::new(Expression::Num(Some(x)));
                rhs = match &*lhs {
                    Self::Add(_, _) => Rc::new(Expression::Sub(rhs, x)),
                    Self::Sub(_, _) => Rc::new(Expression::Add(rhs, x)),
                    Self::Mul(_, _) => Rc::new(Expression::Div(rhs, x)),
                    Self::Div(_, _) => Rc::new(Expression::Mul(rhs, x)),
                    _ => unreachable!(),
                };
                lhs = a;
            }
            Expression::solve_for(lhs, rhs)
        } else {
            rhs.eval()
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(n) => match n {
                Some(n) => write!(f, "{}", n),
                None => write!(f, "x"),
            },
            Self::Add(a, b) => write!(f, "({}+{})", a, b),
            Self::Sub(a, b) => write!(f, "({}-{})", a, b),
            Self::Mul(a, b) => write!(f, "({}*{})", a, b),
            Self::Div(a, b) => write!(f, "({}/{})", a, b),
        }
    }
}

#[derive(Clone)]
pub enum Monkey {
    Val(Option<u64>),
    Cal(String, String, char),
}

type Nodes = HashMap<String, Monkey>;
type Edges = HashMap<String, Vec<String>>;
type Parsed = (Nodes, Edges);

fn add_edge(from: &String, to: &String, edges: &mut Edges) {
    if let Some(neighbors) = edges.get_mut(from) {
        neighbors.push(to.to_owned());
    } else {
        edges.insert(from.to_owned(), vec![to.to_owned()]);
    }
}

#[aoc_generator(day21)]
pub fn parse(input: &str) -> Parsed {
    let mut edges = HashMap::<String, Vec<String>>::new();
    let monkeys: HashMap<String, Monkey> = input
        .replace(':', "")
        .lines()
        .map(|l| {
            let mut split = l.split(' ');
            let label = split.next().unwrap().to_owned();
            let a = split.next().unwrap().to_owned();

            let monkey = if let Ok(n) = a.parse::<u64>() {
                Monkey::Val(Some(n))
            } else {
                let op = split.next().unwrap().chars().next().unwrap();
                let b = split.next().unwrap().to_owned();

                add_edge(&a, &label, &mut edges);
                add_edge(&b, &label, &mut edges);

                Monkey::Cal(a, b, op)
            };

            (label, monkey)
        })
        .collect();

    (monkeys, edges)
}

fn expressions<F>((monkeys, edges): &Parsed, init: F) -> HashMap<String, Rc<Expression>>
where
    F: Fn(&String, &Monkey) -> Option<(String, Monkey)>,
{
    let mut visited = HashMap::<String, Rc<Expression>>::new();
    let mut queue: VecDeque<_> = monkeys
        .iter()
        .filter_map(|(label, monkey)| init(label, monkey))
        .collect();

    while let Some((label, monkey)) = queue.pop_front() {
        let expr = match monkey {
            Monkey::Val(n) => Expression::Num(n),
            Monkey::Cal(a, b, op) => {
                let a = visited.get(&a).unwrap().clone();
                let b = visited.get(&b).unwrap().clone();
                match op {
                    '+' => Expression::Add(a, b),
                    '-' => Expression::Sub(a, b),
                    '*' => Expression::Mul(a, b),
                    '/' => Expression::Div(a, b),
                    _ => unreachable!(),
                }
            }
        };
        visited.insert(label.to_owned(), Rc::new(expr));

        if let Some(neighbors) = edges.get(&label) {
            for next in neighbors {
                let next_monkey = monkeys.get(next).unwrap();
                match next_monkey {
                    Monkey::Cal(a, b, _) => {
                        if visited.contains_key(a) && visited.contains_key(b) {
                            queue.push_back((next.to_owned(), next_monkey.to_owned()))
                        }
                    }
                    Monkey::Val(_) => unreachable!(),
                }
            }
        }
    }

    visited
}

#[aoc(day21, part1)]
pub fn solve_part1(parsed: &Parsed) -> u64 {
    let expressions = expressions(parsed, |label, monkey| match monkey {
        Monkey::Val(_) => Some((label.to_owned(), monkey.clone())),
        _ => None,
    });

    expressions.get("root").unwrap().eval().unwrap()
}

#[aoc(day21, part2)]
pub fn solve_part2(parsed: &Parsed) -> u64 {
    let expressions = expressions(parsed, |label, monkey| {
        if label == "humn" {
            Some((label.to_owned(), Monkey::Val(None)))
        } else {
            match monkey {
                Monkey::Val(_) => Some((label.to_owned(), monkey.clone())),
                _ => None,
            }
        }
    });

    let monkeys = &parsed.0;
    let root = monkeys.get("root").unwrap();
    match root {
        Monkey::Val(_) => unreachable!(),
        Monkey::Cal(a, b, _) => {
            let lhs = expressions.get(a).unwrap().clone();

            let b = expressions.get(b).unwrap().eval().unwrap();
            let rhs = Rc::new(Expression::Num(Some(b)));

            Expression::solve_for(lhs, rhs).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn check_part1() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 152);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 301);
    }
}
