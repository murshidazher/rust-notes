const path = require("path");

module.exports = {
  entry: "./www/bootstrap.ts",
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
};
