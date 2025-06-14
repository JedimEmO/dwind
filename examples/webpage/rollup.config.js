import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";
import {terser} from "rollup-plugin-terser";
import copy from 'rollup-plugin-copy'

const is_watch = !!process.env.ROLLUP_WATCH;

export default {
    input: {
        example: "Cargo.toml",
    },
    output: {
        dir: "dist/js",
        format: "es",
        sourcemap: true,
    },
    plugins: [
        rust({
            optimize: {wasmOpt: false}
        }),

        copy({
            targets: [
                {src: 'index.html', dest: 'dist/'}
            ]
        }),
        is_watch && serve({
            contentBase: "dist",
            open: true,
        }),

        is_watch && livereload("dist"),

        !is_watch && terser(),
    ],
};