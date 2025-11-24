# Spaces

Spaces have weight and meaning in HTML. They are a part of the composition of an HTML document.

`Coyote` _never_ adds spaces or new lines.

Developers can write HTML components expecting browser parity. As in, their HTML will render without any unexpected new lines or spaces.

HTML in, HTML out. No suprises. No mental overhead.

## No additive mutations

`Coyote` will output w3 spec-compliant HTML while respecting the lines and spaces defined in a template. So every new line and space is intentionally composed by the developer.

### No spaces

A template without spaces:

```rust
tmpl("<span>hai :3</span>", [])
```

Will output without spaces:

```html
<p>hai :3</p>
```

### Trailing spaces

A template with trailing spaces:

```rust
tmpl("<p> hai :3 </p>", [])
```

Will output trailing spaces:

```html
<p> hai :3 </p>
```

### New lines

A template with new lines and block elements:

```rust
tmpl("
	<p>
		hai :3
	</p>
	",
	[]
)
```

Will output new lines.
```html
<p>
	hai :3
</p>
```

A template with new lines and inline elements:

```rust
tmpl("
	<span>
		hai :3
	</span>
	",
	[]
)
```

Will output new lines without indentation:

```html
<span>
hai :3
</span>
```

## Collapse spaces, not new lines

`Coyote-rs` will collapse spaces but not new lines.

For example, a template with extra new lines and spaces:

```rust
tmpl("
	<p>
		
		hai       :3

		hello     ^_^

		UwU       hai

	</p>
	",
	[]
)
```

Will output html with extra new lines with spaces collapsed:

```html
<p>

	hai :3

	hello ^_^

	UwU hai

</p>
```

Whereas a browser might render as:

```html
<p>
	hai :3
	hello ^_^
	UwU hai
</p>
```

But both will _visually_ render just the same.

## Injections



