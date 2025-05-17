# Default `index.html`

<style>
/*
The syntax highlighter theme makes nested languages slightly transparent. This makes it hard to
read CSS and JS embedded in HTML, so we disable it.
*/
.xml .javascript,
.xml .css {
  opacity: 1.0 !important;
}
</style>

This is the default `index.html` that `bevy build web` and `bevy run web` uses to load your application. You may customize `index.html` by creating a `web/index.html` to override the default. The default is provided below, so you can copy it instead of writing your own from scratch:

<div class="warning">

**Warning**

The default `index.html` has the following line:

```js
import init from "./build/bevy_app.js";
```

You will need to replace `bevy_app` with the name of your compiled binary. This is usually your crate's name, but it can be customized in `Cargo.toml` with the `[[bin]]` table, so be careful!

</div>

```html
{{#include ../../../../assets/web/index.html}}
```
