# Mycroft Rust library

This is a small libary containing tools to build services and skills for [Mycroft](https://github.com/mycroftai/mycroft-core) in Rust.

This library does NOT mean theat it's a good idea to write Mycroft skills or services in Rust. It doesn't mean it's a bad idea either. It just means that it can be done and it may be fun and/or interesting.

The library is in a very early stage of development but support enough of the Mycroft infrastructure to create simple skills using Adapt intents.

As said it is very early days, the skill can be started and the intents are created, however the skill shutdown is not implemented and the registered intents aren't removed at the moment. There are a lot of limitations, for example the skills only connects to the default mycroft-core websocket url.

But it works! For a simple example skill check the example folder.

## Building

The library can easily be built using Cargo

```
cargo build
```

## Helping out

I'd very much appreciate help developing this, I don't really know Rust and Mycroft has a big API and many functions.

### Things to work on

If you think this sounds fun there are a bunch of things that's just waiting to
be done:
- Loading vocabulary from file
- Dialog renderer
- Localization
- Audioservice API
- Communication with the Mycroft backend
- Tests

## Please note

I (Åke, the author of this) has very little experience with Rust, and so far this has been a huge learining experience. The quality of this library should not be taken as an indication of the quality of the Mycroft Virtual Assistant, any faults are likely mine.
