use serenity::framework::standard::macros::group;

mod an;
mod big;
mod choose;
mod cute;
mod lang;
mod mock;
mod poll;
mod react;
mod roasted;
mod roll;
mod tg;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod uwu;

use an::*;
use big::*;
use choose::*;
pub use cute::*;
pub use lang::*;
use mock::*;
use poll::*;
use react::*;
use roasted::*;
use roll::*;
pub use tg::*;

#[group]
#[commands(an, tg, mock, roasted, big, react, choose, roll, poll)]
struct General;
