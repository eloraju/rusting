use rusty::array_engine::engine::AEngine;
use rusty::engine_core::engine::Engine;
use rusty::test::mocks::mock_board_states::get_test_fen;
fn main() {
    let engine  = AEngine::new();
    engine.print();

    let e2 = AEngine::state_from_fen(&get_test_fen());
    e2.print();
}
