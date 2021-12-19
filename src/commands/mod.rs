use serenity::framework::standard::macros::group;

mod big;
mod choose;
mod cute;
mod mock;
mod poll;
mod react;
mod roasted;
mod roll;
mod tg;
// mod uwu;

use big::*;
use choose::*;
pub use cute::*;
use mock::*;
use poll::*;
use react::*;
use roasted::*;
use roll::*;
pub use tg::*;

#[group]
#[commands(tg, mock, roasted, big, react, choose, roll, poll)]
struct General;
