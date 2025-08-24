// Remove any existing CSP meta tags
const old = document.querySelectorAll('meta[http-equiv="Content-Security-Policy"]');
old.forEach(tag => tag.remove());

// Create a new CSP meta tag with relaxed rules
const meta = document.createElement("meta");
meta.httpEquiv = "Content-Security-Policy";
meta.content = `
default-src 'self' 'unsafe-inline' 'unsafe-eval' data: blob:;
img-src * data: blob:;
media-src *;
frame-src https://www.youtube.com https://discord.com https://twitter.com;
connect-src *;
`.replace(/\s+/g, ' ').trim(); // collapse whitespace
document.head.appendChild(meta);

