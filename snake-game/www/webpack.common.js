const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "public"),
    filename: "bootstrap.js"
  },
  plugins: [
    new CleanWebpackPlugin({
      cleanOnceBeforeBuildPatterns: [
        'public/*.wasm',
      ],
    }),
    new CopyWebpackPlugin({
      patterns: [
        { from: "./index.html", to: "./"}
      ]
    }),

  ]
}
