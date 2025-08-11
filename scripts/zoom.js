// zoom hotkeys

import { newIpcMessage } from "./ipc";

document.body.addEventListener("keydown", (event) => {
    if (event.ctrlKey && event.key === "+") {
        newIpcMessage("zoom_in", {});
    } else if (event.ctrlKey && event.key === "-") {
        newIpcMessage("zoom_out", {});
    }
});
