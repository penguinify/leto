const path = require('path');

module.exports = {
    // gets all the files in the ./scripts
    entry: {"./scripts/pre": "./scripts/preInject/index.js", "./scripts/post": "./scripts/postInject/index.js"},
    mode: 'production',
    output: {
        filename: '[name].js',
        path: path.resolve(__dirname),
    },
};
