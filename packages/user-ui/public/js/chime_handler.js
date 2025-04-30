let chimeSession = null;
let videoTileMap = {};
let isVideoOn = true;
let isAudioMuted = false;
let isScreenSharing = false;
let attendeeStatusInterval = null;
let chimeObserver = null;

// function startSendingAttendeeStatus() {
//   stopSendingAttendeeStatus();

//   attendeeStatusInterval = setInterval(() => {
//     sendAttendeeStatus();
//   }, 3000);
// }

// function stopSendingAttendeeStatus() {
//   if (attendeeStatusInterval) {
//     clearInterval(attendeeStatusInterval);
//     attendeeStatusInterval = null;
//   }
// }

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

    console.log("Chime session started successfully.");
  } catch (e) {
    console.error("Failed to start Chime session:", e);
  }

  await session.audioVideo.start();
  session.audioVideo.startLocalVideoTile();

  chimeObserver = {
    videoTileDidUpdate: (tileState) => {
      if (!chimeSession || !chimeSession.audioVideo) return;
      console.log("tileStatus:", tileState);
      if (!tileState.tileId) return;

      const container = document.getElementById("video-grid");

      if (tileState.isContent) {
        console.log("Rendering content share tile");
        let contentVideo = document.getElementById("content-share-video");
        if (!contentVideo) {
          contentVideo = document.createElement("video");
          contentVideo.id = "content-share-video";
          contentVideo.autoplay = true;
          contentVideo.playsInline = true;
          contentVideo.className = "w-full h-full object-contain";
          container.appendChild(contentVideo);
        }

        const myVideo = document.getElementById("my-video");
        const focusedVideo = document.getElementById("focused-video");
        if (myVideo) myVideo.style.display = "none";
        if (focusedVideo) focusedVideo.style.display = "none";

        chimeSession.audioVideo.bindVideoElement(
          tileState.tileId,
          contentVideo
        );
        return;
      }

      videoTileMap[tileState.boundAttendeeId] = tileState.tileId;

      if (tileState.localTile) {
        let myVideo = document.getElementById("my-video");
        if (!myVideo) {
          myVideo = document.createElement("video");
          myVideo.id = "my-video";
          myVideo.autoplay = true;
          myVideo.playsInline = true;
          myVideo.muted = true;
          myVideo.className = "w-full h-full object-cover";
          container.appendChild(myVideo);
        }
        chimeSession.audioVideo.bindVideoElement(tileState.tileId, myVideo);
        console.log("Bound my local video");
      }
    },

    videoTileWasRemoved: (tileId) => {
      if (!chimeSession || !chimeSession.audioVideo) return;
      console.log("videoTileWasRemoved:", tileId);

      const contentVideo = document.getElementById("content-share-video");
      if (contentVideo) {
        contentVideo.remove();

        const myVideo = document.getElementById("my-video");
        const focusedVideo = document.getElementById("focused-video");
        if (myVideo) myVideo.style.display = "block";
        if (focusedVideo) focusedVideo.style.display = "block";
      }

      const elem = document.getElementById("video-tile-" + tileId);
      if (elem) elem.remove();
    },
  };

  session.audioVideo.addObserver(chimeObserver);

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
  if (!chimeSession) return;

  const myId = chimeSession.configuration.credentials.attendeeId;

  const videoGrid = document.getElementById("video-grid");
  const myVideo = document.getElementById("my-video");
  let focusedVideo = document.getElementById("focused-video");

  if (attendeeId === myId) {
    if (focusedVideo) {
      focusedVideo.remove();
    }
    if (myVideo) {
      myVideo.style.display = "block";
    }
    console.log("Returned to my local video");
    return;
  }

  if (!focusedVideo && videoGrid) {
    focusedVideo = document.createElement("video");
    focusedVideo.id = "focused-video";
    focusedVideo.autoplay = true;
    focusedVideo.playsInline = true;
    focusedVideo.className = "w-full h-full object-cover";
    videoGrid.appendChild(focusedVideo);
  }

  if (!focusedVideo) return;

  if (myVideo) {
    myVideo.style.display = "none";
  }

  const tileId = videoTileMap[attendeeId];

  console.log("tileId: ", tileId);
  if (tileId) {
    chimeSession.audioVideo.bindVideoElement(tileId, focusedVideo);
    focusedVideo.style.backgroundColor = "transparent";
    console.log(`Focused on attendee with video: ${attendeeId}`);
  } else {
    focusedVideo.srcObject = null;
    focusedVideo.style.backgroundColor = "black";
    console.warn(`Focused on attendee with no video: ${attendeeId}`);
  }
}

function toggleVideo() {
  if (!chimeSession) {
    console.error("No active Chime session");
    return;
  }

  const myId = chimeSession.configuration.credentials.attendeeId;

  if (isVideoOn) {
    chimeSession.audioVideo.stopLocalVideoTile();
    isVideoOn = false;
    console.log("Video stopped.");
  } else {
    chimeSession.audioVideo.startLocalVideoTile();
    isVideoOn = true;
    console.log("Video started.");
  }

  focusVideo(myId);
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
  if (!chimeSession || !chimeSession.audioVideo) {
    console.warn("chimeSession is null in videoTileDidUpdate");
    return;
  }

  try {
    console.log("Cleaning up Chime session...");

    chimeSession.audioVideo.stopLocalVideoTile();
    chimeSession.audioVideo.stopContentShare();
    chimeSession.audioVideo.stop();

    const videoGrid = document.getElementById("video-grid");
    if (videoGrid) {
      videoGrid.innerHTML = "";
    }

    videoTileMap = {};
    isVideoOn = true;
    isAudioMuted = false;
    isScreenSharing = false;

    if (chimeSession && chimeObserver) {
      chimeSession.audioVideo.removeObserver(chimeObserver);
      chimeObserver = null;
    }

    console.log("Chime session cleaned up completely.");
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
