const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dev_dir = path.resolve(__dirname, "test");

module.exports = {
  mode: "development",
  entry: {
    index: `${dev_dir}/js/index.js`,
  },
  output: {
    path: `${dev_dir}/dist`,
    filename: "[name].js",
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: `${dev_dir}/index.html`,
      inject: false,
    }),
    new WasmPackPlugin({
      outDir: path.resolve(__dirname , "pkg"),
      crateDirectory: path.resolve(__dirname , "src")
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
  resolve: {
    extensions: [".ts", ".js"],
  },
  devServer: {
      open: true
  }
};

