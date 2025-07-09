wasm-pack build --target web
rm -rf ./www/pkg/
mkdir ./www/pkg/
cp -r ./pkg/ ./www/