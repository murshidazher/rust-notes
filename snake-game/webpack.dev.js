const path = require("path");
const { merge } = require("webpack-merge");
const common = require("./webpack.common");

module.exports = merge(common, {
  watch: true,
  mode: "development",
  devtool: "cheap-source-map",
  watchOptions: {
    ignored: /node_modules/,
  },
  devServer: {
    static: {
      directory: path.join(__dirname, "public"),
    },
    historyApiFallback: true,
  },
});
