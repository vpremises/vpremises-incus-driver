use vpremises_incus_driver::{ResolvedEnvironment, render};

fn fixture() -> ResolvedEnvironment {
    serde_json::from_str(include_str!("fixtures/resolved.json")).unwrap()
}

#[test]
fn renders_restricted_project_and_bounded_instance() {
    let rendered = render(&fixture()).unwrap();
    let project = &rendered["resource"]["incus_project"]["managed"];
    let instance = &rendered["resource"]["incus_instance"]["managed"];
    assert_eq!(project["config"]["restricted"], "true");
    assert_eq!(project["force_destroy"], false);
    assert_eq!(instance["type"], "virtual-machine");
    assert_eq!(instance["config"]["boot.autostart"], "true");
    assert_eq!(instance["wait_for"][0]["type"], "cloud-init");
}

#[test]
fn rejects_non_cloud_image() {
    let mut raw: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/resolved.json")).unwrap();
    raw["environment"]["incus"]["image"] = "images:debian/13".into();
    let invalid: ResolvedEnvironment = serde_json::from_value(raw).unwrap();
    assert!(render(&invalid).is_err());
}
