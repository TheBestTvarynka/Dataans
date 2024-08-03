[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://stand-with-ukraine.pp.ua/)

Table of content:

- [Totes](#totes)
  - [Motivation](#motivation)
  - [App philosophy](#app-philosophy)
  - [Installation](#installation)
  - [Contributing](#contributing)
  - [Meta](#meta)

# Totes

> _Why **totes**?_

Teleram notes -> T[elegram n]otes -> Totes. Get it?

Get it? :wink: :grin:

![](https://totes.qkation.com/imgs/2024-08-03_12-29.png)

## Motivation

Have you noticed how convenient and easy it is to take notes in Telegram or any other messaging app? The idea was to create a note-taking app where notes are small markdown snippets grouped in spaces. Such notes are easy to create, read, remember, and edit. In addition, there are several other particularly important features:

**_Desktop app_**. Usually, the browser has dozens of opened tabs across multiple windows. It becomes hard to find the tab with notes (even when it's pinned).

> _FYI, the app window is also hard to find when you have a dozen other windows_ :raised_eyebrow:.

Yes, you are right. This is why the second most important feature is the **_drop-down mode_**. You don't need to track the app window location/position. You just open/hide it using the global shortcut.

**_Cross platform_**. This app supports **Windows** and **Linux**. It can be compiled on macOS too, but it wasn't tested on it. So, you can try to use it on macOS but you can face more bugs.

There are many similar existing apps but all of them have one or more major flaws. You can read more about motivation and features on the app's official website: https://totes.qkation.com.

## App philosophy

Did you hear about [the _worse-is-better_ philosophy](https://www.dreamsongs.com/RiseOfWorseIsBetter.html)? If not I encourage you to read [The Rise of Worse is Better](https://www.dreamsongs.com/RiseOfWorseIsBetter.html) article.

TL;DR. This is a citation from the mentioned article above:

> The worse-is-better philosophy:
>   - Simplicity -- the design must be simple, both in implementation and interface. It is more important for the implementation to be simple than the interface.
>   - Correctness -- the design must be correct in all observable aspects. It is slightly better to be simple than correct.
>   - Consistency -- the design must not be overly inconsistent. Consistency can be sacrificed for simplicity in some cases, but it is better to drop those parts of the design that deal with less common circumstances than to introduce either implementational complexity or inconsistency.
>   - Completeness -- the design must cover as many important situations as is practical. All reasonably expected cases should be covered. Completeness can be sacrificed in favor of any other quality. Consistency can be sacrificed to achieve completeness if simplicity is retained.

:thinking: What does it mean for the app? It means that the implementation is always _straightforward_, only _common use cases_ are covered, only _wanted and valuable features_ are implemented.

If you lack any functionality or you face a bug, then report it ([issue](https://github.com/TheBestTvarynka/totes/issues/new) or [discussion](https://github.com/TheBestTvarynka/totes/discussions). Any reasonable/valuable bugs/features will be fixed/implemented!

## Installation

You can download app installer here: https://github.com/TheBestTvarynka/totes/releases. Or alternatively you can build the app from source code:

```bash
git clone https://github.com/TheBestTvarynka/totes.git
cd totes/totes
cargo tauri build
```

## Contributing

Feel free to contribute. Contributions are very welcome :blush:.

If you want to implement a missing/wanting feature or fix a bug, then create an issue/discussion and we'll guide you. We'll help you set up the environment, explain needed parts of the code, discuss implementation, and review the code.
If you don't know what to implement, then you can browse [existing issues](https://github.com/TheBestTvarynka/totes/issues?q=sort%3Aupdated-desc+is%3Aissue+is%3Aopen) or ask for help in [the discussions](https://github.com/TheBestTvarynka/totes/discussions). We always have something to do :stuck_out_tongue_winking_eye:.

## Meta

[Pavlo Myroniuk](https://github.com/TheBestTvarynka) - [the.best.tvarynka@gmail.com](mailto:the.best.tvarynka@gmail.com).

Distributed under the [MIT](https://github.com/TheBestTvarynka/crypto-helper/blob/main/LICENSE) license.
