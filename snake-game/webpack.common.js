const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");

module.exports = {
  entry: "./www/bootstrap.ts",
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
  output: {
    path: path.resolve(__dirname, "public"),
    filename: "bootstrap.js",
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: "./www/index.html", to: "./" },
        { from: "./www/assets", to: "./assets" },
      ],
    }),
  ],
};
