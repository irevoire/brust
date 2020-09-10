use rand::Rng;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};
use std::str::FromStr;

#[command]
#[description = r#"Roll dice"#]
pub fn roll(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();
    let mut rng = data.get::<crate::Random>().unwrap().lock().unwrap();

    let terms = args
        .iter::<String>()
        .map(|arg| arg.unwrap())
        .map(|s| s.parse::<Term>())
        .collect::<Result<Vec<Term>, Box<dyn std::error::Error>>>();

    if let Err(ref e) = terms {
        let _ = msg.reply(&ctx, format!("{}", e));
        return Err(e.into());
    }
    let terms = terms.unwrap();

    let mut parser = Parser::default();
    let mut res = Vec::new();

    for term in terms {
        let step = match parser.and(&term, &mut *rng) {
            Ok(step) => step,
            Err(e) => {
                let _ = msg.reply(&ctx, format!("{}", e));
                return Err(e.into());
            }
        };
        match term {
            Term::Roll(_) => res.push(format!("{}", step)),
            Term::Binop(op) => res.push(op.to_string()),
        }
    }

    let res = res.join(" ");

    let _ = msg.reply(
        &ctx,
        format!("{} ({})", parser.total.ok_or("There was no op")?, res),
    );
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
                (0..*rolls).fold(0, |acc, _| acc + rng.gen_range(1, faces + 1) as i64)
            }
            Roll::Const(c) => *c as i64,
        }
    }
}

impl FromStr for Roll {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(c) = s.parse::<usize>() {
            return Ok(Roll::Const(c));
        }
        let v: Vec<&str> = s.split('d').collect();
        if v.len() != 2 {
            return Err("Failed to parse the dice".into());
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
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(Binop::Add),
            "-" => Ok(Binop::Sub),
            "*" => Ok(Binop::Mul),
            s => Err(format!("Unsupported operator {}", s).into()),
        }
    }
}

enum Term {
    Binop(Binop),
    Roll(Roll),
}

impl FromStr for Term {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(b) = s.parse::<Binop>() {
            Ok(Term::Binop(b))
        } else if let Ok(r) = s.parse::<Roll>() {
            Ok(Term::Roll(r))
        } else {
            Err(format!("Can't parse this term: \"{}\"", s).into())
        }
    }
}

#[derive(Default)]
struct Parser {
    total: Option<i64>,
    binop: Option<Binop>,
}

impl Parser {
    pub fn and(
        &mut self,
        term: &Term,
        rng: &mut impl Rng,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        match term {
            Term::Binop(binop) => {
                if self.binop.is_some() || self.total.is_none() {
                    return Err(format!(
                        "Malformed equation: was expecting a term but instead got: {}",
                        binop
                    )
                    .into());
                }
                self.binop = Some(*binop);
                self.total.ok_or("Bug".into())
            }
            Term::Roll(roll) => {
                let res = roll.roll(rng);

                if self.binop.is_none() && self.total.is_some() {
                    Err(format!(
                        "Malformed equation: was expecting an operator but intsead got: {}",
                        roll
                    )
                    .into())
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
