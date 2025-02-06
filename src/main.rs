use game_test::game_test_controller::GameTestController;

mod game_test;

fn main() {
    let mut example_app_controller = GameTestController::new();
    example_app_controller.init();
}
