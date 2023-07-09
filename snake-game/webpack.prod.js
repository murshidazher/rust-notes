const { merge } = require("webpack-merge");
const common = require("./webpack.common");
const CopyWebpackPlugin = require("copy-webpack-plugin");

module.exports = merge(common, {
  mode: "production",
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: "./www/index.html", to: "./" },
        { from: "./www/assets", to: "./assets" },
      ],
    }),
  ],
});
