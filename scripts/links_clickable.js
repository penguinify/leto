// makes links open in the default browser
import { newIpcMessage } from "./ipc";

document.addEventListener("click", (event) => {
    // unreadable mess but you get the point
    const getLink = (el) =>
        el && el.tagName === "A" && el.href && el.target === "_blank"
            ? el
            : el && el.tagName === "SPAN" && el.parentElement
                ? getLink(el.parentElement)
                : null;

    const link = getLink(event.target);

    if (link) {
        event.preventDefault();
        newIpcMessage("click_link", { url: link.href });
    }
});
