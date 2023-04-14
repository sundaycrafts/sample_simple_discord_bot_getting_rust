use discord_command::Command;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum InteractionType {
    /**
     * A ping.
     */
    Ping = 1,
    /**
     * A command invocation.
     */
    ApplicationCommand = 2,
    /**
     * Usage of a message's component.
     */
    MessageComponent = 3,
    /**
     * An interaction sent when an application command option is filled out.
     */
    ApplicationCommandAutocomplete = 4,
    /**
     * An interaction sent when a modal is submitted.
     */
    ModalSubmit = 5,
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum InteractionResponseType {
    /**
     * Acknowledge a `PING`.
     */
    Pong = 1,
    /**
     * Respond with a message, showing the user's input.
     */
    ChannelMessageWithSource = 4,
    /**
     * Acknowledge a command without sending a message, showing the user's input. Requires follow-up.
     */
    DeferredChannelMessageWithSource = 5,
    /**
     * Acknowledge an interaction and edit the original message that contains the component later; the user does not see a loading state.
     */
    DeferredUpdateMessage = 6,
    /**
     * Edit the message the component was attached to.
     */
    UpdateMessage = 7,
    /*
     * Callback for an app to define the results to the user.
     */
    ApplicationCommandAutocompleteResult = 8,
    /*
     * Respond with a modal.
     */
    Modal = 9,
}

#[derive(Deserialize, Debug)]
pub struct RequestBodyData {
    pub name: Command,
}

#[derive(Deserialize, Debug)]
pub struct RequestBody {
    #[serde(rename = "type")]
    pub event_type: InteractionType,
    pub data: RequestBodyData,
}

#[derive(Serialize, Debug)]
pub struct ResponseData {
    pub content: String,
}

impl From<&str> for ResponseData {
    fn from(value: &str) -> Self {
        ResponseData {
            content: value.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct Response {
    #[serde(rename = "type")]
    pub event_type: InteractionResponseType,
    pub data: ResponseData,
}
