
function logMessage(message) {
    const timestamp = new Date().toISOString();
    console.log(`[${timestamp}] [leto]`, message);
}

function logError(error) {
    const timestamp = new Date().toISOString();
    console.error(`[${timestamp}] [leto] Error: ${error}`);
}

export { logMessage, logError };
