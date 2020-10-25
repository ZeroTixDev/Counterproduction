'use strict';

const ctx = require.context('./src', false, /\.js$/);
console.log(ctx.keys());
ctx.keys().forEach((a) => (exports[/\.\/([^.]*)\.js$/.exec(a)[1]] = ctx(a)));
