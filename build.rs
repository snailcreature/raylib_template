fn main() {
    println!("cargo::rustc-check-cfg=cfg(target_os, values(\"emscripten\"))");
    {
        use std::{env, fs, path::Path};
        let crate_name = env!("CARGO_PKG_NAME");
        let crate_desc = env!("CARGO_PKG_DESCRIPTION");

        let out_dir = env::var("OUT_DIR").unwrap();
        let out_filename = format_args!("{}/../../../index.html", out_dir).to_string();
        let out_filepath = Path::new(&out_filename);
        let index_html = format_args!("
    <!doctype html>\n
    <html lang=\"en-GB\">\n
    <head>\n
        <meta charset=\"utf-8\">\n
        <meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\">\n
\n
        <title>{crate_name}</title>\n
\n
        <meta name=\"title\" content=\"{crate_name}\">\n
        <meta name=\"description\" content=\"{crate_desc}\">\n
        <meta name=\"viewport\" content=\"width=device-width\">\n
\n
        <meta property=\"og:title\" content=\"{crate_name}\">\n
        <meta property=\"og:description\" content=\"{crate_desc}\">\n
\n
        <meta property=\"twitter:card\" content=\"summary\">\n
        <meta property=\"twitter:title\" content=\"{crate_name}\">\n
        <meta property=\"twitter:descripion\" content=\"{crate_desc}\">\n
\n
        <link rel=\"shortcut icon\" href=\"https://www.raylib.com/favicon.ico\">
\n
        <style>\n
            body {{\n
                font-family: arial;\n
                margin: 0;\n
                padding: none;\n
            }}\n
\n
            header {{\n
                width: 100%;\n
                height: 80px;\n
                background-color: #888888;\n
            }}\n
\n
            /* note: raylib logo is embedded in the page as base64 png image */\n
            #logo {{\n
                width: 64px;\n
                height: 64px;\n
                float: left;\n
                position: relative;\n
                margin: 10px;\n
            }}\n
\n
            .emscripten {{\n
                padding-right: 0;\n
                margin-left: auto;\n
                margin-right: auto;\n
                display: block;\n
            }}\n
\n
            div.emscripten {{\n
                text-align: center;\n
            }}\n
\n
            div.emscripten_border {{\n
                border: 1px solid black;\n
            }}\n
\n
            /* note: canvas *must not* have any border or padding, or mouse coords will be wrong*/\n
            canvas.emscripten {{\n
                border: 0px none;\n
                background: black;\n
                width: 100%\n
            }}\n
\n
            .spinner {{\n
                height: 30px;\n
                width: 30px;\n
                margin: 0;\n
                margin-top: 20px;\n
                margin-left: 20px;\n
                display: inline-block;\n
                vertical-align: top;\n
                -webkit-animation: rotation .8s linear infinite;\n
                -moz-animation: rotation .8s linear infinite;\n
                -o-animation: rotation .8s linear infinite;\n
                animation: rotation 0.8s linear infinite;\n
                border-left: 5px solid black;\n
                border-right: 5px solid black;\n
                border-bottom: 5px solid black;\n
                border-top: 5px solid red;\n
                border-radius: 100%;\n
                background-color: rgb(245, 245, 245);\n
            }}\n
\n
            @-webkit-keyframes rotation {{\n
                from {{\n
                    -webkit-transform: rotate(0deg);\n
                }}\n
\n
                to {{\n
                    -webkit-transform: rotate(360deg);\n
                }}\n
            }}\n
\n
            @-moz-keyframes rotation {{\n
                from {{\n
                    -moz-transform: rotate(0deg);\n
                }}\n
\n
                to {{\n
                    -moz-transform: rotate(360deg);\n
                }}\n
            }}\n
\n
            @-o-keyframes rotation {{\n
                from {{\n
                    -o-transform: rotate(0deg);\n
                }}\n
\n
                to {{\n
                    -o-transform: rotate(360deg);\n
                }}\n
            }}\n
\n
            @keyframes rotation {{\n
                from {{\n
                    transform: rotate(0deg);\n
                }}\n
\n
                to {{\n
                    transform: rotate(360deg);\n
                }}\n
            }}\n
\n
            #status {{\n
                display: inline-block;\n
                vertical-align: top;\n
                margin-top: 30px;\n
                margin-left: 20px;\n
                font-weight: bold;\n
                color: rgb(40, 40, 40);\n
            }}\n
\n
            #progress {{\n
                height: 0px;\n
                width: 0px;\n
            }}\n
\n
            #controls {{\n
                display: inline-block;\n
                float: right;\n
                vertical-align: top;\n
                margin-top: 15px;\n
                margin-right: 20px;\n
            }}\n
\n
            #output {{\n
                width: 100%;\n
                height: 140px;\n
                margin: 0 auto;\n
                margin-top: 10px;\n
                display: block;\n
                background-color: black;\n
                color: rgb(37, 174, 38);\n
                font-family: 'lucida console', monaco, monospace;\n
                outline: none;\n
            }}\n
\n
            input[type=button] {{\n
                background-color: lightgray;\n
                border: 4px solid darkgray;\n
                color: black;\n
                text-decoration: none;\n
                cursor: pointer;\n
                width: 140px;\n
                height: 50px;\n
            }}\n
\n
            input[type=button]:hover {{\n
                background-color: #f5f5f5ff;\n
                border-color: black;\n
            }}\n
        </style>\n
    </head>\n
\n
    <body>\n
        <header id=\"header\">\n
            <a id=\"logo\" href=\"https://www.raylib.com\"></a>\n
\n
            <div class=\"spinner\" id='spinner'></div>\n
            <div class=\"emscripten\" id=\"status\">Downloading...</div>\n
\n
            <span id='controls'>\n
                <span><input type=\"button\" value=\"🖵 FULLSCREEN\" onclick=\"Module.requestFullscreen(false, false)\"></span>\n
                <span><input type=\"button\" id=\"btn-audio\" value=\"🔇 SUSPEND\" onclick=\"toggleAudio()\"></span>\n
            </span>\n
\n
            <div class=\"emscripten\">\n
                <progress value=\"0\" max=\"100\" id=\"progress\" hidden></progress>\n
            </div>\n
        </header>\n
\n
        <div class=\"emscripten_border\">\n
            <canvas class=\"emscripten\" id=\"canvas\" oncontextmenu=\"event.preventDefault()\" tabindex=-1></canvas>\n
        </div>\n
\n
        <textarea id=\"output\" rows=\"8\"></textarea>\n
\n
        <script type='text/javascript'\n
            src=\"https://cdn.jsdelivr.net/gh/eligrey/FileSaver.js/dist/FileSaver.min.js\">\n
        </script>\n
        <script type='text/javascript'>\n
            function saveFileFromMEMFSToDisk(memoryFSname, localFSname)     // This can be called by C/C++ code\n
            {{\n
                var isSafari = /^((?!chrome|android).)*safari/i.test(navigator.userAgent);\n
                var data = FS.readFile(memoryFSname);\n
                var blob;\n
\n
                if (isSafari) blob = new Blob([data.buffer], {{ type: \"application/octet-stream\" }});\n
                else blob = new Blob([data.buffer], {{ type: \"application/octet-binary\" }});n
\n
                /** NOTE: SaveAsDialog is a browser setting. For example, in Google Chrome,\n
                / * in Settings/Advanced/Downloads section you have a setting:\n
                / * 'Ask where to save each file before downloading' - which you can set\n
                / * true/false.\n
                / * If you enable this setting it would always ask you and bring the SaveAsDialog\n
                / *saveAs(blob, localFSname);\n
                 **/\n
            }}\n
        </script>\n
        <script type='text/javascript'>\n
            var statusElement = document.querySelector('#status');\n
            var progressElement = document.querySelector('#progress');\n
            var spinnerElement = document.querySelector('#spinner');\n
            var Module = {{\n
                preRun: [],\n
                postRun: [],\n
                print: (function () {{\n
                    var element = document.querySelector('#output');\n
\n
                    if (element) element.value = '';    // Clear browser cache\n
\n
                    return function (text) {{\n
                        if (arguments.length > 1) text = Array.prototype.slice.call(arguments).join(' ');\n
                        /** These replacements are necessary if you render to raw HTML*/\n
                        text = text.replace(/&/g, \"&amp;\");\n
                        text = text.replace(/</g, \"&lt;\");\n
                        text = text.replace(/>/g, \"&gt;\");\n
                        text = text.replace('\\n', '<br>', 'g');\n
                        console.log(text);\n
\n
                        if (element) {{\n
                            element.value += text + \"\\n\";\n
                            element.scrollTop = element.scrollHeight; /* focus on bottom */\n
                        }}\n
                    }};\n
                }})(),\n
                printErr: function (text) {{\n
                    if (arguments.length > 1) text = Array.prototype.slice.call(arguments).join(' ');\n
\n
                    console.error(text);\n
                }},\
                canvas: (function () {{\
                    var canvas = document.querySelector('#canvas');\n
\
                    /** As a default initial behavior, pop up an alert when webgl context is lost.\n
                    / * To make your application robust, you may want to override this behavior before shipping!\n
                    / * See http://www.khronos.org/registry/webgl/specs/latest/1.0/#5.15.2\n
                     **/\n
                    canvas.addEventListener(\"webglcontextlost\", function (e) {{ alert('WebGL context lost. You will need to reload the page.'); e.preventDefault(); }}, false);\n
\n
                    return canvas;\n
                }})(),\n
                setStatus: function (text) {{\n
                    if (!Module.setStatus.last) Module.setStatus.last = {{ time: Date.now(), text: '' }};\n
                    if (text === Module.setStatus.last.text) return;\n
\n
                    var m = text.match(/([^(]+)\\((\\d+(\\.\\d+)?)\\/(\\d+)\\)/);\n
                    var now = Date.now();\n
\n
                    if (m && now - Module.setStatus.last.time < 30) return; // If this is a progress update, skip it if too soon\n
\n
                    Module.setStatus.last.time = now;\n
                    Module.setStatus.last.text = text;\n
\n
                    if (m) {{\n
                        text = m[1];\n
                        progressElement.value = parseInt(m[2]) * 100;\n
                        progressElement.max = parseInt(m[4]) * 100;\n
                        progressElement.hidden = true;\n
                        spinnerElement.hidden = false;\n
                    }} else {{\n
                        progressElement.value = null;\n
                        progressElement.max = null;\n
                        progressElement.hidden = true;\n
                        if (!text) spinnerElement.style.display = 'none';\n
                    }}\n
\n
                    statusElement.innerHTML = text;\n
                }},\n
                totalDependencies: 0,\n
                monitorRunDependencies: function (left) {{\n
                    this.totalDependencies = Math.max(this.totalDependencies, left);\n
                    Module.setStatus(left ? 'Preparing... (' + (this.totalDependencies - left) + '/' + this.totalDependencies + ')' : 'All downloads complete.');\n
                }},\n
                //noInitialRun: true\n
            }};\n
\n
            Module.setStatus('Downloading...');\n
\n
            window.onerror = function () {{\n
                Module.setStatus('Exception thrown, see JavaScript console');\n
                spinnerElement.style.display = 'none';\n
                Module.setStatus = function (text) {{ if (text) Module.printErr('[post-exception status] ' + text); }};\n
            }};\n
        </script>\n
\n
        <!-- REF: https://developers.google.com/web/updates/2018/11/web-audio-autoplay -->\n
        <script type='text/javascript'>\n
            var audioBtn = document.querySelector('#btn-audio');\n
\n
            /* An array of all contexts to resume on the page */\n
            const audioContexList = [];\n
            (function () {{\n
                /** A proxy object to intercept AudioContexts and \n
                / * add them to the array for tracking and resuming later\n
                 **/\n
                self.AudioContext = new Proxy(self.AudioContext, {{\n
                construct(target, args) {{\n
                        const result = new target(...args);\n
                        audioContexList.push(result);\n
                        if (result.state == \"suspended\") audioBtn.value = \"🔈 RESUME\";\n
                        return result;\n
                    }}\n
                }});\n
            }})();\n
\n
            function toggleAudio() {{\n
                var resumed = false;\n
                audioContexList.forEach(ctx => {{\n
                    if (ctx.state == \"suspended\") {{ ctx.resume(); resumed = true; }}\n
                    else if (ctx.state == \"running\") ctx.suspend();\n
                }});\n
\n
                if (resumed) audioBtn.value = \"🔇 SUSPEND\";\n
                else audioBtn.value = \"🔈 RESUME\";\n
            }}\n
        </script>\n
        <script>\n
            /** // This is read and used by `site.js`\n
            / * var Module = {{\n
            / *     wasmBinaryFile: \"raylib_showcase.wasm\"\n
            / * }}\n
             **/\n
        </script>\n
        <script src=\"{crate_name}.js\"></script>\n
    </body>\n
\n
    </html>
        ").to_string();

        match fs::write(out_filepath, index_html) {
            Ok(_) => println!(
                "cargo::warning=\"Created index.html. Remember to copy the {crate_name}.data file from deps.\""
            ),
            Err(err) => println!(
                "cargo::error=\"Failed to create index.html withh error: {}\"",
                err
            ),
        }
    }
}
