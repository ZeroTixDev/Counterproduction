'use strict';

const m = require('mithril');
const HTML = require('hyperscript-adapter');
const HTMLModule = require('hyperscript-adapter-modules');
const $ = HTML({
    h(...args) {
        console.log([...args]);
        return m(...args);
    },
    textConvert: (a) => `${a}`,
    combineId: true,
    combineClasses: true,
});
const mod = HTMLModule($, {
    partialApply: true,
    useDefault: true,
});
module.exports = { mod, m, HTML: $ };
