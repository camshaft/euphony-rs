use euphony_sc::include_synthdef;

include_synthdef!("../euphony-sc-core/artifacts/v1.scsyndef" as thing);

#[test]
fn thing_test() {
    thing::new().note(1);
}
