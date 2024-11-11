use actix_web::{web, Scope};
use actix_web::web::Json;
use crate::receipts::service::{ReceiptSummary, ReceiptsService};

fn receipt_routes(service: Box<dyn ReceiptsService>) -> Scope {
   web::scope("/v1/dishes/")
       .app_data(web::Data::new(service))
       .route("rendered", web::get().to(render_dishes))

}

async fn render_dishes(service: web::Data<dyn ReceiptsService>) -> Json<Vec<ReceiptSummary>> {
    todo!()
}
