use super::dto::{InteractionResponseType, Response};
use discord_command::Command;
use http::HeaderMap;
use lambda_http::aws_lambda_events::apigw::ApiGatewayProxyResponse;
use lambda_runtime::Error;

pub fn command_controller(
    command: &Command,
    headers: HeaderMap,
) -> Result<ApiGatewayProxyResponse, Error> {
    match command {
        Command::Ask => Ok(ApiGatewayProxyResponse {
            status_code: 200,
            multi_value_headers: headers.clone(),
            headers: headers.clone(),
            is_base64_encoded: Some(false),
            body: Some(
                serde_json::to_string(&Response {
                    event_type: InteractionResponseType::ChannelMessageWithSource,
                    data: "<Response Here>".into(),
                })?
                .into(),
            ),
        }),
    }
}
