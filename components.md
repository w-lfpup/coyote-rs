# Components

`Coyote` creates documents with components.

## Function Components

Function components are functions that return components!

```rust
use coyote_rs::components::{Component, tmpl};

fn hello_world() -> Component {
    tmpl("<p>hai :3</p>", [])
}
```

## The template component

## Tags, void elements, fragments

`Coyote` templates support self-closing tags, void elements, and jsx-like fragments:

```rs
fn syntax_story() -> Component {
    tmpl("
        <article>
            <>
                <p>no waaaay?</p>
                <custom-element />
                <input type=button value='high-five!' />
            </>
        </article>
    ", [])
}
```

However, `coyote` will only output w3-spec compliant HTML:

```html
<article>
    <p>no waaaay?</p>
    <custom-element></custom-element>
    <input value=button value="high-five!">
</article>
```

This provides an robust template syntax while adhering modern HTML standards.

## Injections

`Injections` create more complex components with template nesting and attribute assignments.

There are only two valid _injections_ in a `tmpl` component:
- attribute injections
- descendant injections

Likewise there are only two valid injection locations in a `tmpl` component:

```rs
fn injection_story() -> Component {
    let attribute = attr("uwu");
    let descendant = text("hai :3")

    tmpl("
        <article {}>
            {}
        </article>
    ", [attribute, descendant])
}
```

Which renders:

```html
<article uwu>
    hai :3
<article>
```

Any other instance of `{}` in a template component will not be considered an injection.

### Escape the `{` character

To use a `{` in a template without creating an injection, use the left-bracket html escape charactor `&123;`.

So ...

```html
helloooo { world }
```

would look like the following as a template:

```rust
tmpl("hellooo, &#123; world }", []); 
```

## Nested templates

The `list` and `vlist` components immitate the `node -> [node, text, node, ...]` heiarchy of an xml-like document.

The example below creates a form defined by lists of attributes, templates, and text.

```rust
use coyote_rs::{Component, attr_val, list, text, tmpl};

fn submit_button() -> Component {
    tmpl("<input type=submit value='yus -_-'>", [])
}

fn form() -> Component {
    let attributes = [
        attr_val("action", "/uwu"),
        attr_val("method", "post"),
    ];

    let mut descendants: Vec<Component> = Vec::new();
    descendants.push(text("you're a good dog aren't you >:3"));
    descendants.push(submit_button());
    
    tmpl(
        "<form {}>
            {}
        </form>",
        [list(attributes), vlist(descendants)],
    )
}
```

And the output will be:

```html
<form action="/uwu" method="post">
    you're a good dog aren't you >:3
    <input type=submit value="yus -_-">
</form>
```

## Types of components

`Components` are the atomic chunks used to build documents.

```rs
use coyote_rs::{
    attr,
    attr_val,
    text,
    unescaped_text,
    tmpl,
    tmpl_string,
    list,
    vlist,
}
```

#### Attribute

an element attribute

```rs
attr(name: &str)
```

#### Attribute with value

an attribute and value pair

```rs
attr_val(name: &str, value: &str)
```

#### Text

text with the HTML-safe escaped text

```rs
text(text_str: &str)
```

#### Unescaped text

dangerously unescaped text

```rs
unescaped_text(text_str: &str)
```

#### Template

a document fragment described by a static string template and a list of injections

```rs
tmpl(template_str: &'static str, injections: [Component; N])
```

#### Template string

a document fragment described by a string template and a list of injections

```rs
tmpl_string(template_str: &str, injections: [Component; N])
```

#### List

a list of components

```rs
list(components: [Component; N]) -> Component
```

#### Vector list

a vector list of components

```rs
vlist(components: Vec<Component>)-> Component
```

#### None

the abscence of a component

```rs
Component::None
```

## Components as an IMR

Components are not quite HTML or XML.

Components are an (I)ntermediate (R)endering (F)ormat.

They are the _potential_ for a document like HTML or XML.

## Document builders

Coyote renders components with [document builders](./document_builders.md).
