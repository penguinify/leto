const path = require('path');

module.exports = {
    // gets all the files in the ./scripts
    entry: "./scripts/index.js",
    mode: 'production',
    output: {
        filename: 'internal.js',
        path: path.resolve(__dirname, 'scripts'),
    },
};
