// Automatically reload when the server restarts
// Adopted from <https://github.com/trunk-rs/trunk/pull/231>
(function () {
  const url = 'ws://' + window.location.host + '/_bevy_dev/websocket';
  const pollIntervalMs = 5_000;
  /** @type WebSocket | undefined */
  let webSocket;

  let isFirstLoad = true;
  let isConnected = false;

  function onOpen() {
    if (isFirstLoad) {
      console.info("Connected to dev websocket");
    } else {
      console.info("Reconnected to dev websocket");
      window.location.reload()
    }

    isFirstLoad = false;
    isConnected = true;
  }

  function onClose() {
    if (isConnected) {
      console.warn("Lost connection to the dev server");
    }

    isConnected = false;

    // Periodically try to reconnect to the server
    window.setTimeout(recreateWebsocket, pollIntervalMs);
  }

  function onMessage(ev) {
    try {
      const msg = JSON.parse(ev.data);

      switch (msg.type) {
        case "reload":
          window.location.reload();
        default:
          console.warn("Unknown websocket message", msg);
      }
    } catch (error) {
      console.warn("Failed to parse websocket message", error);
    }
  }

  function recreateWebsocket() {
    if (webSocket) {
      webSocket.removeEventListener("open", onOpen);
      webSocket.removeEventListener("close", onClose);
      webSocket.removeEventListener("message", onMessage);
    }

    webSocket = new WebSocket(url);
    webSocket.addEventListener("open", onOpen);
    webSocket.addEventListener("close", onClose);
    webSocket.addEventListener("message", onMessage);
  }

  // Initial connection
  recreateWebsocket();
})()
