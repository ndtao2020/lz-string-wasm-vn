# lz-string-wasm-vn
[![npm](https://img.shields.io/npm/v/lz-string-wasm-vn)](https://www.npmjs.com/package/lz-string-wasm-vn)

## Browser

```js
import init, { compressToUTF16, decompressFromUTF16 } from 'lz-string-wasm-vn/web';

await init();

const data = "This feature will replace some internal maps' hashers with rustc-hash, boosting performance at the cost of not using a DOS-resistant hasher."

const compressed = compressToUTF16(data);
const decompressed = decompressFromUTF16(compressed);

console.log("data: ", data);
console.log("compressed: ", compressed);
console.log("decompressed: ", decompressed);
```
