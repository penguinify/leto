// Remove any existing CSP meta tags
const old = document.querySelectorAll('meta[http-equiv="Content-Security-Policy"]');
old.forEach(tag => tag.remove());

// Create a new CSP meta tag with relaxed rules
const meta = document.createElement("meta");
meta.httpEquiv = "Content-Security-Policy";
meta.content = `
connect-src * 'self' data: blob:;
`.replace(/\s+/g, ' ').trim(); // collapse whitespace
document.head.appendChild(meta)

