# swc-plugin-core-fe-hmr

This swc plugin add `module.hot.decline()` in the program which has named import `Module` from core-fe

## Setup

```sh
npm install --save-dev swc-plugin-core-fe-hmr
```

update your `.swcrc` file like below:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [["swc-plugin-core-fe-hmr", {}]]
    }
  }
}
```
