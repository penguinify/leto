// zoom hotkeys

document.body.addEventListener("keydown", (event) => {
    if (event.ctrlKey && event.key === "+") {
        window.ipc.postMessage("zoom_in");
    } else if (event.ctrlKey && event.key === "-") {
        window.ipc.postMessage("zoom_out");
    }
});
