use std::sync::Arc;

use axum::{extract::{ws::{Message, Utf8Bytes, WebSocket}, Query, State, WebSocketUpgrade}, http::StatusCode, response::IntoResponse};
use futures_util::{SinkExt, StreamExt};

use crate::{app_state::AppState, auth::ticket::TicketQuery, models::user::UserID};



pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(ticket_query): Query<TicketQuery>,
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    let user_id = match state.tickets.verify(&ticket_query.ticket).await{
        Some(_user_id) => _user_id,
        None => return (StatusCode::UNAUTHORIZED, "Invalid ticket").into_response()
    };
    ws.on_upgrade(move |socket| websocket(socket, state, user_id))
}

// This function deals with a single websocket connection, i.e., a single
// connected client / user, for which we will spawn two independent tasks (for
// receiving / sending chat messages).
pub async fn websocket(stream: WebSocket, state: Arc<AppState>, user_id: UserID) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    // Loop until a text message is found.
    // while let Some(Ok(message)) = receiver.next().await {
    //     if let Message::Text(text) = message {
    //         // If username that is sent by client is not taken, fill username string.
    //         // check_username(&state, &mut username, name.as_str());

    //         // If not empty we want to quit the loop else we want to quit function.
    //         if !text.is_empty() {
    //             break;
    //         } else {
    //             // Only send our client that username is taken.
    //             let _ = sender
    //                 .send(Message::Text(Utf8Bytes::from_static(
    //                     "Username already taken.",
    //                 )))
    //                 .await;

    //             return;
    //         }
    //     }
    // }



    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            let parsed: Result<WsRequest, _> = serde_json::from_str(&text);

            match parsed {
                Ok(request) => {
                    if let Some(handler) = state.handlers.get(&request.method) {
                        let response = handler(state.clone(), request.data).await;
                        let response_str = serde_json::to_string(&response).unwrap_or_default();
                        let _ = sender.send(Message::Binary(response_str.as_bytes())).await;
                    } else {
                        let _ = sender.send(Message::Binary("Unknown method".as_bytes())).await;
                    }
                }
                Err(_) => {
                    let _ = sender.send(Message::Text("Invalid JSON".to_string())).await;
                }
            }
        }
    }



    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut rx = state.tx.subscribe();

    // Now send the "joined" message to all subscribers.
    let msg = format!("{username} joined.");
    tracing::debug!("{msg}");
    let _ = state.tx.send(msg);

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Clone things we want to pass (move) to the receiving task.
    let tx = state.tx.clone();
    let name = username.clone();

    // Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Add username before message.
            let _ = tx.send(format!("{name}: {text}"));
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };

    // Send "user left" message (similar to "joined" above).
    let msg = format!("{username} left.");
    tracing::debug!("{msg}");
    let _ = state.tx.send(msg);

    // Remove username from map so new clients can take it again.
    // state.clients.write().await.remove(&username);
}