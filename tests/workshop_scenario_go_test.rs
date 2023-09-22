use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn workshop_go() {
    world().run("scenarios/workshop.scen.json");
}
