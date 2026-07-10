fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "emscripten" {
        use std::{env, fs, path::Path};
        let crate_name = env!("CARGO_PKG_NAME");
        let crate_desc = env!("CARGO_PKG_DESCRIPTION");

        let out_dir = env::var("OUT_DIR").unwrap();
        let out_filename = format_args!("{}/../../../index.html", out_dir).to_string();
        let out_filepath = Path::new(&out_filename);
        let index_html = format_args!(
            "
<!DOCTYPE html>
<html lang=\"en\">
      <head>
            <meta charset=\"UTF-8\" />
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />
            <title>{crate_name}</title>

            <meta name=\"title\" content=\"{crate_name}\">\n
            <meta name=\"description\" content=\"{crate_desc}\">\n

            <meta property=\"og:title\" content=\"{crate_name}\">\n
            <meta property=\"og:description\" content=\"{crate_desc}\">\n

            <meta property=\"twitter:card\" content=\"summary\">\n
            <meta property=\"twitter:title\" content=\"{crate_name}\">\n
            <meta property=\"twitter:descripion\" content=\"{crate_desc}\">\n

            <link rel=\"shortcut icon\" href=\"https://www.raylib.com/favicon.ico\">
            
            <style>
                  html, body {{
                        margin: 0;
                        height: 100%;
                        background: #000;
                        overflow: hidden;
                  }}
                  body {{
                        display: grid;
                        place-items: center;
                  }}
                  canvas {{
                        display: block;
                        max-width: 100vw;
                        max-height: 100vh;
                        image-rendering: pixelated;
                  }}
            </style>
      </head>
      <body>
            <canvas
                  id=\"canvas\"
                  oncontextmenu=\"event.preventDefault()\"
                  tabindex=\"-1\"
            ></canvas>
            <script>
                  // Browsers block AudioContext until the user interacts with the page.
                  // Resume on first click so raylib's audio comes through.
                  window.addEventListener(\"click\", function () {{
                        var Ctx = window.AudioContext || window.webkitAudioContext;
                        if (!Ctx) return;
                        [Ctx, Module && Module.audioContext].forEach(function (ctx) {{
                              if (ctx && ctx.state === \"suspended\") ctx.resume();
                        }});
                  }}, {{ once: true }});
                  var Module = {{
                        canvas: document.getElementById(\"canvas\"),
                        print: function (t) {{
                              console.log(t);
                        }},
                        printErr: function (t) {{
                              console.error(t);
                        }},
                  }};
            </script>
            <script src=\"{crate_name}.js\"></script>
      </body>
</html>
        "
        )
        .to_string();

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
