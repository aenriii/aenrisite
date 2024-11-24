# marker::transpile::md

this module is currently a wrapper for the `comrak` package, and
powers markdown transpilation into the `Page` struct.


## deviations

### feature: subheadings

a header where the first non-whitespace string is a line starting in -- will
have the remaining contents of that line viewed as a subheader

example:
```md
# Header
-- subheader
```

example 2:
```md
# writing a dynamic site in only serverside rust
-- fast, efficient, and why you shouldn't do it
```

### feature: tags and k/v pairs

a text block (header's text, quote, paragraph, etc) which does not
have a link at the beginning but does start with a section of text
within brackets (`[]`) will have tags parsed out of the contents of
those brackets and whitespace prior to the succeeding text will be
ignored

example:
```rs
"[align=center;italic] hi, im some text!"
// turns into
Node::Paragraph {
    ...,
    nodes: &[Node::Text {
        content: "hi, im some text!",
        flags: &["italic"],
        options: &[("align", "center")]
    }]
}
```

### feature: sections

define sections/containers manually! this allows you to use flex-box
logic in your site. open a container with `#{`, and close with `}#`.
they must be on lines alone, and to set a container as a flex you can
just open the container with `#{ flex (row|column);`!

containers are recursive! just open another and be sure to close it!
you can set what percent / fraction of a given flex container another
container is naturally allowed to take up by opening a nested container
with `#{ size 1/3` (for 1/3)

example of a flex container with two unequal pieces
```md
#{ flex row;

 #{ size 2/5;

  # some area!

  this place can contain normal content, images, tables, and anything
  else you'd expect from a container!

  ![image](https://example.com/image.png)

 }

 #{ size 3/5

  ## now, a table that takes up the rest of the space

  | foo | bar | baz |
  | --- | --- | --- |
  | 111 | 222 | 333 |
  | 111 | 222 | 333 |
  | 111 | 222 | 333 |
 }

}#
