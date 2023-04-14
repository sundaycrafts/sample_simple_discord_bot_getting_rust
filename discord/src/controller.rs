mod command_controller;
pub mod dto;

use crate::controller::command_controller::command_controller;
use crate::domains::authorizer::Authorizer;
use anyhow::anyhow;
use http::HeaderMap;
use lambda_http::aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use lambda_http::Body::Text;
use lambda_runtime::{Error, LambdaEvent};

pub struct Controller<A>
where
    A: Authorizer,
{
    authorizer: A,
}

impl<A: Authorizer> Controller<A> {
    pub fn new(authorizer: A) -> Self {
        Controller { authorizer }
    }

    pub async fn handle_event(
        &self,
        event: LambdaEvent<ApiGatewayProxyRequest>,
    ) -> Result<ApiGatewayProxyResponse, Error> {
        let raw_body = event
            .payload
            .body
            .ok_or_else(|| "body is empty".to_string())?;

        self.authorizer
            .authorize(&event.payload.headers, &raw_body)
            .map_err(|e| anyhow!(e.to_string()))?;

        let body = serde_json::from_str::<dto::RequestBody>(&raw_body)?;
        let mut res_header = HeaderMap::new();
        res_header.insert("Content-Type", "application/json".parse().unwrap());

        match body.event_type {
            dto::InteractionType::Ping => {
                let result = serde_json::to_string(&dto::Response {
                    event_type: dto::InteractionResponseType::Pong,
                    data: "Acknowledged".into(),
                })?;

                Ok(ApiGatewayProxyResponse {
                    status_code: 200,
                    multi_value_headers: res_header.clone(),
                    headers: res_header,
                    is_base64_encoded: Some(false),
                    body: Some(Text(result)),
                })
            }

            dto::InteractionType::ApplicationCommand => {
                command_controller(&body.data.name, res_header)
            }

            _ => Err("unsupported event type".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::aws_lambda_events::apigw::{
        ApiGatewayProxyRequest, ApiGatewayProxyRequestContext,
    };
    use lambda_runtime::{Context, LambdaEvent};
    use serde_json::Value;
    use std::collections::HashMap;

    // TODO: implement this test
    #[tokio::test]
    async fn response_is_good_for_simple_input() {
        let id = "ID";

        let mut context = Context::default();
        context.request_id = id.to_string();

        let payload: ApiGatewayProxyRequest<Value> = ApiGatewayProxyRequest {
            resource: None,
            path: None,
            http_method: Default::default(),
            headers: Default::default(),
            multi_value_headers: Default::default(),
            query_string_parameters: Default::default(),
            multi_value_query_string_parameters: Default::default(),
            path_parameters: Default::default(),
            stage_variables: Default::default(),
            request_context: ApiGatewayProxyRequestContext {
                account_id: None,
                resource_id: None,
                operation_name: None,
                stage: None,
                domain_name: None,
                domain_prefix: None,
                request_id: None,
                protocol: None,
                identity: Default::default(),
                resource_path: None,
                path: None,
                authorizer: HashMap::new(),
                http_method: Default::default(),
                request_time: None,
                request_time_epoch: 0,
                apiid: None,
            },
            body: None,
            is_base64_encoded: None,
        };

        let event = LambdaEvent { payload, context };

        // let response = controller(event).await.unwrap();

        // assert_eq!(response.status_code, 200);
    }
}
