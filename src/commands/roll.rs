use anyhow::{anyhow, bail};
use rand::Rng;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};
use std::str::FromStr;

#[command]
#[description = r#"Roll dice"#]
pub async fn roll(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let mut rng = data.get::<crate::Random>().unwrap().lock().await;

    let terms = args
        .iter::<String>()
        .map(|arg| arg.unwrap())
        .map(|s| s.parse::<Term>())
        .collect::<anyhow::Result<Vec<Term>>>();

    if let Err(ref e) = terms {
        let _ = msg.reply(&ctx, format!("{}", e)).await?;
        return Ok(());
    }
    let terms = terms.unwrap();

    let mut parser = Parser::default();
    let mut res = Vec::new();

    for term in terms {
        let step = match parser.and(&term, &mut *rng) {
            Ok(step) => step,
            Err(e) => {
                let _ = msg.reply(&ctx, format!("{}", e)).await?;
                return Ok(());
            }
        };
        match term {
            Term::Roll(_) => res.push(format!("{}", step)),
            Term::Binop(op) => res.push(op.to_string()),
        }
    }

    let res = res.join(" ");
    let parser = parser.total;
    if parser.is_none() {
        return Ok(());
    }

    let _ = msg
        .reply(&ctx, format!("{} ({})", parser.unwrap(), res))
        .await;
    Ok(())
}

enum Roll {
    // a roll in the form of x throw of dices of y faces
    Dice(usize, usize),
    // a const value added or subbed to a dice
    Const(usize),
}

impl std::fmt::Display for Roll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Roll::Dice(rolls, faces) => format!("{}d{}", rolls, faces),
            Roll::Const(c) => format!("{}", c),
        };
        write!(f, "{}", s)
    }
}

impl Roll {
    /// Generate a value for the roll
    pub fn roll(&self, rng: &mut impl Rng) -> i64 {
        match self {
            Roll::Dice(rolls, faces) => {
                (0..*rolls).fold(0, |acc, _| acc + rng.gen_range(1..faces + 1) as i64)
            }
            Roll::Const(c) => *c as i64,
        }
    }
}

impl FromStr for Roll {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        if let Ok(c) = s.parse::<usize>() {
            return Ok(Roll::Const(c));
        }
        let v: Vec<&str> = s.split('d').collect();
        if v.len() != 2 {
            bail!("Failed to parse the dice");
        }
        let (rolls, faces) = (v[0], v[1]);
        Ok(Roll::Dice(rolls.parse()?, faces.parse()?))
    }
}

/// represent different binary operations
#[derive(Clone, Copy)]
enum Binop {
    Add,
    Sub,
    Mul,
}

impl std::fmt::Display for Binop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Binop::Add => "+",
            Binop::Sub => "-",
            Binop::Mul => "*",
        };
        write!(f, "{}", s)
    }
}

impl Binop {
    pub fn run(&self, left: i64, right: i64) -> i64 {
        match self {
            Binop::Add => left + right,
            Binop::Sub => left - right,
            Binop::Mul => left * right,
        }
    }
}

impl FromStr for Binop {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(Binop::Add),
            "-" => Ok(Binop::Sub),
            "*" => Ok(Binop::Mul),
            s => bail!("Unsupported operator {}", s),
        }
    }
}

enum Term {
    Binop(Binop),
    Roll(Roll),
}

impl FromStr for Term {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        if let Ok(b) = s.parse::<Binop>() {
            Ok(Term::Binop(b))
        } else if let Ok(r) = s.parse::<Roll>() {
            Ok(Term::Roll(r))
        } else {
            bail!("Can't parse this term: \"{}\"", s)
        }
    }
}

#[derive(Default)]
struct Parser {
    total: Option<i64>,
    binop: Option<Binop>,
}

impl Parser {
    pub fn and(&mut self, term: &Term, rng: &mut impl Rng) -> anyhow::Result<i64> {
        match term {
            Term::Binop(binop) => {
                if self.binop.is_some() || self.total.is_none() {
                    bail!(
                        "Malformed equation: was expecting a term but instead got: {}",
                        binop
                    );
                }
                self.binop = Some(*binop);
                self.total.ok_or(anyhow!("Bug"))
            }
            Term::Roll(roll) => {
                let res = roll.roll(rng);

                if self.binop.is_none() && self.total.is_some() {
                    bail!(
                        "Malformed equation: was expecting an operator but intsead got: {}",
                        roll
                    );
                } else if self.total.is_none() {
                    self.total = Some(res);
                    Ok(res)
                } else {
                    self.total = Some(self.binop.take().unwrap().run(self.total.unwrap(), res));
                    Ok(res)
                }
            }
        }
    }
}
