'use strict';

require('./styles.sass');

const ui = require('autofactory-ui');
const m = require('mithril');
m.mount(document.body, ui.test.component);
