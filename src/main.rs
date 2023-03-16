use futures::StreamExt;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::ListParams,
    runtime::controller::{Action, Controller},
    Api, Client, ResourceExt,
};
use std::{sync::Arc, time::Duration};
mod spec;
use spec::Zeppelin;

#[derive(thiserror::Error, Debug)]
pub enum CrdError {}
pub type Result<T, E = CrdError> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let client = Client::try_default().await?;
    let crds = Api::<Zeppelin>::all(client.clone());

    // For all resources that the CRD creates, we should watch them
    let pods = Api::<Pod>::all(client.clone());

    // Shared state
    let state = Arc::new(());

    Controller::new(crds, ListParams::default())
        .owns(pods, ListParams::default())
        .run(reconcile, error_policy, state)
        .for_each(|_| futures::future::ready(()))
        .await;

    Ok(())
}

async fn reconcile(obj: Arc<Zeppelin>, _ctx: Arc<()>) -> Result<Action> {
    println!("reconcile request: {}", obj.name_any());
    println!("reconcile crd with name: {}", &obj.spec.name);
    Ok(Action::requeue(Duration::from_secs(3600)))
}

fn error_policy(_object: Arc<Zeppelin>, _err: &CrdError, _ctx: Arc<()>) -> Action {
    Action::requeue(Duration::from_secs(5))
}
