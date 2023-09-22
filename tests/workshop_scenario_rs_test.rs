use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");

    blockchain.register_contract("file:output/workshop.wasm", workshop::ContractBuilder);
    blockchain
}

#[test]
fn workshop_rs() {
    world().run("scenarios/workshop.scen.json");
}
