use std::collections::{HashMap, HashSet};
use std::sync::{mpsc, LockResult};
// use std::time::SystemTime;
use common_game::components::planet::{Planet, PlanetAI, PlanetState, PlanetType};
use common_game::components::resource::BasicResourceType::Carbon;
use common_game::components::resource::ComplexResourceType::Diamond;
use common_game::components::resource::{BasicResource, BasicResourceType, Combinator, ComplexResource, ComplexResourceRequest, Dolphin, Generator, GenericResource};
use common_game::components::rocket::Rocket;
use common_game::protocols::messages::{
    ExplorerToPlanet, OrchestratorToPlanet, PlanetToExplorer, PlanetToOrchestrator,
};
use common_game::protocols::messages::OrchestratorToPlanet::Asteroid;
use crate::components::planet::stacks::{CHARGED_CELL_STACK, FREE_CELL_STACK};

///////////////////////////////////////////////////////////////////////////////////////////
// CrabRave Constructor
///////////////////////////////////////////////////////////////////////////////////////////
pub struct CrabRaveConstructor;

impl CrabRaveConstructor {
    pub fn new(
        id: u32,
        orchestrator_channels: (
            mpsc::Receiver<OrchestratorToPlanet>,
            mpsc::Sender<PlanetToOrchestrator>,
        ),
        explorer_channels: mpsc::Receiver<ExplorerToPlanet>,
    ) -> Result<Planet, String> {
        let (planet_type, ai, gen_rules, comb_rules, orchestrator_channels, explorer_channels) = (
            PlanetType::C,
            AI::new(),
            vec![Carbon],
            vec![Diamond],
            orchestrator_channels,
            explorer_channels,
        );
        let new_planet = Planet::new(
            id,
            planet_type,
            Box::new(ai),
            gen_rules,
            comb_rules,
            orchestrator_channels,
            explorer_channels,
        )?;
        Ok(new_planet)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////
// PlanetAI
///////////////////////////////////////////////////////////////////////////////////////////

const N_CELLS: usize = 5; // based on the planet

pub struct AI;

impl AI {
    fn new() -> Self {
        // the cell stack needs to be started here
        // otherwise it would get reset when the AI
        // gets stopped
        initialize_free_cell_stack(); // REVIEW rimuovere se si sceglie la vecchia implementazione
        Self
    }
}

impl PlanetAI for AI {
    fn handle_orchestrator_msg(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        msg: OrchestratorToPlanet,
    ) -> Option<PlanetToOrchestrator> {
        match msg {
            OrchestratorToPlanet::InternalStateRequest => {
                Some(PlanetToOrchestrator::InternalStateResponse {
                    planet_id: state.id().clone(),
                    planet_state: PlanetState::to_dummy(&state)
                                                 // timestamp: SystemTime::now(),
                })
            }
            OrchestratorToPlanet::Sunray(sunray) => {
                // for i in 0..N_CELLS {
                //     // non ho trovato un modo per ottenere il vettore, l'unico modo penso sia quello di ciclare
                //     if !state.cell(i).is_charged() {
                //         state.cell_mut(i).charge(sunray);
                //         return Some(PlanetToOrchestrator::SunrayAck {
                //             planet_id: state.id(),
                //             // timestamp: SystemTime::now(),
                //         });
                //     }
                // }
                // None
                if let Some(idx) = get_free_cell_index() {
                    state.cell_mut(idx as usize).charge(sunray);
                    push_charged_cell(idx);
                    return Some(PlanetToOrchestrator::SunrayAck {
                        planet_id: state.id().clone(),
                    })
                }
                None
            }
            _ => None,
        }
    }

    fn handle_explorer_msg(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        msg: ExplorerToPlanet,
    ) -> Option<PlanetToExplorer> {
        match msg {
            ExplorerToPlanet::AvailableEnergyCellRequest { explorer_id: _ } => {
                // restituisce la prima cell carica, se c'è
                // DO NOT REMOVE -> the following commented lines are the old implementation, so do not remove them till the final decision of the implementation
                // for i in 0..N_CELLS {
                //     if state.cell(i).is_charged() {
                //         return Some(PlanetToExplorer::AvailableEnergyCellResponse {
                //             available_cells: i as u32,
                //         });
                //     }
                // }
                // None
                if let Some(idx) = peek_charged_cell_index() {
                    return Some(PlanetToExplorer::AvailableEnergyCellResponse {
                        available_cells: idx
                    })
                }
                None
            }
            ExplorerToPlanet::SupportedResourceRequest { explorer_id: _ } => {
                Some(PlanetToExplorer::SupportedResourceResponse {
                    resource_list: generator.all_available_recipes()
                })
            }
            ExplorerToPlanet::SupportedCombinationRequest { explorer_id: _ } => {
                Some(PlanetToExplorer::SupportedCombinationResponse {
                    combination_list: combinator.all_available_recipes() 
                })
            }

            //TODO use explorer_id to send the gen resource to correct Explorer
            ExplorerToPlanet::GenerateResourceRequest {
                explorer_id,
                resource,
            } => {
                let requested_resource = resource;
                // controllo se c'è una cella carica
                // DO NOT REMOVE -> the following commented lines are the old implementation, so do not remove them till the final decision of the implementation
                // let cell_idx = (0..N_CELLS).find(|&i| state.cell(i).is_charged());
                // if let Some(cell_idx) = cell_idx {
                if let Some(cell_idx) = get_charged_cell_index() {
                    // se c'è una cella carica
                    // ottengo la cella da passare al generator
                    let cell = state.cell_mut(cell_idx as usize); // TODO remove the "as usize" if using the old implementation of getting the index of energy cell
                    // pattern matching per generare la risorsa corretta
                    let generated_resource = match requested_resource {
                        BasicResourceType::Carbon => {
                            generator.make_carbon(cell).map(BasicResource::Carbon)
                        } // make_ controlla già se la risorsa è presente in generator
                        BasicResourceType::Silicon => {
                            generator.make_silicon(cell).map(BasicResource::Silicon)
                        }
                        BasicResourceType::Oxygen => {
                            generator.make_oxygen(cell).map(BasicResource::Oxygen)
                        }
                        BasicResourceType::Hydrogen => {
                            generator.make_hydrogen(cell).map(BasicResource::Hydrogen)
                        }
                    };
                    // verifico il risultato di state.generator.make...
                    match generated_resource {
                        Ok(resource) => {
                            push_free_cell(cell_idx);
                            return Some(PlanetToExplorer::GenerateResourceResponse {
                                resource: Some(resource),
                            });
                        }
                        Err(err) => {
                            push_charged_cell(cell_idx);
                            println!("{}", err);
                        }
                    }
                } else {
                    println!("No available cell found"); // non dovrebbe accadere, si spera che l'explorer chieda se ce ne è una libera
                }
                Some(PlanetToExplorer::GenerateResourceResponse {
                    //TA: TODO ritorno come ho fatto o direttamente None?
                    //DDC: io terrei cosi', esplicita il fatto che questo sia un caso
                    //di errore ma comunque atteso. dipende anche dalla spec
                    resource: None,
                })
            }
            //TODO use explorer_id to send the gen resource to correct Explorer
            ExplorerToPlanet::CombineResourceRequest { explorer_id, msg } => {
                // searching the index of the first free cell
                // DO NOT REMOVE -> the following commented lines are the old implementation, so do not remove them till the final decision of the implementation
                // let cell_idx = (0..N_CELLS).find(|&i| state.cell(i).is_charged());
                // if let Some(cell_idx) = cell_idx {
                if let Some(cell_idx) = get_charged_cell_index() {
                    let cell = state.cell_mut(cell_idx as usize); // TODO remove the "as usize" if using the old implementation of getting the index of energy cell
                    // pattern matching to generate the correct resource
                    let complex_resource: Result<ComplexResource, (String, GenericResource, GenericResource)> = match msg {
                        ComplexResourceRequest::Water(r1, r2) => combinator
                            .make_water(r1, r2, cell)
                            .map(ComplexResource::Water)
                            .map_err(|(e, r1, r2)| { (e , GenericResource::BasicResources(BasicResource::Hydrogen(r1)), GenericResource::BasicResources(BasicResource::Oxygen(r2)))}),
                        ComplexResourceRequest::Diamond(r1, r2) => combinator
                            .make_diamond(r1, r2, cell)
                            .map(ComplexResource::Diamond)
                            .map_err(|(e, r1, r2)| { (e , GenericResource::BasicResources(BasicResource::Carbon(r1)), GenericResource::BasicResources(BasicResource::Carbon(r2)))}),
                        ComplexResourceRequest::Life(r1, r2) => combinator
                            .make_life(r1, r2, cell)
                            .map(ComplexResource::Life)
                            .map_err(|(e, r1, r2)| { (e , GenericResource::ComplexResources(ComplexResource::Water(r1)), GenericResource::BasicResources(BasicResource::Carbon(r2)))}),

                        ComplexResourceRequest::Robot(r1, r2) => combinator
                            .make_robot(r1, r2, cell)
                            .map(ComplexResource::Robot)
                            .map_err(|(e, r1, r2)| { (e , GenericResource::BasicResources(BasicResource::Silicon(r1)), GenericResource::ComplexResources(ComplexResource::Life(r2)))}),

                        ComplexResourceRequest::Dolphin(r1, r2) => combinator
                            .make_dolphin(r1, r2, cell)
                            .map(ComplexResource::Dolphin)
                            .map_err(|(e, r1, r2)| { (e , GenericResource::ComplexResources(ComplexResource::Water(r1)), GenericResource::ComplexResources(ComplexResource::Life(r2)))}),

                        ComplexResourceRequest::AIPartner(r1, r2) => combinator
                            .make_aipartner(r1, r2, cell)
                            .map(ComplexResource::AIPartner)
                            .map_err(|(e, r1, r2)| { (e , GenericResource::ComplexResources(ComplexResource::Robot(r1)), GenericResource::ComplexResources(ComplexResource::Diamond(r2)))}),

                    };
                    // checking the result of complex_resource
                    return match complex_resource {
                        Ok(resource) => {
                            push_free_cell(cell_idx);
                            Some(PlanetToExplorer::CombineResourceResponse {
                                complex_response: Ok(resource),
                            })
                        }
                        Err(err) => {
                            push_charged_cell(cell_idx);
                            println!("{}", err.0);
                            Some(PlanetToExplorer::CombineResourceResponse {
                                complex_response: Err(err),
                            })
                        }
                    }
                } else {
                    println!("No available cell found");
                    let (ret1,ret2) = match msg {
                        ComplexResourceRequest::Water(r1, r2) => {
                            (GenericResource::BasicResources(BasicResource::Hydrogen(r1)),
                            GenericResource::BasicResources(BasicResource::Oxygen(r2)))
                        }
                        ComplexResourceRequest::AIPartner(r1, r2) => {
                            (GenericResource::ComplexResources(ComplexResource::Robot(r1)),
                            GenericResource::ComplexResources(ComplexResource::Diamond(r2)))
                        }
                        ComplexResourceRequest::Life(r1, r2) => {
                            (GenericResource::ComplexResources(ComplexResource::Water(r1)),
                            GenericResource::BasicResources(BasicResource::Carbon(r2)))
                        }
                        ComplexResourceRequest::Diamond(r1, r2) => {
                            (GenericResource::BasicResources(BasicResource::Carbon(r1)),
                            GenericResource::BasicResources(BasicResource::Carbon(r2)))
                        }
                        ComplexResourceRequest::Dolphin(r1, r2) => {
                            (GenericResource::ComplexResources(ComplexResource::Water(r1)),
                            GenericResource::ComplexResources(ComplexResource::Life(r2)))
                        }
                        ComplexResourceRequest::Robot(r1, r2) => {
                            (GenericResource::BasicResources(BasicResource::Silicon(r1)),
                            GenericResource::ComplexResources(ComplexResource::Life(r2)))
                        }
                    };
                    return Some(PlanetToExplorer::CombineResourceResponse { complex_response: Err(("no available cell".to_string(),ret1, ret2)) });
                }
                None
            }
        }
    }

    fn handle_asteroid(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
    ) -> Option<Rocket> {
        //if the planet can't build rockets, you're screwed
        if !state.can_have_rocket() {
            return None;
        }

        //if you've already got a rocket ready, use it!
        if state.has_rocket() {
            return state.take_rocket();
        }

        //try to build a rocket if you have any energy left
        if let Some(idx) = get_charged_cell_index() {
            match state.build_rocket(idx as usize) {
                Ok(_) => {
                    push_free_cell(idx);
                    match state.cell_mut(idx as usize).discharge() {
                        //build was successful, log the rocket creation
                        Ok(_) => {
                            println!("Used a charged cell at index {}, to build a rocket", idx);
                        }
                        Err(err) => {
                            println!("{}", err);
                        }
                    }
                    return state.take_rocket();
                }
                //build failed, log the error and return none
                Err(err) => {
                    push_free_cell(idx);
                    println!("{}", err);
                    return None;
                }
            }
        }
        //shouldn't be able to get here, but just in case...
        None
    }

    fn start(&mut self, state: &PlanetState) {
        println!("Planet {} AI started", state.id());
        // TODO non ho capito bene cosa deve fare planet.ai.start, deve creare il thread o lo fa l'orchestrator?
        // Mi sembra che lo start AI semplicemente dia il via al loop che permette l'AI di gestire le azioni
        // TODO non so se ha senso mettere l'inizializzazione degli stack qui o se va messa quando creaiamo AI
        // initialize_free_cell_stack() // TODO remove if the choice is the old implementation
    }

    fn stop(&mut self, _state: &PlanetState) {
        println!("Planet AI stopped");
        // TODO stessa cosa di "start"
    }
}

mod stacks {
    use std::sync::Mutex;
    pub(super) static FREE_CELL_STACK: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    pub(super) static CHARGED_CELL_STACK: Mutex<Vec<u32>> = Mutex::new(Vec::new());
}

fn initialize_free_cell_stack(){
    let free_cell_stack = FREE_CELL_STACK.lock();
    match free_cell_stack {
        Ok(mut vec) => {
            for i in 0..N_CELLS {
                vec.push(i as u32);
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn get_free_cell_index() -> Option<u32> {
    let free_cell_stack = FREE_CELL_STACK.lock();
    match free_cell_stack {
        Ok(mut vec) => {
            vec.pop()
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

fn get_charged_cell_index() -> Option<u32> {
    let charged_cell_stack = CHARGED_CELL_STACK.lock();
    match charged_cell_stack {
        Ok(mut vec) => {
            vec.pop()
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

fn push_free_cell(index: u32) {
    let free_cell_stack = FREE_CELL_STACK.lock();
    match free_cell_stack {
        Ok(mut vec) => {
            vec.push(index);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn push_charged_cell(index: u32) {
    let charged_cell_stack = CHARGED_CELL_STACK.lock();
    match charged_cell_stack {
        Ok(mut vec) => {
            vec.push(index);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn peek_charged_cell_index() -> Option<u32> {
    let charged_cell_stack = CHARGED_CELL_STACK.lock();
    match charged_cell_stack {
        Ok(vec) => {
            vec.last().copied()
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

