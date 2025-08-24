const path = require('path');

module.exports = {
    // gets all the files in the ./scripts
    entry: { "./scripts/pre": "./scripts/preInject/index.js", "./scripts/post": "./scripts/postInject/index.js" },
    // svg loader
    module: {
        rules: [
            {
                test: /\.svg$/,
                use: 'svg-inline-loader',
            },
        ],
    },

    mode: 'production',
    output: {
        filename: '[name].js',
        path: path.resolve(__dirname),
    },
};
