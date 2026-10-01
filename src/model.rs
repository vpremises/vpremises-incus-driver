use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ResolvedEnvironment {
    pub schema: String,
    pub environment: Environment,
    pub service: Service,
}

#[derive(Debug, Deserialize)]
pub struct Environment {
    #[serde(rename = "environment_id")]
    pub id: String,
    pub driver: String,
    pub instance_type: String,
    pub resources: Resources,
    pub network: Network,
    pub incus: Option<Incus>,
}

#[derive(Debug, Deserialize)]
pub struct Resources {
    pub cpu: u16,
    pub memory_mib: u32,
    pub root_disk_gib: u32,
}

#[derive(Debug, Deserialize)]
pub struct Network {
    pub public_listener: bool,
}

#[derive(Debug, Deserialize)]
pub struct Incus {
    pub instance_name: String,
    pub project: String,
    pub image: String,
    pub storage_pool: String,
    pub network: String,
}

#[derive(Debug, Deserialize)]
pub struct Service {
    pub service_id: String,
    pub process: Process,
}

#[derive(Debug, Deserialize)]
pub struct Process {
    pub user: String,
}
