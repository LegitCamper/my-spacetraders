// use my_spacetraders::interface::{
//     parse_waypoint, BuyShip, Coordinates, Credentials, Method, SpaceTraders,
// };

#[tokio::main]
async fn main() {
    let credentials = Credentials::new(
        "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZGVudGlmaWVyIjoiTEVHSVRDQU1QRVIiLCJpYXQiOjE2ODM2NTE0MDcsInN1YiI6ImFnZW50LXRva2VuIn0.fB1FkKfy57TZZCy9DNBm24aIRrYwxM1JbuHWd7sA6qdDYVbkCVwVliEP27nzkU3fcY2XbL0InUxsx-biV9Ux9fJcPUzyOEj92X_I8ZzxNTIrj4J7g7Zyp1Hbpr9056BmrshkrzeX65PspVZvSPmXLsntFiZsF1ncmlgnjePElfUcUdY2wY12xgMjne6sJXueDGNzEXSrkARDexCeogfwYXo3RxfVtwtx6mxI3z7hPaa80c8cHhvTteKLJ3eioIWhs8Yv4xnaQIwakLQcvyrVNmFPYl5mjOMt9rBzTP5cGpcko7AXlzNfpXdFK6O_3fU1PgwhutuFTZPKFCMDLetK2aEwp9f5Rg_KHaCglUMkkjsvJw1AGYcSKVP1eLB5KTxRw9UnNwAwi4ocGKNm9AlaJionFUIZXpRCOs9T6pnntV5IdGuNlZ5JR40buaBA6I3g5Lqa_Sg7g1NVF9Wt3ZgrgpKed4l7frd6mupq1JE2sm0-XVYH7f6H4Cx3Em2uzNxe"
    );


    let interface_handler = InterfaceHandler(credentials);

    let interface_handler_sender = interface_handler.sender.send("hello from main".to_String())




    

    // let space_traders = SpaceTraders::new(credentials);

    // prints agent info
    // let agent_info = space_traders.agent_details().await;
    // println!("{:?}", agent_info);

    // println!("{:?}", );

    // gets all waypoints in a system
    // let waypoints = space_traders
    //     .waypoint_list(parse_waypoint(agent_info.data.headquarters).system)
    //     .await;

    // lists all ships to buy in a junkyard
    // for i in waypoints.data.iter() {
    //     for o in i.traits.iter() {
    //         if o.symbol == my_spacetraders::interface::WaypointTrait::Shipyard {
    //             println!("{:?}", i);

    //             println!(
    //                 "{}",
    //                 space_traders
    //                     .waypoint_custom(i.system_symbol.clone(), i.symbol.clone(), "shipyard")
    //                     .await
    //             );
    //         }
    //     }
    // }

    // println!(
    //     "{}",
    //     space_traders
    //         .make_reqwest(
    //             Method::Post,
    //             "/my/ships",
    //             Some(space_traders.make_json(BuyShip {
    //                 shipType: "SHIP_MINING_DRONE".to_string(),
    //                 waypointSymbol: "X1-DF55-69207D".to_string()
    //             }))
    //         )
    //         .await
    // );

    // println!(
    //     "{:?}",
    //     space_traders
    //         .contract_accept("clhgikslz0bjxs60dwfqg3zto")
    //         .await
    // );




















    
}
