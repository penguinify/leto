
// Very much copied from Dorion, but I had to tweak it for it to be used with wry.

import { newIpcMessage } from "../shared/ipc";
import { logMessage } from "../shared/logger";
window.__LETO__.nativeFetch = window.fetch;
window.__LETO__.callbacks = {};

logMessage("Injection Proxy Extension");

window.__LETO__.fetch = async (url, options = {}) => {
    // Straight from dorion: injection/shared/recreate.ts
    const discordReg = /https?:\/\/(?:[a-z]+\.)?(?:discord\.com|discordapp\.com)(?:\/.*)?/g
    const scienceReg = /\/api\/v.*\/(science|track)/g

    // If it matches, just let it go through native OR its a relative URL
    if (url.toString().match(discordReg) || url.toString().startsWith('ipc://') || url.toString().startsWith('/')) {
        // Block science though!
        if (url.toString().match(scienceReg)) {
            console.log(`[Fetch Proxy] Blocked URL: ${url}`)
            return
        }

        return window.nativeFetch(url, options);
    }

    // Format body if there is any
    if (options && options.body) {
        const bodyObject = typeof options.body === 'string' ? JSON.parse(options.body) : options.body;
        options.body = JSON.stringify({
            ...bodyObject,
            __leto_injected: true
        });
    }

    // convert headers
    if (options && options.headers) {
        const headersObject = {};
        // typescript would fuck me for this, luckily I used js
        for (const [key, value] of Object.entries(options.headers)) {
            headersObject[key] = value;
        }

        options.headers = headersObject
    }

    options = {
        method: options.method || "GET",
        headers: options.headers || {},
        body: options.body || null
    
    }

    return new Promise((resolve, reject) => {
        const requestId = Math.random().toString(36).substr(2, 9);
        window.__LETO__.callbacks = window.__LETO__.callbacks || {};
        window.__LETO__.callbacks[requestId] = { resolve, reject };

        // Send message to Rust
        newIpcMessage(
            'fetch',
            {
                url: url,
                options: options,
                req_id: requestId
            }
        )
    });
}

window.__LETO__.handleFetchResponse = (id, success, data) => {
    const cb = window.__LETO__.callbacks[id];
    if (!cb) return;

    if (success) {
        // Simulate a fetch Response object
        const res = new Response(data.body, {
            status: data.status,
            headers: data.headers
        });
        cb.resolve(res);
    } else {
        cb.reject(data);
    }

    delete window.__LETO__.callbacks[id];
}




Object.defineProperty(window, 'fetch', {
    value: window.__LETO__.fetch,           // your custom fetch or original
    writable: false,          // prevents reassignment
    configurable: false       // prevents redefining the property
});
