use std::sync::mpsc;

use common_game::protocols::messages::{ExplorerToPlanet, OrchestratorToPlanet, PlanetToExplorer, PlanetToOrchestrator};


pub struct Explorer{
    orchestrator_channels: Option<(
            mpsc::Receiver<OrchestratorToPlanet>,
            mpsc::Sender<PlanetToOrchestrator>,
        )>,
    explorer_channels: Option<(
            mpsc::Receiver<ExplorerToPlanet>,
            mpsc::Sender<PlanetToExplorer>,
        )>,
}

impl Explorer{
    pub fn new()->Self{
        Self{
            orchestrator_channels:None,
            explorer_channels:None,
        }
    }
}