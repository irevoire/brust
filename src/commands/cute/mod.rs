use serenity::framework::standard::macros::group;

mod cat;
mod dog;
mod fox;
mod frog;
mod spood;

use cat::*;
use dog::*;
use fox::*;
use frog::*;
use spood::*;

#[group]
#[commands(cat, dog, fox, spood, frog)]
pub struct Cute;
