# Spaces

`Coyote` _never_ adds spaces or new lines.

Spaces are part of the composition in HTML. They have weight and meaning.

`Coyote` outputs html that respects the lines and spaces defined by templates and text injections.

So every new line and space is intentional.

## A few expectations

There are a couple broad expectations when writing templates:
- spaces collapse
- new lines do not collapse
- injections repeat their preceeding space

## Templates

The following examples demonstrate how spaces collapse in templates.

### No added spaces

A template without trailing spaces:

```rust
tmpl("<p>hai :3</p>", [])
```

Will output without trailing spaces:

```html
<p>hai :3</p>
```

### Collapse spaces

A template with trailing spaces:

```rust
tmpl("<p>   hai   :3   </p>", [])
```

Will output collapsed spaces:

```html
<p> hai :3 </p>
```

### Preserve new lines

`Coyote` respects new lines found in template text nodes.

New lines are often used by developers to visually organize content.

So a template with new lines:

```rust
tmpl("
	<p>

		hai :3
	
	</p>
	",
	[]
)
```

Will output every new line:
```html
<p>

	hai :3

</p>
```

### Attributes

Attribute spacing is somewhat special. It collapses spaces _and_ new lines.

So a template with attributes spaced out:

```rs
tmpl("<p    attr    attr2    att3    ></p>", [])
```

Will collapse spaces on render:

```html
<p attr attr2 att3></p>
```

And a template with new lines:

```rs
tmpl("
	<p

		attr

		attr2

		attr3>

	</p>
	",
	[]
)
```

Will collapse new lines on render:

```html
<p
	attr
	attr2
	attr3>
</p>
```

### Attribute values

Attribute value spacing will only collapse spaces.

So a template with a multi-line attribute value:

```rs
tmpl("
	<p
		attr='

		hai   :3    hello!

		'></p>
	",
	[]
)
```

Will collapse spacing but preserve new lines on render:

```html
<p
	attr='

	hai :3 hello!

	'></p>
```


## Injections

Injections repeat the spacing that preceeds them.

So if a `space` is followed by an injection, the injections will be preceeded by a `space`.

Likewise, if a `new line` is followed by an injection, the injections will be preceeded by a `new line`.

### Attribute injections

So when a space is followed by an attribute injection:

```rs
let attributes = list([
	text("hai"),
	text("hello"),
]);

tmpl(
	"<p {}></p>",
	[attributes]
)
```

A template will output spaces before attributes:

```html
<p hai hello></p>
```

And when a new line is followed by an attribute injection:

```rs
let attributes = list([
	attr("hai"),
	attr("hello"),
]);

tmpl(
	"<p
		{}></p>",
	[attributes]
)
```

A template will output new lines before attributes:

```html
<p
	hai
	hello></p>
```

### Descendant injections

Similarly, if a descendant injection is preceeded by a space (or the start of a new tag):

```rs
let descendants = list([
	tmpl("<span>hai :3</span>", []),
	tmpl("<span>hello</span>", []),
]);

tmpl(
	"<p>{}</p>",
	[descendants]
)
```

Then the template will render descendants preceeded by a space:

```html
<p><span>hai :3</span> <span>hello</span></p>
```

If a descendant injection is preceeded by a new line:

```rs
let descendants = list([
	tmpl("<span>hai :3</span>", []),
	tmpl("<span>hello</span>", []),
]);

tmpl(
	"
	<p>
		{}
	</p>
	",
	[descendants]
)
```

Then a template will render descendants preceeded by new lines:

```html
<p>
	<span>hai :3</span>
	<span>hello</span>
</p>
```