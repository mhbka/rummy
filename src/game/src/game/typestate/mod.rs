pub mod phases;
pub(crate) mod transitions;
pub(crate) mod actions;

use phases::{GamePhase, HasGamePhase};
use actions::GameActions;
use transitions::GamePhaseTransitions;

trait Game<G, P>: 
    HasGamePhase<P> 
    + GameActions
    + GamePhaseTransitions<G, P>
where
    G: HasGamePhase<P>,
    P: GamePhase,
{}