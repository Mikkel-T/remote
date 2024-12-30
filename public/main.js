htmx.on("htmx:wsBeforeMessage", (e) => {
  if (e.detail.message.startsWith("{")) {
    const obj = JSON.parse(e.detail.message);

    if (obj.type === "volume") {
      document.getElementById("volume").innerText = obj.data;
      document.getElementById("vol").value = obj.data;
    }
  }
});
