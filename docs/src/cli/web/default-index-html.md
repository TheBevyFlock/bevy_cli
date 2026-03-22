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

> [!NOTE]
> The default `index.html` has the following line:
>
> ```js
> import init from "{% entrypoint %}";
> ```
>
> The CLI automatically replaces `{% entrypoint %}` with the name of the generated JS entrypoint. Make sure to use it for your custom `index.html`

```html
{{#include ../../../../assets/web/index.html}}
```
