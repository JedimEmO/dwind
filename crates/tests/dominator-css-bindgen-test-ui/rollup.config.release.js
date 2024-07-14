import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve-proxy";
import livereload from "rollup-plugin-livereload";
import { terser } from "rollup-plugin-terser";
const is_watch = !!process.env.ROLLUP_WATCH;
import fg from 'fast-glob';

export default {
    input: {
        index: "./Cargo.toml",
    },
    output: {
        dir: "public/js",
        format: "iife",
        sourcemap: true,
    },
    plugins: [
        rust({
            serverPath: "js/",
            debug: false
        }),
        is_watch && serve({
            contentBase: "public",
            open: true,
            host: "0.0.0.0",
            proxy: {
                debug_dump_explorer_api: "http://127.0.0.1:9870"
            },
            allowCrossOrigin: true,
            headers: {
                "Access-Control-Allow-Origin": "*",
            }
        }),
        is_watch && livereload("public"),
        {
            name: 'watch-external',
            async buildStart(){
                const files = await fg('../../crates/dominator-css-bindgen/src/**/*');

                for(let file of files){
                    this.addWatchFile(file);
                }

                const files2 = await fg('../dominator-css-bindgen-test/resources/**/*');

                for(let file of files2){
                    this.addWatchFile(file);
                }

                const files3 = await fg('../../dwind/resources/**/*');

                for(let file of files3){
                    this.addWatchFile(file);
                }
            }
        },
        !is_watch && terser(),
    ],
};
