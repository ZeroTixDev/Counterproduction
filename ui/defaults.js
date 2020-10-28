'use strict';

module.exports = (_def, af) =>
    function withDefaults(obj) {
        const def = typeof def === 'function' ? def() : def;
        return new Proxy(obj, {
            get(_, prop) {
                const val = obj[prop];
                const defval = def[prop];
                if (val === undefined) {
                    return defval;
                } else if (typeof val === 'object') {
                    return withDefaults(val, defval);
                } else if (af && typeof defval === 'function' && typeof val !== 'function') {
                    return () => val;
                } else {
                    return val;
                }
            },
        });
    };
