lemmy-scheduled-post-bot
=========================

Make posts to Lemmy based on the next configured post in a [TOML file](https://toml.io).

1. Set up a configuration file as described below.
1. Run this program with that file as an argument, e.g. `lemmy-scheduled-post-bot ./sample.toml`.

This program then does the following:

1. Identifies the configured post whose publish time has most recently passed.
1. Checks to see if a post with the same instance/community/user/title has already been published, and if so exits early.
1. Publishes the post.

Note that this does not run as a service, and must be invoked directly to run. It's recommended to use in conjunction with `cron` or the Windows Task Scheduler.

Configuration
--------------

This program takes as an argument the filename of a TOML file. This TOML file can contain the following sections:

### `settings` (required)

The `[settings]` section must contain the following fields:

* `instance` - The domain of an instance of Lemmy, i.e. `"lemmy.world"`.
* `community` - The community on that instance to which to post.
* `username_or_email` - The login of the account that will make the post.
* `password` - The password of the account.
* `enabled` - If absent or `false`, the input file is validated but no server communication is attempted.

### `defaults` (optional)

The `[defaults]` section can contain all the same fields as `[[post]]`, and if a field is absent in the post, the default field's value is used.

### `post`

One or more `[[post]]` sections can be supplied, each of which can contain the following fields:

* `post_at` (required) - A date and time when the post should be published. Should be a string in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format.
* `title` - The name of the post.
* `body` - The text content of the post in Markdown format.
* `link` - The URL content of the post.
* `thumbnail` - A URL to an image that should appear alongside the post.
* `alt_text` - A text description of the thumbnail.


Templating
-----------

The post fields support limited template substitution. Supply custom fields in the `[defaults]` or `[[post]]` sections, and reference them within the post fields within braces like so: `"My name is {name}."`. Templating is not recursive, and only the defined post fields will be interpolated. See [sample.toml](./sample.toml) for a full example.

License
--------

Copyright (C) 2024 Steven Vergenz
Licensed under the terms of the General Public License 3.0 or later
