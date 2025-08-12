// zoom hotkeys

import { newIpcMessage } from "./ipc";

const zoomCookie = document.cookie
    .split('; ')
    .find(row => row.startsWith('zoomLevel='));
let zoomLevel = zoomCookie ? parseFloat(zoomCookie.split('=')[1]) : 1.0;

if (zoomLevel !== 1.0) {
    newIpcMessage("zoom", { level: zoomLevel });
}

console.log("Zoom level initialized to:", zoomLevel);



document.body.addEventListener("keydown", (event) => {
    if (event.ctrlKey && event.key === "+") {
        zoomLevel = Math.min(3.0, zoomLevel + 0.1); // Prevent zooming in too far
    } else if (event.ctrlKey && event.key === "-") {
        zoomLevel = Math.max(0.1, zoomLevel - 0.1); // Prevent zooming out too far
    } else if (event.ctrlKey && event.key === "0") {
        zoomLevel = 1.0; // Reset to default zoom level
    } else {
        return; // Exit if not a zoom key combination
    }

    newIpcMessage("zoom", { level: zoomLevel });
    document.cookie = `zoomLevel=${zoomLevel}; path=/; max-age=31536000`;
    event.preventDefault(); // Prevent default browser zoom behavior


}); 
