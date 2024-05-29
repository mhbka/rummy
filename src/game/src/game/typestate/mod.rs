pub mod phases;
pub(crate) mod actions;

use phases::{
    DiscardPhase, 
    DrawPhase, 
    GameEndPhase, 
    GamePhase, 
    HasGamePhase, 
    PlayPhase,
    RoundEndPhase
};
use actions::GameActions;

trait Game<G, P>: 
    HasGamePhase<P> 
    + GameActions
where
    P: GamePhase,
    G: HasGamePhase<P>
{}

