//use std::sync::mpsc;
use crossbeam_channel::{Receiver, Sender, select_biased, unbounded};
use std::collections::HashMap;
use std::thread;

use common_game::components::forge::Forge;
use common_game::components::planet::{self, Planet};
use common_game::protocols::messages::{
    ExplorerToOrchestrator, ExplorerToPlanet, OrchestratorToExplorer, OrchestratorToPlanet,
    PlanetToExplorer, PlanetToOrchestrator,
};

use crate::components::CrabRaveConstructor;
use crate::components::explorer::{BagType, Explorer};

//Types for making things clear in comms initialization
type OPChannels = (Receiver<PlanetToOrchestrator>, Sender<OrchestratorToPlanet>);
type POChannels = (Receiver<OrchestratorToPlanet>, Sender<PlanetToOrchestrator>);
type EPChannels = (Receiver<PlanetToExplorer>, Sender<ExplorerToPlanet>);
type PEChannels = (Receiver<ExplorerToPlanet>, Sender<PlanetToExplorer>);
type OEChannels = (
    Receiver<ExplorerToOrchestrator<BagType>>,
    Sender<OrchestratorToExplorer>,
);
type EOChannels = (
    Receiver<OrchestratorToExplorer>,
    Sender<ExplorerToOrchestrator<BagType>>,
);

struct CustomHashMap<T, U, V> {
    map:HashMap<u32, ((Receiver<T>, Sender<U>), Sender<V>)>
};
impl<T, U, V> CustomHashMap<T, U, V> {
    fn send(&self, id:u32, msg: U) -> Result<(), String> {
        self.map.get(&id)
            .ok_or_else(|| format!("Planet {} not found", id))?
            .0.1.send(msg)
            .map_err(|_| "Channel closed".to_string())
    }
}

// B generic is there for representing the content type of the bag
pub struct Orchestrator {
    pub forge: Forge,
    pub planets_id: Vec<u32>,
    pub planets: Vec<Planet>,
    pub explorers: Vec<Explorer>,

    pub planet_channels: HashMap<u32, (OPChannels, Sender<ExplorerToPlanet>)>,
    pub explorer_channels: HashMap<u32, (OEChannels, Sender<PlanetToExplorer>)>,
}

impl Orchestrator {
    //Check and init orchestrator
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            forge: Forge::new()?,
            planets_id: Vec::new(),
            planets: Vec::new(),
            explorers: Vec::new(),
            planet_channels: HashMap::new(),
            explorer_channels: HashMap::new(),
        })
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
        self.explorer_channels.insert(
            explorer.id(),
            (orchestrator_to_explorer_channels, planet_sender),
        );
        self.explorers.push(explorer);
    }

    fn add_planet(&mut self, id: u32) -> Result<(), String> {
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
        self.planet_channels.insert(
            new_planet.id(),
            PChannels(orchestrator_to_planet_channels, explorer_sender),
        );
        //Add new planet id to the list
        self.planets_id.push(new_planet.id());
        //Add new planet to the list
        self.planets.push(new_planet);

        Ok(())
    }

    //The return is Result<(), String> because if an error occur it go back to the main that finishes
    // I don't know if there are better approach but I think it is pretty elegant

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

        self.add_explorer(0);
        self.add_planet(2);
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        // let mut planet1 = match self.planets.pop() {
        //     Some(p) => p,
        //     None => return Err("Cannot find any planet to pop".to_string()),
        // };

        // println!("Creating planet thread...");
        // thread::spawn(move || -> Result<(), String> {
        //     println!("Planet running...");
        //     let success = planet1.run()?;
        //     Ok(())
        // });

        // println!("Start Planet...");
        // let only_planet = self.planets_id[0];
        // let only_comms = match self.planet_channels.get(&only_planet){
        //     None=> return Err("Error channels...")
        // }
        // let start_planet = self
        //     .planet_channels.get(&only_planet).unwrap().send(OrchestratorToPlanet::StartPlanetAI);

        // // loop{
        // // println!("Receive planet messages...");
        // // let planet_response = match self.planet_channels.0.try_recv(){
        // //     Ok(res)=>res,
        // //     Err(_)=>return Err("Planet is disconnected\n".to_string())
        // // };

        // println!("Send Asteroid to Planet");
        // let planet_message = self.planet_channels.send(OrchestratorToPlanet::Asteroid(
        //     self.forge.generate_asteroid(),
        // ));
        // let planet_response = match self.planet_channels.0.recv() {
        //     Ok(res) => res,
        //     Err(_) => {
        //         return Err("Planet is disconnected".to_string());
        //     }
        // };
        // println!("Planet should have finished running...");

        // let planet_message = match self.planet_channels.1.send(OrchestratorToPlanet::Asteroid(
        //     self.forge.generate_asteroid(),
        // )) {
        //     Ok(_) => println!("PLANET STILL WORKING..."),
        //     Err(_) => {
        //         println!("Everthing is okey...");
        //         // break;
        //     }
        // };
        // // }
        // // let result = spawning_thread.join().expect("Something went wrong in the thread...");
        Ok(())
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
