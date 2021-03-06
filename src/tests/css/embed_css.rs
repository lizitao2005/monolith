use reqwest::blocking::Client;
use std::collections::HashMap;

use crate::css;

//  ██████╗  █████╗ ███████╗███████╗██╗███╗   ██╗ ██████╗
//  ██╔══██╗██╔══██╗██╔════╝██╔════╝██║████╗  ██║██╔════╝
//  ██████╔╝███████║███████╗███████╗██║██╔██╗ ██║██║  ███╗
//  ██╔═══╝ ██╔══██║╚════██║╚════██║██║██║╚██╗██║██║   ██║
//  ██║     ██║  ██║███████║███████║██║██║ ╚████║╚██████╔╝
//  ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝

#[test]
fn passing_empty_input() {
    let cache = &mut HashMap::new();
    let client = Client::new();

    assert_eq!(css::embed_css(cache, &client, "", "", false, false,), "");
}

#[test]
fn passing_style_exclude_unquoted_images() {
    let cache = &mut HashMap::new();
    let client = Client::new();

    const STYLE: &str = "/* border: none;*/\
background-image: url(https://somewhere.com/bg.png); \
list-style: url(/assets/images/bullet.svg);\
width:99.998%; \
margin-top: -20px; \
line-height: -1; \
height: calc(100vh - 10pt)";

    assert_eq!(
        css::embed_css(
            cache,
            &client,
            "https://doesntmatter.local/",
            &STYLE,
            true,
            true,
        ),
        format!(
            "/* border: none;*/\
background-image: url('{empty_image}'); \
list-style: url('{empty_image}');\
width:99.998%; \
margin-top: -20px; \
line-height: -1; \
height: calc(100vh - 10pt)",
            empty_image = empty_image!()
        )
    );
}

#[test]
fn passing_style_exclude_single_quoted_images() {
    let cache = &mut HashMap::new();
    let client = Client::new();

    const STYLE: &str = "/* border: none;*/\
background-image: url('https://somewhere.com/bg.png'); \
list-style: url('/assets/images/bullet.svg');\
width:99.998%; \
margin-top: -20px; \
line-height: -1; \
height: calc(100vh - 10pt)";

    assert_eq!(
        css::embed_css(cache, &client, "", &STYLE, true, true,),
        format!(
            "/* border: none;*/\
background-image: url('{empty_image}'); \
list-style: url('{empty_image}');\
width:99.998%; \
margin-top: -20px; \
line-height: -1; \
height: calc(100vh - 10pt)",
            empty_image = empty_image!()
        )
    );
}

#[test]
fn passing_style_block() {
    let cache = &mut HashMap::new();
    let client = Client::new();

    const CSS: &str = "\
#id.class-name:not(:nth-child(3n+0)) {\n  \
  // border: none;\n  \
  background-image: url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=');\n\
}\n\
\n\
html > body {}";

    assert_eq!(
        css::embed_css(cache, &client, "file:///", &CSS, false, true,),
        CSS
    );
}

#[test]
fn passing_attribute_selectors() {
    let cache = &mut HashMap::new();
    let client = Client::new();

    const CSS: &str = "\
[data-value] {
    /* Attribute exists */
}

[data-value='foo'] {
    /* Attribute has this exact value */
}

[data-value*='foo'] {
    /* Attribute value contains this value somewhere in it */
}

[data-value~='foo'] {
    /* Attribute has this value in a space-separated list somewhere */
}

[data-value^='foo'] {
    /* Attribute value starts with this */
}

[data-value|='foo'] {
    /* Attribute value starts with this in a dash-separated list */
}

[data-value$='foo'] {
    /* Attribute value ends with this */
}
";

    assert_eq!(css::embed_css(cache, &client, "", &CSS, false, false,), CSS);
}

#[test]
fn passing_import_string() {
    let cache = &mut HashMap::new();
    let client = Client::new();

    const CSS: &str = "\
@charset 'UTF-8';\n\
\n\
@import 'data:text/css,html{background-color:%23000}';\n\
\n\
@import url('data:text/css,html{color:%23fff}')\n\
";

    assert_eq!(
        css::embed_css(
            cache,
            &client,
            "https://doesntmatter.local/",
            &CSS,
            false,
            true,
        ),
        "\
@charset 'UTF-8';\n\
\n\
@import 'data:text/css;base64,ZGF0YTp0ZXh0L2NzcyxodG1se2JhY2tncm91bmQtY29sb3I6IzAwMH0=';\n\
\n\
@import url('data:text/css;base64,ZGF0YTp0ZXh0L2NzcyxodG1se2NvbG9yOiNmZmZ9')\n\
"
    );
}

#[test]
fn passing_hash_urls() {
    let cache = &mut HashMap::new();
    let client = Client::new();

    const CSS: &str = "\
body {\n    \
    behavior: url(#default#something);\n\
}\n\
\n\
.scissorHalf {\n    \
    offset-path: url(#somePath);\n\
}\n\
";

    assert_eq!(
        css::embed_css(
            cache,
            &client,
            "https://doesntmatter.local/",
            &CSS,
            false,
            true,
        ),
        CSS
    );
}
