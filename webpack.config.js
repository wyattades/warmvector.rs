const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const ROOT = path.resolve(__dirname);

/** @type {import('webpack').Configuration} */
module.exports = {
  mode: process.env.NODE_ENV === "development" ? "development" : "production",
  experiments: {
    asyncWebAssembly: true,
  },
  entry: {
    index: "./js/index.js",
  },
  output: {
    path: path.join(ROOT, "www"),
    filename: "[name].js",
  },
  devServer: {
    open: true,
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        path.join(ROOT, "public"),
        {
          from: path.join(ROOT, "assets"),
          to: "assets",
        },
      ],
    }),

    new WasmPackPlugin({
      crateDirectory: ROOT,
    }),
  ],
};
