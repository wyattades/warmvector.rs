const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
// const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const WasmPackPlugin = require("./WasmPackPlugin");

const ROOT = path.resolve(__dirname);

const IS_DEV = process.env.NODE_ENV === "development";

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

    ...(IS_DEV
      ? [
          new WasmPackPlugin({
            crateDirectory: ROOT,
          }),
        ]
      : []),
  ],
};
