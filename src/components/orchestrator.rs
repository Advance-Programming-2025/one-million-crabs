use std::sync::mpsc;

use common_game::components::generator::Generator;
use common_game::components::planet::Planet;
use common_game::protocols::messages::{
    ExplorerToOrchestrator, ExplorerToPlanet, OrchestratorToExplorer, OrchestratorToPlanet,
    PlanetToExplorer, PlanetToOrchestrator,
};

use crate::components::CrabRaveConstructor;
use crate::components::explorer::Explorer;

// use common_game::components::sunray::Sunray;
// use common_game::protocols::messages::StartPlanetAiMsg;
// use common_game::protocols::messages::StopPlanetAiMsg;
// use common_game::protocols::messages::SupportedCombinationRequest;
// use common_game::components::asteroid::Asteroid;
// use common_game::protocols::messages::CombineResourceRequest;
// use common_game::protocols::messages::SupportedResourceRequest;
// use common_game::protocols::messages::GenerateResourceRequest;
// use common_game::protocols::messages::CurrentPlanetRequest;
// use common_game::protocols::messages::MoveToPlanet;
// use common_game::protocols::messages::ResetExplorerAIMsg;

pub type GalaxyInit = (
    Vec<Planet>,
    Vec<Explorer>,
    (
        mpsc::Receiver<PlanetToOrchestrator>,
        mpsc::Sender<OrchestratorToPlanet>,
    ),
    Option<(
        mpsc::Receiver<ExplorerToOrchestrator>,
        mpsc::Sender<OrchestratorToExplorer>,
    )>,
);

// B generic is there for representing the content type of the bag
pub struct Orchestrator {
    generator: Generator,

    galaxy_topology: Vec<Planet>, //At the moment we need only one planet for testing
    explorers: Vec<Explorer>,     //At the momet we need only one explorer for testing

    //we can better define communication like this: galaxy_communication: Option<HashMap<u32,u32>>
    planet_channels: (
        mpsc::Receiver<PlanetToOrchestrator>,
        mpsc::Sender<OrchestratorToPlanet>,
    ),
    explorer_channels: Option<(
        mpsc::Receiver<ExplorerToOrchestrator>,
        mpsc::Sender<OrchestratorToExplorer>,
    )>,
}

impl Orchestrator {
    //Check and init orchestrator
    pub fn new() -> Result<Self, String> {
        let generator = Generator::new()?;
        let galaxy_initialization = Orchestrator::initialize_galaxy()?;
        Ok(Orchestrator {
            generator: generator,
            galaxy_topology:galaxy_initialization.0,
            explorers: galaxy_initialization.1,
            planet_channels: galaxy_initialization.2,
            explorer_channels: galaxy_initialization.3,
        })
    }

    //The return is Result<(), String> because if an error occur it go back to the main that finishes
    // I don't know if there are better approach but I think it is pretty elegant

    pub fn initialize_galaxy(/*_path: &str*/) -> Result<GalaxyInit, String> {
        //planet-orch and orch-planet
        let (planet_sender, orch_receiver): (
            mpsc::Sender<PlanetToOrchestrator>,
            mpsc::Receiver<PlanetToOrchestrator>,
        ) = mpsc::channel();
        let (orch_sender, planet_receiver): (
            mpsc::Sender<OrchestratorToPlanet>,
            mpsc::Receiver<OrchestratorToPlanet>,
        ) = mpsc::channel();

        let planet_to_orchestrator_channels = (planet_receiver, planet_sender);
        let orchestrator_to_planet_channels = (orch_receiver, orch_sender);

        //planet-explorer and explorer-planet
        let (planet_sender, explorer_receiver): (
            mpsc::Sender<PlanetToExplorer>,
            mpsc::Receiver<PlanetToExplorer>,
        ) = mpsc::channel();
        let (explorer_sender, planet_receiver): (
            mpsc::Sender<ExplorerToPlanet>,
            mpsc::Receiver<ExplorerToPlanet>,
        ) = mpsc::channel();

        let planet_to_explorer_channels = (planet_receiver, planet_sender);
        let explorer_to_planet_channels = (explorer_receiver, explorer_sender);

        //explorer-orchestrator and orchestrator-explorer
        let (explorer_sender, orch_receiver): (
            mpsc::Sender<ExplorerToOrchestrator>,
            mpsc::Receiver<ExplorerToOrchestrator>,
        ) = mpsc::channel();
        let (orch_sender, explorer_receiver): (
            mpsc::Sender<OrchestratorToExplorer>,
            mpsc::Receiver<OrchestratorToExplorer>,
        ) = mpsc::channel();

        let explorer_to_orchestrator_channels = (explorer_receiver, explorer_sender);
        let orchestrator_to_explorer_channels = (orch_receiver, orch_sender);

        //Construct crab-rave planet
        let crab_rave_planet = CrabRaveConstructor::new(
            0,
            planet_to_orchestrator_channels,
            planet_to_explorer_channels,
        )?;
        // self.planet_channels = Some(orchestrator_to_planet_channels);
        // self.explorer_channels = Some(orchestrator_to_explorer_channels);
        //Add the constructed galaxy to our Orchestrator
        let galaxy = vec![crab_rave_planet];

        //Construct Explorer
        let explorer = Explorer::new(Some(galaxy[0].id()), explorer_to_orchestrator_channels, explorer_to_planet_channels);
        let explorers = vec![explorer];

        Ok((galaxy, explorers, orchestrator_to_planet_channels, Some(orchestrator_to_explorer_channels)))
    }

    // fn make_planet<T: PlanetAI>(&self, init_sting: String) -> Planet<T> {
    //     let gen_rules = vec![Carbon];
    //     Planet::new(0, PlanetType::C, ai, , comb_rules, orchestrator_channels, explorer_channels)
    // }
}

// struct Dummy;
// impl Dummy{
//     fn new()->Self{
//         Dummy
//     }
// }

/*
    Implementazioni presenti nelle prime versioni dell'orchestrator,
    salvate qua perchè sarebbe utile utilizzarle e avere presente quali funzioni dovremmo implementare
impl OrchestratorTrait for Orchestrator{
    fn combine_resource_request<T, E>(&self, msg: CombineResourceRequest) -> Result<T, E> {
        todo!()
    }
    fn current_planet<T, E>(&self, msg: CurrentPlanetRequest) -> Result<T, E> {
        todo!()
    }
    fn generate_resource_request<T, E>(&self, msg: GenerateResourceRequest) -> Result<T, E> {
        todo!()
    }

    fn make_explorer(&self) -> Explorer {
        todo!()
    }
    fn make_planet<T: PlanetAI>(&self, init_sting: String) -> Planet<T> {
        todo!()
    }
    fn move_to_planet<T, E>(&self, msg: MoveToPlanet) -> Result<T, E> {
        todo!()
    }
    fn reset_explorer_ai<T, E>(&self, msg: ResetExplorerAIMsg, explorer_id: u32) -> Result<T, E> {
        todo!()
    }
    fn send_asteroid<T, E>(&self, a: Asteroid, planet_id: u32) -> Result<T, E> {
        todo!()
    }
    fn send_sunray<T, E>(&self, s: Sunray, planet_id: u32) -> Result<T, E> {
        todo!()
    }
    fn start_game(path: &str) -> Self {
        todo!()
    }
    fn start_planet_ai<T, E>(&self, msg: StartPlanetAiMsg, planet_id: u32) -> Result<T, E> {
        todo!()
    }
    fn stop_planet_ai<T, E>(&self, msg: StopPlanetAiMsg, planet_id: u32) -> Result<T, E> {
        todo!()
    }
    fn supported_combination_request<T, E>(&self, msg: SupportedCombinationRequest)
        -> Result<T, E> {
        todo!()
    }
    fn supported_resource_request<T, E>(&self, msg:SupportedResourceRequest) -> Result<T, E> {
        todo!()
    }

}
    */
