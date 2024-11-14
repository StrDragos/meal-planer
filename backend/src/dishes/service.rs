// use anyhow::Error;
// use serde::Serialize;
// use uuid::Uuid;
//
// #[derive(Serialize, Debug, PartialEq)]
// pub struct ReceiptSummary {
//     id: Uuid,
//     summary: String,
//     title: String,
//     tags: Vec<Tag>
// }
//
// #[derive(Serialize, Debug, PartialEq)]
// pub struct Tag {
//     name: String
// }
//
// #[derive(Serialize, Debug, PartialEq)]
// pub struct CookingStep {
//
// }
//
// #[derive(Serialize, Debug, PartialEq, Eq, Hash)]
// pub struct UserId(String);
//
// pub trait DishService {
//     async fn list(user_id: UserId, get_owned: bool) -> Result<Vec<ReceiptSummary>, anyhow::Error>;
//
//     async fn save(&self, receipt: ReceiptSummary) -> Result<(), anyhow::Error>;
//
//     async fn delete(&self, receipt: ReceiptSummary) -> Result<(), anyhow::Error>;
// }
//
// pub struct DishServiceLive {}
// impl DishService for DishServiceLive {
//     async fn list(user_id: UserId, get_owned: bool) -> Result<Vec<ReceiptSummary>, Error> {
//         Ok(vec![])
//     }
//
//     async fn save(&self, receipt: ReceiptSummary) -> Result<(), Error> {
//         todo!()
//     }
//
//     async fn delete(&self, receipt: ReceiptSummary) -> Result<(), Error> {
//         todo!()
//     }
// }