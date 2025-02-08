use game_test::engine_controller::EngineController;

mod game_test;

fn main() {
    let mut example_app_controller = EngineController::new();
    example_app_controller.init();
}
