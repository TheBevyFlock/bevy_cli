// Automatically reload when the server restarts
// Adopted from <https://github.com/trunk-rs/trunk/pull/231>
(function () {
  const url = 'ws://' + window.location.host + '/_bevy_dev/websocket';
  const pollIntervalMs = 5_000;
  let webSocket;

  const reload_upon_connect = () => {
    console.warn("Lost connection to the dev websocket");

    window.setTimeout(
      () => {
        // Reload when reconnecting to the websocket, e.g. because the server has been restarted
        webSocket = new WebSocket(url);
        webSocket.onopen = () => {
          console.info("Reconnected to dev websocket");

          window.location.reload()
        };
        webSocket.onclose = reload_upon_connect;

      },
      pollIntervalMs);
  };

  webSocket = new WebSocket(url);
  webSocket.onmessage = (ev) => {
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
  };

  webSocket.onopen = () => console.info("Connected to dev websocket");
  webSocket.onclose = reload_upon_connect;
  webSocket.readyState
})()
