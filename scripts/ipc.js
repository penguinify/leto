export function newIpcMessage(id, data) {
    window.ipc.postMessage(JSON.stringify({
        id: id,
        ...data
    }));
}
