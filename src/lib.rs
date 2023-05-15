pub mod interface;

use interface::{parse_waypoint, Credentials, SpaceTradersHandler};

// use std::sync::Arc;
// use tokio::sync::broadcast;

async fn create_interface() -> SpaceTradersHandler {
    let credentials = Credentials::new(
        "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZGVudGlmaWVyIjoiTEVHSVRDQU1QRVIiLCJpYXQiOjE2ODQwMDI0NjksInN1YiI6ImFnZW50LXRva2VuIn0.MoA89hJwCWYTRpJNvh7GXCF5zRpW26yvZH2EWoAbqsO-zNOjVCmlLw2OLUllh4IWrAmZ6gI4F95I-JspDj_ustl6d6FltXJGcA28KHf-ieU3sl3FUqC7QxLCCtTUioAVZDIqQGDg1gFyAFiykHMWNR2f60ip-aj3sZ178a5N5f5ETXNP2il-pHaujE5ZZyvT_WHNbeGcAO-Wq2qjOhSKgOpMEYfYnL2TqqZu2Km3tRPoik6JiunCMfsjvfnNmc2T0SCreoTKUaM7OASpOhJyrnRCqLDVafHXbVtEZTjjhAXpp62w7bYzPyzdQ0olGDEpMCRFwM7Q3qGLNjmUthrrEw"
    );

    SpaceTradersHandler::new(credentials).await
}

pub async fn main_algo() {
    let interface_sender = create_interface().await;

    complete_contracts(interface_sender).await;
}

async fn complete_contracts(interface: SpaceTradersHandler) {
    println!("{:?}", interface.agent().await);
    println!("{:?}", interface.contract_list().await);
    if let Some(system) = interface.agent().await {
        println!(
            "{:?}",
            interface
                .list_waypoints(&parse_waypoint(system.data.headquarters).system)
                .await
        )
    }
}
