const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "public"),
    filename: "bootstrap.js",
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
  plugins: [
    new CleanWebpackPlugin({
      cleanOnceBeforeBuildPatterns: [
        "public/*.wasm",
        "public/bootstrap.js",
        "public/*.bootstrap.js",
      ],
    }),
    new CopyWebpackPlugin({
      patterns: [{ from: "./index.html", to: "./" }],
    }),
  ],
};
