# obsv-notif

This crate provides the utilities for notifications.

It is structured around the `Channel` trait, which represents a notification channel.

A number of built-in channels are provided:

- `webhook`
- `email`

## Testing

- `webhook`: tests require a server (`brew install nlargueze/formulas/http-intercept`)
- `email`: tests require an insecure smtp server listeining on 1025 (see `mailpit`, or `mailhog` - old, `mailcrab` - new, untested).
