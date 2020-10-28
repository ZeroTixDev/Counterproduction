'use strict';

const ctx = require.context('./components', false, /\.js$/);
ctx.keys().forEach((a) => (exports[/\.\/([^.]*)\.js$/.exec(a)[1]] = ctx(a)));
