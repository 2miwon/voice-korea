use bdk::prelude::*;
use models::{
    dto::{AttendeeInfo, MeetingData, MeetingInfo, ParticipantData},
    Discussion,
};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{js_sys::eval, window, CustomEvent};

use crate::routes::Route;

use super::chat_message::{Chat, ChatMessage};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    #[allow(dead_code)]
    id: ReadOnlySignal<i64>,
    #[allow(dead_code)]
    discussion_id: ReadOnlySignal<i64>,
    pub nav: Navigator,

    meeting_info: Signal<MeetingInfo>,
    attendee_info: Signal<AttendeeInfo>,

    participants: Resource<ParticipantData>,
    #[allow(dead_code)]
    chat_messages: Signal<Vec<Chat>>,
}

impl Controller {
    pub fn init(
        lang: Language,
        id: ReadOnlySignal<i64>,
        discussion_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let participants = use_server_future(move || async move {
            ParticipantData::get_client(&crate::config::get().api_url)
                .get(discussion_id())
                .await
                .unwrap_or_default()
        })?;

        let mut ctrl = Self {
            lang,
            id,
            discussion_id,
            nav: use_navigator(),

            meeting_info: use_signal(|| MeetingInfo::default()),
            attendee_info: use_signal(|| AttendeeInfo::default()),

            participants,
            chat_messages: use_signal(|| vec![]),
        };

        ctrl.listen_chat_messages();

        use_future(move || async move {
            let _ = ctrl.start_meeting(discussion_id()).await;
        });

        Ok(ctrl)
    }

    fn listen_chat_messages(&mut self) {
        let mut chat_messages = self.chat_messages;
        let participants = self.participants;
        let extract_user_id = Self::extract_user_id;
        let get_email_by_user_id = Self::get_email_by_user_id;

        let closure =
            Closure::<dyn FnMut(web_sys::Event)>::wrap(Box::new(move |event: web_sys::Event| {
                if let Ok(event) = event.dyn_into::<CustomEvent>() {
                    if let Some(detail) = event.detail().as_string() {
                        if let Ok(chat) = serde_json::from_str::<ChatMessage>(&detail) {
                            let user_id = extract_user_id(&chat.sender_external_user_id);
                            let email = get_email_by_user_id(participants().unwrap(), user_id);

                            let c = Chat {
                                text: chat.text,
                                user_id,
                                email,
                                timestamp_ms: chat.timestamp_ms,
                            };

                            chat_messages.with_mut(|chat| {
                                if chat.is_empty()
                                    || chat.last().unwrap().timestamp_ms != c.timestamp_ms
                                {
                                    tracing::debug!("chat data: {:?}", c);
                                    chat.push(c);
                                }
                            });
                        }
                    }
                }
            }));

        window()
            .unwrap()
            .document()
            .unwrap()
            .add_event_listener_with_callback("chat-received", closure.as_ref().unchecked_ref())
            .unwrap();

        closure.forget();
    }

    fn get_email_by_user_id(participants: ParticipantData, user_id: i64) -> String {
        let (p, u) = (&participants.participants, &participants.users);

        p.iter()
            .position(|d| d.user_id == user_id)
            .map(|index| u[index].email.clone())
            .unwrap_or_default()
    }

    fn extract_user_id(external_user_id: &str) -> i64 {
        if external_user_id.starts_with("u-") {
            let trimmed = external_user_id.trim_start_matches("u-");
            let unquoted = trimmed.trim_matches('"');

            unquoted.to_string().parse::<i64>().unwrap_or_default()
        } else {
            0
        }
    }

    pub fn handle_refresh(&mut self) {
        self.participants.restart();
    }

    // NOTE: this function is not testing because multiple user testing is restricted.
    pub fn handle_selecting_attendee(&mut self, attendee_id: String) {
        let _ = eval(&format!(r#"window._focusVideo("{attendee_id}");"#)).unwrap();
    }

    pub async fn start_meeting(&mut self, discussion_id: i64) {
        let project_id = self.id();
        let meeting = Discussion::get_client(&crate::config::get().api_url)
            .start_meeting(project_id, discussion_id)
            .await
            .unwrap_or_default();

        tracing::debug!("meeting: {:?}", meeting);

        let participant = Discussion::get_client(&crate::config::get().api_url)
            .participant_meeting(project_id, discussion_id)
            .await
            .unwrap_or_default();

        tracing::debug!("discussion participant: {:?}", participant);

        let meeting = match MeetingData::get_client(&crate::config::get().api_url)
            .find_one(project_id, discussion_id)
            .await
        {
            Ok(v) => {
                tracing::debug!("meeting data: {:?}", meeting);
                v
            }
            Err(e) => {
                tracing::debug!("get_meeting data error: {:?}", e);
                MeetingData::default()
            }
        };

        self.meeting_info.set(meeting.meeting.clone());
        self.attendee_info.set(meeting.attendee.clone());

        let meeting_info = meeting.meeting;
        let attendee_info = meeting.attendee;

        //FIXME: fix to js file
        let js = format!(
            r#"
                            setTimeout(async () => {{
                                const logger = new window.chime.ConsoleLogger("log", window.chime.LogLevel.INFO);
                                const deviceController = new window.chime.DefaultDeviceController(logger);
                                const config = new window.chime.MeetingSessionConfiguration({meeting}, {attendee});
                                const session = new window.chime.DefaultMeetingSession(config, logger, deviceController);

                                navigator.mediaDevices.getUserMedia({{ audio: true }})
                                    .then((stream) => {{
                                        console.log("getUserMedia Success", stream);
                                    }})
                                    .catch((err) => {{
                                        console.error("getUserMedia Failed", err.name, err.message);
                                        return;
                                    }});

                                const audioInputs = await session.audioVideo.listAudioInputDevices();
                                const videoInputs = await session.audioVideo.listVideoInputDevices();

                                if (audioInputs.length > 0) {{
                                    await session.audioVideo.startAudioInput(audioInputs[0].deviceId);
                                }}

                                if (videoInputs.length > 0) {{
                                    await session.audioVideo.startVideoInput(videoInputs[0].deviceId);
                                }}

                                let isVideoOn = true;
                                let isAudioMuted = false;
                                let isShared = false;

                                window._videoOn = true;
                                window._shared = false;
                                window._audioMuted = false;

                                window._toggleVideo = function () {{
                                    if (!window._videoOn) {{
                                        session.audioVideo.startLocalVideoTile();
                                        window._videoOn = true;
                                    }} else {{
                                        session.audioVideo.stopLocalVideoTile();
                                        window._videoOn = false;
                                    }}
                                }};

                                window._sendMessage = function (text) {{
                                    if (!window._chimeSession) return;
                                    try {{
                                        window._chimeSession.audioVideo.realtimeSendDataMessage("chat", text, 1000);
                                        console.log("success to send message with text: ", text);
                                    }} catch (err) {{
                                        console.error("Send data message error", err);
                                    }};
                                }}

                                window._toggleAudio = function () {{
                                    if (window._audioMuted) {{
                                        session.audioVideo.realtimeUnmuteLocalAudio();
                                        window._audioMuted = false;
                                    }} else {{
                                        session.audioVideo.realtimeMuteLocalAudio();
                                        window._audioMuted = true;
                                    }}
                                }};

                                window._toggleShared = async function () {{
                                    if (window._shared) {{
                                        await session.audioVideo.stopContentShare();
                                        window._shared = false;
                                    }} else {{
                                        await session.audioVideo.startContentShareFromScreenCapture();
                                        window._shared = true;
                                    }}
                                }};

                                window._focusVideo = function(attendeeId) {{
                                    if (!window._videoTileMap) return;
                                    const tileId = window._videoTileMap[attendeeId];
                                    if (!tileId) return;

                                    const container = document.getElementById("video-grid");
                                    let videoElement = document.getElementById("video-grid-video");
                                    if (!videoElement) {{
                                        videoElement = document.createElement("video");
                                        videoElement.id = "video-grid-video";
                                        videoElement.autoplay = true;
                                        videoElement.playsInline = true;
                                        videoElement.className = "w-full h-full object-cover";
                                        container.innerHTML = "";
                                        container.appendChild(videoElement);
                                    }}

                                    session.audioVideo.bindVideoElement(tileId, videoElement);
                                }};

                                session.audioVideo.addObserver({{
                                    videoTileDidUpdate: (tileState) => {{
                                        if (!tileState.tileId || tileState.isContent) return;

                                        if (!window._videoTileMap) window._videoTileMap = {{}};
                                        window._videoTileMap[tileState.attendeeId] = tileState.tileId;

                                        const container = document.getElementById("video-grid");
                                        let videoElement = document.getElementById("video-grid-video");
                                        if (!videoElement) {{
                                            videoElement = document.createElement("video");
                                            videoElement.id = "video-grid-video";
                                            videoElement.autoplay = true;
                                            videoElement.playsInline = true;
                                            videoElement.muted = tileState.localTile;
                                            videoElement.className = "w-full h-full object-cover"; 
                                            container.innerHTML = ""; 
                                            container.appendChild(videoElement);
                                        }}

                                        session.audioVideo.bindVideoElement(tileState.tileId, videoElement);
                                    }},

                                    videoTileWasRemoved: (tileId) => {{
                                        const elem = document.getElementById("video-tile-" + tileId);
                                        if (elem) elem.remove();
                                    }}
                                }});

                                session.audioVideo.realtimeSubscribeToReceiveDataMessage("chat", (dataMessage) => {{
                                    console.log("message received: ", dataMessage);
                                    if (!dataMessage) return;

                                    const chat = {{
                                        topic: dataMessage.topic,
                                        sender_attendee_id: dataMessage.senderAttendeeId,
                                        sender_external_user_id: dataMessage.senderExternalUserId,
                                        text: new TextDecoder().decode(dataMessage.data),
                                        timestamp_ms: Math.floor(dataMessage.timestampMs),
                                    }};

                                    document.dispatchEvent(new CustomEvent("chat-received", {{
                                        detail: JSON.stringify(chat)
                                    }}));
                                }});

                                window._chimeSession = session;
                                session.audioVideo.start();
                                console.log("success to start chime session");
                                session.audioVideo.startLocalVideoTile();
                                
                            }}, 500);
                        "#,
            meeting = serde_json::to_string(&meeting_info).unwrap(),
            attendee = serde_json::to_string(&attendee_info).unwrap(),
        );
        self.participants.restart();
        let _ = eval(&js);
    }

    pub fn send_message(&self, text: String) {
        let escaped = text.replace('"', "\\\"");
        let js = format!(
            r#"
            if (window._sendMessage) {{
                window._sendMessage("{escaped}");

                const chat = {{
                    topic: "chat",
                    sender_attendee_id: window._chimeSession.configuration.credentials.attendeeId,
                    sender_external_user_id: window._chimeSession.configuration.credentials.externalUserId,
                    text: "{escaped}",
                    timestamp_ms: Date.now(),
                }};
                document.dispatchEvent(new CustomEvent("chat-received", {{
                    detail: JSON.stringify(chat)
                }}));
            }}
            "#
        );
        let _ = eval(&js);
    }

    pub async fn back(&self) {
        let _ = eval(
            r#"
        if (window._chimeSession) {
            try {
                window._chimeSession.audioVideo.stop();
                window._chimeSession = null;
                const container = document.getElementById("video-grid");
                if (container) container.innerHTML = "";
            } catch (e) {
                console.error("Failed to clean up Chime session", e);
            }
        }
    "#,
        );

        let _ = match Discussion::get_client(&crate::config::get().api_url)
            .exit_meeting(self.id(), self.discussion_id())
            .await
        {
            Ok(_) => {
                self.nav.replace(Route::ProjectPage {
                    lang: self.lang,
                    project_id: self.id(),
                });
            }
            Err(e) => {
                btracing::error!("failed to exit room with error: {:?}", e);
                self.nav.replace(Route::ProjectPage {
                    lang: self.lang,
                    project_id: self.id(),
                });
            }
        };
    }
}
