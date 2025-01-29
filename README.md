# aenrisite

hii its my site

## implementation notes (for myself, mostly)

### `aenrisite`

the main package of the site, holds actix bootstrapping code and ties everything
together

### `aenrisite::web`

the core of the site itself, turning fragments and components into pages

### `aenrisite::web::framework`

the items used to create the site, fragments and components

### `aenrisite::web::blog`

both the templates for and the code for generating blog posts based on post entries,
supports markdown through `marker` and dtf through `marker-dtf` (soon)

### `marker`

wrapper around `comrak` that adds some additional parsing to markdown, specifically
for use within my website

### `dtf`

basic parser for ida deerz's "deer text format"

### `marker-dtf`

wrapper around dtf that converts dtf files into a `marker`-compatible format

### `awrf`

abstract web rust framework, some silly thing to make my specific kind of webdev
on rust bearable. 
