use common_game::components::generator::Generator;

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
pub struct Orchestrator {
    generator: Generator,
}

impl Orchestrator {
    //Check and init orchestrator
    pub fn new() -> Result<Self, String> {
        let generator = Generator::new()?;
        Ok(Orchestrator {
            generator: generator,
        })
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



// fn initialize_galaxy(&mut self, path: &str) -> impl GalaxyTrait {
//     Dummy::new()
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
