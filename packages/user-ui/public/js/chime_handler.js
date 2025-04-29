let chimeSession = null;
let videoTileMap = {};
let isVideoOn = true;
let isAudioMuted = false;
let isScreenSharing = false;

async function startChimeSession(meetingInfo, attendeeInfo) {
  console.log("startChimeSession called");

  const logger = new window.chime.ConsoleLogger(
    "log",
    window.chime.LogLevel.INFO
  );
  const deviceController = new window.chime.DefaultDeviceController(logger);
  const config = new window.chime.MeetingSessionConfiguration(
    meetingInfo,
    attendeeInfo
  );
  const session = new window.chime.DefaultMeetingSession(
    config,
    logger,
    deviceController
  );

  chimeSession = session;

  try {
    await navigator.mediaDevices.getUserMedia({ audio: true, video: true });

    const videoInputs = await session.audioVideo.listVideoInputDevices();
    console.log("Available video inputs:", videoInputs);

    if (videoInputs.length > 0) {
      await session.audioVideo.startVideoInput(videoInputs[0].deviceId);
      console.log("Started video input:", videoInputs[0].label);
    } else {
      console.warn("No video input devices found.");
    }

    await session.audioVideo.start();
    session.audioVideo.startLocalVideoTile();
    console.log("Chime session started successfully.");
  } catch (e) {
    console.error("Failed to start Chime session:", e);
  }

  session.audioVideo.addObserver({
    videoTileDidUpdate: (tileState) => {
      console.log("videoTileDidUpdate called:", tileState);
      if (!tileState.tileId || tileState.isContent) return;

      videoTileMap[tileState.attendeeId] = tileState.tileId;

      const container = document.getElementById("video-grid");
      if (!container) return;

      let videoElement = document.getElementById("video-grid-video");
      if (!videoElement) {
        videoElement = document.createElement("video");
        videoElement.id = "video-grid-video";
        videoElement.autoplay = true;
        videoElement.playsInline = true;
        videoElement.muted = tileState.localTile ?? false;
        videoElement.className = "w-full h-full object-cover";
        container.innerHTML = "";
        container.appendChild(videoElement);
      }

      session.audioVideo.bindVideoElement(tileState.tileId, videoElement);
    },

    videoTileWasRemoved: (tileId) => {
      console.log("videoTileWasRemoved called:", tileId);
      const elem = document.getElementById("video-tile-" + tileId);
      if (elem) elem.remove();
    },
  });

  session.audioVideo.realtimeSubscribeToReceiveDataMessage(
    "chat",
    (dataMessage) => {
      console.log("Received dataMessage (chat):", dataMessage);

      const detail = {
        topic: dataMessage.topic,
        sender_attendee_id: dataMessage.senderAttendeeId,
        sender_external_user_id: dataMessage.senderExternalUserId,
        text: new TextDecoder().decode(dataMessage.data),
        timestamp_ms: Math.floor(dataMessage.timestampMs),
      };

      console.log("Dispatching chat-received event:", detail);

      document.dispatchEvent(
        new CustomEvent("chat-received", { detail: JSON.stringify(detail) })
      );
    }
  );

  session.audioVideo.realtimeSubscribeToReceiveDataMessage(
    "attendee-status",
    (dataMessage) => {
      const detailStr = new TextDecoder().decode(dataMessage.data);
      console.log("Dispatching attendee-status event:", detailStr);

      window.dispatchEvent(
        new CustomEvent("attendee-status", { detail: detailStr })
      );
    }
  );

  session.audioVideo.realtimeSubscribeToAttendeeIdPresence(
    (attendeeId, present, externalUserId, dropped) => {
      console.log("Presence changed:", attendeeId, present);
      const eventName = "participant-refresh";
      if (present) {
        window.dispatchEvent(new Event(eventName));
      } else {
        setTimeout(() => {
          window.dispatchEvent(new Event(eventName));
        }, 2000);
      }
    }
  );
}

function sendChimeMessage(text) {
  if (!chimeSession) {
    console.error("No active Chime session");
    return;
  }

  try {
    chimeSession.audioVideo.realtimeSendDataMessage("chat", text, 1000);
    console.log("Sent chat message:", text);

    const attendeeId = chimeSession.configuration.credentials.attendeeId;
    const externalUserId =
      chimeSession.configuration.credentials.externalUserId;
    const detail = {
      topic: "chat",
      sender_attendee_id: attendeeId,
      sender_external_user_id: externalUserId,
      text: text,
      timestamp_ms: Date.now(),
    };

    document.dispatchEvent(
      new CustomEvent("chat-received", { detail: JSON.stringify(detail) })
    );
    console.log("Dispatched chat-received event:", detail);
  } catch (err) {
    console.error("Failed to send chat message:", err);
  }
}

function focusVideo(attendeeId) {
  if (!chimeSession || !videoTileMap) return;

  const tileId = videoTileMap[attendeeId];
  if (!tileId) return;

  const container = document.getElementById("video-grid");
  if (!container) return;

  let videoElement = document.getElementById("video-grid-video");
  if (!videoElement) {
    videoElement = document.createElement("video");
    videoElement.id = "video-grid-video";
    videoElement.autoplay = true;
    videoElement.playsInline = true;
    videoElement.className = "w-full h-full object-cover";
    container.innerHTML = "";
    container.appendChild(videoElement);
  }

  chimeSession.audioVideo.bindVideoElement(tileId, videoElement);
}

function toggleVideo() {
  if (!chimeSession) {
    console.error("No active Chime session");
    return;
  }

  if (isVideoOn) {
    chimeSession.audioVideo.stopLocalVideoTile();
    isVideoOn = false;
    console.log("Video stopped.");
  } else {
    chimeSession.audioVideo.startLocalVideoTile();
    isVideoOn = true;
    console.log("Video started.");
  }

  sendAttendeeStatus();
}

function toggleAudio() {
  if (!chimeSession) {
    console.error("No active Chime session");
    return;
  }

  if (isAudioMuted) {
    chimeSession.audioVideo.realtimeUnmuteLocalAudio();
    isAudioMuted = false;
    console.log("Audio unmuted.");
  } else {
    chimeSession.audioVideo.realtimeMuteLocalAudio();
    isAudioMuted = true;
    console.log("Audio muted.");
  }

  sendAttendeeStatus();
}

function sendAttendeeStatus() {
  if (!chimeSession) return;

  const attendeeId = chimeSession.configuration.credentials.attendeeId;
  const externalUserId = chimeSession.configuration.credentials.externalUserId;

  const status = {
    attendee_id: attendeeId,
    external_user_id: externalUserId,
    video_on: isVideoOn,
    audio_muted: isAudioMuted,
  };

  try {
    chimeSession.audioVideo.realtimeSendDataMessage(
      "attendee-status",
      JSON.stringify(status),
      1000
    );
    console.log("Sent attendee-status:", status);

    window.dispatchEvent(
      new CustomEvent("attendee-status", { detail: JSON.stringify(status) })
    );
  } catch (err) {
    console.error("Failed to send attendee-status:", err);
  }
}

async function toggleScreenShare() {
  if (!chimeSession) {
    console.error("No active Chime session");
    return;
  }

  try {
    if (isScreenSharing) {
      await chimeSession.audioVideo.stopContentShare();
      isScreenSharing = false;
      console.log("Screen sharing stopped.");
    } else {
      await chimeSession.audioVideo.startContentShareFromScreenCapture();
      isScreenSharing = true;
      console.log("Screen sharing started.");
    }
  } catch (err) {
    console.error("Failed to toggle screen sharing:", err);
  }
}

function cleanupChimeSession() {
  if (!chimeSession) return;
  try {
    chimeSession.audioVideo.stop();
    chimeSession = null;

    const container = document.getElementById("video-grid");
    if (container) {
      container.innerHTML = "";
    }
    videoTileMap = {};
    console.log("Chime session cleaned up.");
  } catch (e) {
    console.error("Failed to clean up Chime session:", e);
  }
}

window.startChimeSession = startChimeSession;
window.sendChimeMessage = sendChimeMessage;
window.focusVideo = focusVideo;
window.toggleVideo = toggleVideo;
window.toggleAudio = toggleAudio;
window.toggleScreenShare = toggleScreenShare;
window.cleanupChimeSession = cleanupChimeSession;

document.addEventListener("chat-received", (e) => {
  console.log("chat-received event fired:", e.detail);
});
