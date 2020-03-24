use crate::schema::{Context, Schema};
use actix_web::{web, Error, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

pub async fn handle_graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn handle_graphql(
    schema: web::Data<Arc<Schema>>,
    context: web::Data<Arc<Context>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let resp = data.execute(&schema, &context);

        Ok::<_, serde_json::error::Error>(serde_json::to_string(&resp)?)
    })
    .await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(result))
}
