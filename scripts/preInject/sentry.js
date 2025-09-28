// This script replaces the Sentry script with a custom script named twinobi.js

import { logMessage } from "../shared/logger";

document.addEventListener("DOMContentLoaded", () => {
let script = document.querySelector("body > script:nth-child(4)");
    logMessage(script)
if (script) {
    script.remove();
}
});

