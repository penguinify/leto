// Makes the title bar of a window draggable on macOS


document.body.addEventListener("mousedown", (event) => {
    if (event.y < 30 && event.button === 0) { // Check if the click is in the top 30 pixels and left mouse button
        window.ipc.postMessage("drag_window")
    }
});
