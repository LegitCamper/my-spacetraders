use  my_spacetraders::{SpaceTraders, Credentials};

#[tokio::main]
async fn main() {
    let credentials = Credentials::new(
        "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZGVudGlmaWVyIjoiTEVHSVRDQU1QRVIiLCJpYXQiOjE2ODM2NTE0MDcsInN1YiI6ImFnZW50LXRva2VuIn0.fB1FkKfy57TZZCy9DNBm24aIRrYwxM1JbuHWd7sA6qdDYVbkCVwVliEP27nzkU3fcY2XbL0InUxsx-biV9Ux9fJcPUzyOEj92X_I8ZzxNTIrj4J7g7Zyp1Hbpr9056BmrshkrzeX65PspVZvSPmXLsntFiZsF1ncmlgnjePElfUcUdY2wY12xgMjne6sJXueDGNzEXSrkARDexCeogfwYXo3RxfVtwtx6mxI3z7hPaa80c8cHhvTteKLJ3eioIWhs8Yv4xnaQIwakLQcvyrVNmFPYl5mjOMt9rBzTP5cGpcko7AXlzNfpXdFK6O_3fU1PgwhutuFTZPKFCMDLetK2aEwp9f5Rg_KHaCglUMkkjsvJw1AGYcSKVP1eLB5KTxRw9UnNwAwi4ocGKNm9AlaJionFUIZXpRCOs9T6pnntV5IdGuNlZ5JR40buaBA6I3g5Lqa_Sg7g1NVF9Wt3ZgrgpKed4l7frd6mupq1JE2sm0-XVYH7f6H4Cx3Em2uzNxe"
    );

    let space_traders = SpaceTraders::new(credentials);
    let current_waypoint = space_traders.agent_details().await;
    
    println!("{:?}", current_waypoint)
}
