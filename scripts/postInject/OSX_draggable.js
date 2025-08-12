// Makes the title bar of a window draggable on macOS

import { newIpcMessage } from "../shared/ipc";


document.body.addEventListener("mousedown", (event) => {
    // magic number because I don't care
    if (event.y < 40 && event.button === 0) { // Check if the click is in the top 30 pixels and left mouse button
        newIpcMessage("drag_window", {});
    }
});
