// makes links open in the default browser
import { newIpcMessage } from "./ipc";

document.body.addEventListener("click", (event) => {
    if (event.target.tagName === "A" && event.target.href) {
        event.preventDefault(); // Prevent the default link behavior
        newIpcMessage("click_link", {
            url: event.target.href
        });
    }
});
