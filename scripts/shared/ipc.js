import { logMessage } from "./logger";

export function newIpcMessage(id, data) {
    logMessage(`Sending IPC Message: ${id} with data: ${JSON.stringify(data)}`);
    window.ipc.postMessage(JSON.stringify({
        id: id,
        ...data
    }));
}
