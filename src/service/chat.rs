use std::{collections::VecDeque, time::Duration};

use futures::{select, FutureExt};
use futures_timer::Delay;
use leptos::prelude::{RwSignal, Update, UpdateUntracked};
use matchbox_socket::{PeerId, PeerState, WebRtcSocket};
use tower::{
    common::{chat::UserMessage, user::UserInfoResp},
    web::common::date,
};
const CHANNEL_ID: usize = 0;

pub async fn async_init_socket(
    user: UserInfoResp,
    room_url: String,
    sender: RwSignal<VecDeque<UserMessage>>,
    receiver: RwSignal<VecDeque<UserMessage>>,
) {
    let (mut socket, loop_fut) = WebRtcSocket::new_reliable(room_url);

    let loop_fut = loop_fut.fuse();
    futures::pin_mut!(loop_fut);

    let timeout = Delay::new(Duration::from_millis(100));
    futures::pin_mut!(timeout);

    loop {
        // Handle any new peers
        for (peer, state) in socket.update_peers() {
            let user = user.clone();
            match state {
                PeerState::Connected => {
                    let connected = UserMessage::joined(user, date::now());
                    socket
                        .channel_mut(CHANNEL_ID)
                        .send(connected.to_packet(), peer);
                }
                PeerState::Disconnected => {
                    let leave = UserMessage::leave(user, date::now());
                    socket.channel_mut(CHANNEL_ID).send(leave.to_packet(), peer);
                }
            }
        }

        // Accept any messages incoming
        for (_, packet) in socket.channel_mut(CHANNEL_ID).receive() {
            let message = UserMessage::from_packet(packet);
            receiver.update(|f| f.push_front(message));
        }
        sender.update_untracked(|f| {
            while let Some(message) = f.pop_back() {
                let peers: Vec<PeerId> = socket.connected_peers().collect();
                for peer in peers {
                    socket
                        .channel_mut(CHANNEL_ID)
                        .send(message.to_packet(), peer);
                }
            }
        });

        select! {
            // Restart this loop every 100ms
            _ = (&mut timeout).fuse() => {
                timeout.reset(Duration::from_millis(100));
            }

            // Or break if the message loop ends (disconnected, closed, etc.)
            _ = &mut loop_fut => {
                break;
            }
        }
    }
}
