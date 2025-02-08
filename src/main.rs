use rusted_engine::engine_controller::EngineController;

mod rusted_engine;

fn main() {
    let mut example_app_controller = EngineController::new();
    example_app_controller.init();
}
