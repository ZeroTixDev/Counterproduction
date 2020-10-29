'use strict';

require('./global-styles.scss');
const m = require('mithril');
window.addEventListener('resize', m.redraw);
const ctx = require.context('./components', false, /\.js$/);
ctx.keys().forEach((a) => (exports[/\.\/([^.]*)\.js$/.exec(a)[1]] = ctx(a)));
