//use std::sync::mpsc;
use crossbeam_channel::{Receiver, Sender, select_biased, unbounded};
use std::collections::HashMap;
use std::thread;

use common_game::components::forge::Forge;
use common_game::components::planet::Planet;
use common_game::protocols::messages::{
    ExplorerToOrchestrator, ExplorerToPlanet, OrchestratorToExplorer, OrchestratorToPlanet,
    PlanetToExplorer, PlanetToOrchestrator,
};

use crate::components::CrabRaveConstructor;
use crate::components::explorer::{BagType, Explorer};

//Types for making things clear in comms initialization
type OPChannels = (Receiver<PlanetToOrchestrator>, Sender<OrchestratorToPlanet>);
type POChannels = (Receiver<OrchestratorToPlanet>, Sender<PlanetToOrchestrator>);
type OEChannels = (
    Receiver<ExplorerToOrchestrator<BagType>>,
    Sender<OrchestratorToExplorer>,
);
type EOChannels = (
    Receiver<OrchestratorToExplorer>,
    Sender<ExplorerToOrchestrator<BagType>>,
);

pub struct PlanetChannels {
    map: HashMap<u32, (OPChannels, Sender<ExplorerToPlanet>)>,
}
impl PlanetChannels {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn send(&self, id: u32, msg: OrchestratorToPlanet) -> Result<(), String> {
        self.map
            .get(&id)
            .ok_or_else(|| format!("Planet {} not found", id))?
            .0
            .1
            .send(msg)
            .map_err(|_| "Planet is disconnected".to_string())
    }

    pub fn try_recv(&self, id: u32) -> Result<PlanetToOrchestrator, String> {
        self.map
            .get(&id)
            .ok_or_else(|| format!("Planet {} not found", id))?
            .0
            .0
            .try_recv()
            .map_err(|_| "Planet is disconnected".to_string())
    }

    pub fn recv(&self, id: u32) -> Result<PlanetToOrchestrator, String> {
        self.map
            .get(&id)
            .ok_or_else(|| format!("Planet {} not found", id))?
            .0
            .0
            .recv()
            .map_err(|_| "Planet is disconnected".to_string())
    }
}

pub struct ExplorerChannels {
    map: HashMap<u32, (OEChannels, Sender<PlanetToExplorer>)>,
}
impl ExplorerChannels {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn send(&self, id: u32, msg: OrchestratorToExplorer) -> Result<(), String> {
        self.map
            .get(&id)
            .ok_or_else(|| format!("Planet {} not found", id))?
            .0
            .1
            .send(msg)
            .map_err(|_| "Explorer is disconnected".to_string())
    }
    pub fn recv(&self, id: u32) -> Result<ExplorerToOrchestrator<BagType>, String> {
        self.map
            .get(&id)
            .ok_or_else(|| format!("Explorer {} not found", id))?
            .0
            .0
            .recv()
            .map_err(|_| "Explorer is disconnected".to_string())
    }
}

// B generic is there for representing the content type of the bag
pub struct Orchestrator {
    pub forge: Forge,
    pub planets_id: Vec<u32>,
    pub planets: Vec<Planet>,
    pub explorers: Vec<Explorer>,

    pub planet_channels: PlanetChannels,
    pub explorer_channels: ExplorerChannels,
}

impl Orchestrator {
    //Check and init orchestrator
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            forge: Forge::new()?,
            planets_id: Vec::new(),
            planets: Vec::new(),
            explorers: Vec::new(),
            planet_channels: PlanetChannels::new(),
            explorer_channels: ExplorerChannels::new(),
        })
    }

    pub fn initialize_galaxy(&mut self /*_path: &str*/) -> Result<(), String> {
        // Orchestrator know the file path where the galaxy topology is written and also the type of each planet
        /*
           Steps of initialization:
           1. read the line to make a planet (at the moment one planet so there is no loop and linear implementation)
           2. generate the id - the id generator methos should be on the orchestrator cause is the one to define everything
           3. generate all the communication channels with the planet
           3. generate the planet - if it fails then handle the error
           4. if planet is generated succefully then add it to the topology
        */

        self.add_explorer(1); //explorer cannot go wrong
        let _init_new_planet = self.add_planet(0)?;
        Ok(())
    }
    pub fn add_planet(&mut self, id: u32) -> Result<(), String> {
        let (
            orchestrator_to_planet_channels,
            planet_to_orchestrator_channels,
            explorer_sender,
            planet_receiver,
        ) = Orchestrator::init_comms_planet();

        //Construct crab-rave planet
        let new_planet =
            CrabRaveConstructor::new(id, planet_to_orchestrator_channels, planet_receiver)?;

        //Map comms for orchestrator
        self.planet_channels.map.insert(
            new_planet.id(),
            (orchestrator_to_planet_channels, explorer_sender),
        );
        //Add new planet id to the list
        self.planets_id.push(new_planet.id());
        //Add new planet to the list
        self.planets.push(new_planet);

        Ok(())
    }
    pub fn add_explorer(&mut self, id: u32) {
        //Create the comms for the new explorer
        let (
            orchestrator_to_explorer_channels,
            explorer_to_orchestrator_channels,
            planet_sender,
            explorer_receiver,
        ) = Orchestrator::init_comms_explorers();

        //Construct Explorer
        let explorer = Explorer::new(
            id,
            None,
            explorer_to_orchestrator_channels,
            explorer_receiver,
        );

        //Map
        self.explorer_channels.map.insert(
            explorer.id(),
            (orchestrator_to_explorer_channels, planet_sender),
        );
        self.explorers.push(explorer);
    }
    fn init_comms_planet() -> (
        OPChannels,
        POChannels,
        Sender<ExplorerToPlanet>,
        Receiver<ExplorerToPlanet>,
    ) {
        //planet-orch and orch-planet
        let (planet_sender, orch_receiver): (
            Sender<PlanetToOrchestrator>,
            Receiver<PlanetToOrchestrator>,
        ) = unbounded();
        let (orch_sender, planet_receiver): (
            Sender<OrchestratorToPlanet>,
            Receiver<OrchestratorToPlanet>,
        ) = unbounded();

        let orchestrator_to_planet_channels = (orch_receiver, orch_sender);
        let planet_to_orchestrator_channels = (planet_receiver, planet_sender);

        //explorer-planet
        let (explorer_sender, planet_receiver): (
            Sender<ExplorerToPlanet>,
            Receiver<ExplorerToPlanet>,
        ) = unbounded();

        (
            orchestrator_to_planet_channels,
            planet_to_orchestrator_channels,
            explorer_sender,
            planet_receiver,
        )
    }

    fn init_comms_explorers() -> (
        OEChannels,
        EOChannels,
        Sender<PlanetToExplorer>,
        Receiver<PlanetToExplorer>,
    ) {
        //explorer-orchestrator and orchestrator-explorer
        let (explorer_sender, orch_receiver): (
            Sender<ExplorerToOrchestrator<BagType>>,
            Receiver<ExplorerToOrchestrator<BagType>>,
        ) = unbounded();
        let (orch_sender, explorer_receiver): (
            Sender<OrchestratorToExplorer>,
            Receiver<OrchestratorToExplorer>,
        ) = unbounded();

        let orchestrator_to_explorer_channels = (orch_receiver, orch_sender);
        let explorer_to_orchestrator_channels = (explorer_receiver, explorer_sender);

        let (planet_sender, explorer_receiver): (
            Sender<PlanetToExplorer>,
            Receiver<PlanetToExplorer>,
        ) = unbounded();

        (
            orchestrator_to_explorer_channels,
            explorer_to_orchestrator_channels,
            planet_sender,
            explorer_receiver,
        )
    }

    //The return is Result<(), String> because if an error occur it go back to the main that finishes
    // I don't know if there are better approach but I think it is pretty elegant
    pub fn run(&mut self) -> Result<(), String> {
        let mut planet1 = match self.planets.pop() {
            Some(p) => p,
            None => return Err("Cannot find any planet to pop".to_string()),
        };

        println!("Creating planet thread...");
        thread::spawn(move || -> Result<(), String> {
            println!("Planet running...");
            let _success = planet1.run()?;
            Ok(())
        });

        println!("Start Planet...");
        let id = self.planets_id[0];
        let _planet_start = self
            .planet_channels
            .send(id, OrchestratorToPlanet::StartPlanetAI)?;
        loop {
            println!("Receive planet messages...");
            let _planet_response = self.planet_channels.try_recv(id)?;

            println!("Send Asteroid to Planet");
            let _planet_message = self.planet_channels.send(
                id,
                OrchestratorToPlanet::Asteroid(self.forge.generate_asteroid()),
            )?;

            let _planet_response = self.planet_channels.recv(id)?;
            println!("Planet should have finished running...");

            let _planet_message = match self.planet_channels.send(
                id,
                OrchestratorToPlanet::Asteroid(self.forge.generate_asteroid()),
            ) {
                Ok(_) => println!("PLANET STILL WORKING..."),
                Err(_) => {
                    println!("Everthing is okey...");
                    break;
                }
            };
        }
        Ok(())
    }
}