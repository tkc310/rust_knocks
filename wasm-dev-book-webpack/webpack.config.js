const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  module: {
    rules: [
      {
        test: /\.ts$/,
        loader: 'ts-loader',
        options: {
          transpileOnly: true
        },
      },
    ],
  },
  resolve: {
    extensions: [".ts", ".tsx", ".js", ".jsx", ".wasm"]
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.join(__dirname, "src/index.html")
    }),
    new WasmPackPlugin({
      crateDirectory: path.join(__dirname, "crate")
    })
  ],
  stats: {
    children: true,
  },
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true,
  }
};
