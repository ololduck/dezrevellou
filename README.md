# Dezrevelloù
_still alpha af_

Dezrevelloù is a lightweight (js + css: 4.6k, 1.6k gzip'd) commenting solution aimed to (but not only) static websites.
Its name is derived from the [breton] word for "comment".

It comprises some client-side javascript and css, and a REST API.

Since it is written in [Rust], it can be used with any http server handling [`proxy_pass`] or equivalent.

## Installation & Usage

The installation is more complicated than i'd like, "thanks" to [CORS].

### The api

The api is responsible for handling the persistent storage of comments, as well as querying them.
As said, it is a binary exposing an HTTP interface that runs well in [docker], then proxyfied by a "real" webserver.

It can also serve the client part via `/static/dezrevellou.min.{css,js}`.

### The client part

Dezrevelloù also handles client rendering via a JS script & a minimal css stylesheet, both made to be as light as possible
to my knowledge.

A simple usage in a "static" webpage would look like the following:

```html
<html>
<head>
    <link type="text/css" href="http://blog.my.domain/static/dezrevellou.min.css" />
    <script type="text/javascript" src="http://blog.my.domain/static/dezrevellou.min.js"></script>
</head>
<body>
<div id="dezrevellou"><!-- you can name the id as you pass the same id to the instanciation below --></div>
<script type="text/javascript">
    window.addEventListener("DOMContentLoaded", function () {
        // function(baseUrl, articleSlug, elemId)
        new Dezrevellou("https://blog.my.domain", "my-article-slug", "dezrevellou");
    });
</script>
</body>
</html>
```
#### specific case: a domain-wide instance

Let's suppose you want to use a single instance of dezrevelloù for all your websites: you would typically install it in 
its own subdomain.

I mentioned [CORS] previously, and here's the catch:

The server serving the previous HTML __must__ add a header to its response,
allowing CORS requests to the `https://comments.my.domain/`. An exemple of this, using nginx would look like this:

```
server {
    # --8<--
    server_name blog.my.domain;
    location / {
        # --8<--
        
        if ($request_method = OPTIONS) {
            return 204;
        }
        
        root /var/www/my-awesome-blog/html;
        add_header Access-Control-Allow-Origin https://blog.my.domain/;
        add_header Access-Control-Max-Age 3600;
        add_header Access-Control-Expose-Headers Content-Length;
        add_header Access-Control-Allow-Headers Range;
        # --8<--
    }
}
```

### Recommended solution

My personnal recommendation is to use the same domain for both the static website & the comment api.

This is done by configuring your favourite webserver to serve different content from different locations.

For instance, `/comments` would pass the request to the api, while the rest of the locations would
simply return your static website, containing your own `dezrevellou.min.{css,js}`.

You will find below a sample nginx configuration, [used for testing](vhost.nginx)
in the [docker-compose file](docker-compose.yml):

```
server {
    listen 80;
    gzip on;
    location /comments {
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_pass http://api:3000;
    }
    location / {
        root /var/www/blog.my.domain/html;
        index index.html;
    }
}
```

`/var/www/blog.my.domain/html` contains the html, and a subdirectory containing the `dezrevellou.min.{css,js}` files.

## Features

### core

- [x] no-login comments
- [ ] edition ability
- [ ] moderation
- [ ] notifications
- [ ] replies
- [ ] sqlite
- [ ] translation support

### nice to have

- [ ] security
- [ ] votes
- [ ] markdown
- [ ] spam auto-detection
- [ ] gravatar?
- [ ] [fake mode](https://github.com/tessalt/echo-chamber-js)?

## Development

You will need git, npm (for js & css compression) and a [rust] development environment.

```shell-session
$ git clone git@github.com:paulollivier/dezrevellou.git
$ make
```

This will build everything:
- the api binary
- the js & css files, compressed & uncompressed 
- a demo.html page

Everything will be located in the `dist/` directory.

[breton]: https://en.wikipedia.org/wiki/Breton_language
[rust]: https://rust-lang.org
[`proxy_pass`]: https://docs.nginx.com/nginx/admin-guide/web-server/reverse-proxy/
[CORS]: https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS