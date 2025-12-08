//use std::sync::mpsc;
use crossbeam_channel::{Sender, Receiver, unbounded};

use common_game::{components::planet::Planet, protocols::messages::{
    ExplorerToOrchestrator, ExplorerToPlanet, OrchestratorToExplorer,
    PlanetToExplorer,
}};

pub type BagType = u32;

#[derive(Debug, Clone)]
pub struct Explorer {
    pub planet_id: Option<u32>, //I assume that the travel isn't instant so I put an Option we should manage the case the planet explodes
    pub orchestrator_channels: (
        Receiver<OrchestratorToExplorer>,
        Sender<ExplorerToOrchestrator<BagType>>,
    ),
    pub explorer_to_planet_channels: Option<(
        Receiver<PlanetToExplorer>,
        Sender<ExplorerToPlanet>,
    )>,
    pub planet_to_explorer_channels: Option<(
        Receiver<ExplorerToPlanet>,
        Sender<PlanetToExplorer>,
    )>
}

impl Explorer {
    //At creation, an Explorer should be connected to Orchestrator and the starting Planet
    pub fn new(
        planet_id: Option<u32>,
        explorer_to_orchestrator_channels: (
            Receiver<OrchestratorToExplorer>,
            Sender<ExplorerToOrchestrator<BagType>>,
        ),
        explorer_to_planet_channels:(
            Receiver<PlanetToExplorer>,
            Sender<ExplorerToPlanet>,
        ),
        planet_to_explorer_channels:(
            Receiver<ExplorerToPlanet>,
            Sender<PlanetToExplorer>,
        )
    ) -> Self {
        Self {
            planet_id: planet_id,
            orchestrator_channels: explorer_to_orchestrator_channels,
            explorer_to_planet_channels: Some(explorer_to_planet_channels),
            planet_to_explorer_channels: Some(planet_to_explorer_channels),
        }
    }
}
