pub mod phases;
pub(crate) mod transitions;
pub(crate) mod actions;

use phases::{GamePhase, HasGamePhase};
use actions::{DrawActions, GameActions};
use transitions::GamePhaseTransitions;

trait Game<G, P>: 
    HasGamePhase<P> 
    + GameActions
where
    G: HasGamePhase<P>,
    P: GamePhase,
{}

pub struct Rummy {}

impl Game<G, P> for Rummy {

}

impl GameActions for Rummy {}

impl DrawActions for Rummy {
    fn draw_stock(&mut self) -> Result<(), String> {
        todo!()
    }

    fn draw_discard_pile(&mut self) -> Result<(), String> {
        todo!()
    }
}