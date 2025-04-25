use bdk::prelude::*;
use models::{
    dto::{AttendeeInfo, MeetingData, MeetingInfo, ParticipantData},
    Discussion,
};
use web_sys::js_sys::eval;

use crate::routes::Route;

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
}

impl Controller {
    pub fn init(
        lang: Language,
        id: ReadOnlySignal<i64>,
        discussion_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let participants = use_server_future(move || async move {
            ParticipantData::get_client(&crate::config::get().api_url)
                .find(discussion_id())
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
        };

        use_future(move || async move {
            let _ = ctrl.start_meeting(discussion_id()).await;
        });

        Ok(ctrl)
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
                                        console.log("✅ getUserMedia 성공", stream);
                                    }})
                                    .catch((err) => {{
                                        console.error("❌ getUserMedia 실패", err.name, err.message);
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

                                session.audioVideo.addObserver({{
                                    videoTileDidUpdate: (tileState) => {{
                                        if (!tileState.tileId || tileState.isContent) return;

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

                                session.audioVideo.start();
                                session.audioVideo.startLocalVideoTile();
                                window._chimeSession = session;
                            }}, 500);
                        "#,
            meeting = serde_json::to_string(&meeting_info).unwrap(),
            attendee = serde_json::to_string(&attendee_info).unwrap(),
        );
        self.participants.restart();
        let _ = eval(&js);
    }

    pub async fn back(&self) {
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
            }
        };
    }
}
