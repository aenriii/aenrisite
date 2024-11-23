# marker::transpile::dtf

this module is a wrapper around a custom `dtf` package, which
is a parser for ida deerz's [dtf](https://dtf.deerz.one) markup
language.

## deviations

### slight change in the `dtf` dynamic item's resolution

the spec defined the `dtf` element to refer directly to a .dtf file,
however due to the nature of this library, if
`TranspilerOptions.dtf_is_relative` is set to `true`, it will resolve
to a `Node::Link` with a path relative to the current page instead of
seeking a relative file.

### some mixins/dynamic items will not appear as mixins in the node tree

#### `link` and `image-link`

these mixins will both appear as `Node::Link`, with `image-link` having
an immediate child of `Node::Media` and `link` having an immediate child
of `Node::Text`

#### `image`

this mixin will appear as `Node::Media`

#### `desc`/`description`

this mixin will simply be a `Node::Container` with borders specified and
an immediate child of `Node::Paragraph` holding the text of the description
parsed and stylized.

<!-- ### `config.dtf` does not have a hardcoded location

in the current primary parser, `config.dtf` has a hardcoded location and
is required for the page to load correctly. for a `config.dtf` and
subsequent theming files to be loaded, `TranspilerOptions.config_location`
must be set to a valid path. -->

### all k/v pairs with meta tag syntax will be read and stored

these will fit into the `options` field of their given location.

### `header` and `footer` meta tags will be within `options`, but otherwise ignored

providing a header and footer are typically the job of site generation,
which this library does not provide. the `dtf` parser is available as
its own package if you would like to parse the header and footer and
subsequently add them to your page.
