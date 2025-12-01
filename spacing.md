# Spaces

`Coyote` _never_ adds spaces or new lines.

Space is compositional in HTML. It has weight and meaning.

`Coyote` outputs html that respects the lines and spaces defined by templates, text nodes, and attribute values.

So every new line and space is intentional.

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

			'
	></p>
	",
	[]
)
```

Will collapse spaces but preserve new lines on render:

```html
<p
	attr='

		hai :3 hello!

		'
></p>
```

## Injections

### Attribute injections

When a space is followed by an attribute injection:

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
		{}></p>
	",
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

#### Spaces

Coyote parses the space between components as if they were one contiguous document.

The result is something similar to how a browser might collapse spaces and lines.

The following 

```rs
let descendants = list([
	tmpl(" <span>hai :3</span> ", []),
	tmpl(
		"
		<span>hello</span>
		", []),
]);

tmpl(
	"<p>{}</p>",
	[descendants]
)
```

And those spaces are rendered accordingly:

```html
<p> <span>hai :3</span>
	<span>hello</span></p>
```

#### Text components

Text components preserve all new lines and collapse all spaces.

```rs
let text_component = text(
	"

	hai   :3

	"
);
	
tmpl(
	"
	<p>{}</p>
	",
	[text_component]
);
```

Those spaces are rendered accordingly:

```html
<p>
	hai :3
</p>	
```