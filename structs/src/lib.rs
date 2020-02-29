
mod ancs;
mod bnr;
mod cmdl;
mod dol;
mod evnt;
mod font;
mod frme;
mod gc_disc;
mod hint;
mod mlvl;
mod mrea;
mod pak;
mod part;
mod savw;
mod scan;
mod scly;
mod strg;
mod thp;
mod txtr;


mod scly_props
{
    // http://www.metroid2002.com/retromodding/wiki/User:Parax0/Sandbox
    mod actor;
    mod damageable_trigger;
    mod dock;
    mod door;
    mod effect;
    mod hud_memo;
    mod memory_relay;
    mod pickup;
    mod platorm;
    mod point_of_interest;
    mod player_actor;
    mod player_hint;
    mod relay;
    mod sound;
    mod spawn_point;
    mod special_function;
    mod streamed_audio;
    mod timer;
    mod trigger;
    mod world_transporter;

    pub mod structs;

    pub use self::actor::*;
    pub use self::damageable_trigger::*;
    pub use self::dock::*;
    pub use self::door::*;
    pub use self::effect::*;
    pub use self::hud_memo::*;
    pub use self::memory_relay::*;
    pub use self::pickup::*;
    pub use self::platorm::*;
    pub use self::point_of_interest::*;
    pub use self::player_actor::*;
    pub use self::player_hint::*;
    pub use self::relay::*;
    pub use self::sound::*;
    pub use self::spawn_point::*;
    pub use self::special_function::*;
    pub use self::streamed_audio::*;
    pub use self::timer::*;
    pub use self::trigger::*;
    pub use self::world_transporter::*;
}
pub use scly_props::*;

pub use ancs::*;
pub use bnr::*;
pub use cmdl::*;
pub use dol::*;
pub use evnt::*;
pub use font::*;
pub use frme::*;
pub use gc_disc::*;
pub use hint::*;
pub use mlvl::*;
pub use mrea::*;
pub use pak::*;
pub use part::*;
pub use savw::*;
pub use scan::*;
pub use scly::*;
pub use strg::*;
pub use thp::*;
pub use txtr::*;
