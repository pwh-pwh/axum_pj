use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;
use axum::extract::{FromRef, Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_ticket))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    let ticket = mc.create_ticket(ticket_fc).await?;
    Ok(Json(ticket))
}

async fn list_ticket(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    let tickets = mc.list_ticket().await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<usize>,
) -> Result<Json<Ticket>> {
    let ticket = mc.delte_ticket(id).await?;
    Ok(Json(ticket))
}
