// makes links open in the default browser
import { newIpcMessage } from "./ipc";

document.addEventListener("click", (event) => {
    const target = event.target;
    console.log(event);
    if (target.tagName === "A" && target.href) {
        event.preventDefault();
        newIpcMessage("click_link", { url: target.href });
    } else if (target.tagName === "SPAN" && target.parentElement.tagName === "A" && target.parentElement.href) {
        event.preventDefault();
        newIpcMessage("click_link", { url: target.parentElement.href });
    }
});
