# Mycroft Rust library

This is a small libary containing tools to build services and skills for [Mycroft](https://github.com/mycroftai/mycroft-core)) in Rust.

The library is in a very early stage of development but support enough of the Mycroft infrastructure to create simple skills using Adapt intents.

As said it is very early days, the skill can be started and the intents are created, however the skill shutdown is not implemented and the registered intents aren't removed at the moment.

for a simple example skill check the example folder.

## Helping out

I'd very much appreciate help developing this, I don't really know Rust and Mycroft has a big API and many functions.

### Things to work on
- Loading vocabulary from file
- Dialog renderer
- Localization
- Audioservice API
- Communication with the Mycroft backend
- Tests

## Please note

I (Åke, the author of this) has very little experience with Rust, and so far this has been a huge learining experience.
