const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");
const target = path.resolve(__dirname, "target/wasm-pack");

module.exports = {
  experiments: {
    asyncWebAssembly: true,
  },
  performance: {
    hints: false,
  },
  entry: {
    index: "./web/index.js",
  },
  output: {
    path: dist,
    filename: "[name].js",
  },
  devServer: {
    static: dist,
    allowedHosts: 'all',
    client: {
      overlay:false,
    },
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        { from: "web/index.html", to: dist },
        { from: "roms", to: path.join(dist, "roms") },
      ],
    }),
    new WasmPackPlugin({
      crateDirectory: __dirname,
      outDir: target,
      outName: "sres",
    }),
  ],
};
