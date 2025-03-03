# general types

## expression

any valid rust expression, for example `a+b`, `a`, `a.b().c(|x| x.d())`

## identifier

any valid rust identifier, for example `for`, `SomeComponent`

## statement

always ends with `;`

```rs
// Type 1: Components, with or without an inner block

ComponentName:ident (key_or_flag:ident, key_or_flag:ident = value:expr) /* (optional) */ {
    inner_block:block
};

// Type 2: control flow: if

if boolean:expr {
    inner_block:block
};

// Type 3: control flow: for

for var_name:ident in iterable:expr {
    inner_block:block
};
```

## block

semicolon-seperated list of statements`

## macro-specific types

### route!: additional statements

```rs
// type 4: use() from route template

use name:ident;
use template_name:ident as name:ident;
use name:ident : type; // asserts that name is of type `type`

```

# macro syntax

## compose!{ }

awrf::compose! simply takes a single `block` or `statement` and returns either a Vec<Box<dyn Component>> or a Component

```rs
// example
awrf::compose! {
	Page (key = "value", ...) {
		YourAwesomeHeader(title = "My Awesome Site!");

		Container {
			for post in blog_posts {
				if !post.private {
					BlogPostMini(post = post);
				}
			}
		}
	}
}
```

## route!( )

awrf::route! takes in first a literal template string for the route, then a block, and returns a special Route item

```rs
// example
awrf::route!("/blog/[post_id]?iframe_view", {
    use iframe_view:bool;
    use post_id;

    let post = get_post_by_id(post_id);

    if iframe_view {
      BlogPost(post = post);
    } else {
      PageWrapper {
        BlogPost(post = post);
      }
    }
});
```

## router!{ }

awrf::router! creates a router from
