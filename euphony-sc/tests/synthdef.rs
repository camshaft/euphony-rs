use euphony_sc::include_synthdef;

include_synthdef!(thing, "../euphony-sc-core/artifacts/v1.scsyndef");

#[test]
fn thing_test() {
    let _ = thing::new().note(1);
}
