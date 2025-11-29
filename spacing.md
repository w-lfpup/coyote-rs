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

### No extra spaces

A template without trailing spaces:

```rust
tmpl("<span>hai :3</span>", [])
```

Will output without trailing spaces:

```html
<p>hai :3</p>
```

### Trailing spaces

A template with trailing spaces:

```rust
tmpl("<p>   hai   :3   </p>", [])
```

Will output trailing spaces:

```html
<p> hai :3 </p>
```

### New lines

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

Will output every new line.
```html
<p>

	hai :3

</p>
```

## Attributes

Attribute spacing will collapse spaces _and_ new lines.

```html
<p attr></p>
```

```html
<p
	attr></p>
```

```html
<p attr    attr2    att3></p>
```

```html
<p
	attr

	attr

	attr>
</p>
```

## Injections

Injections repeat the spacing that preceeded them.

So if a `space` is followed by an injection, the injections will be preceeded by a `space`.

Likewise, if a `new line` is followed by an injection, the injections will be preceeded by a `new line`.

### Attribute injections

```html
<p {}></p>
<p
	{}></p>
<p >
```

### Descendant injections

```html
<p>{}</p>
<p> {} </p>
<p>
	{}
</p>
<p>
	{} {}
<p>
```