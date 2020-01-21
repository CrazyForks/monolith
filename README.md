[![Travis CI Build Status](https://travis-ci.org/Y2Z/monolith.svg?branch=master)](https://travis-ci.org/Y2Z/monolith)
[![AppVeyor Build status](https://ci.appveyor.com/api/projects/status/ae7soyjih8jg2bv7/branch/master?svg=true)](https://ci.appveyor.com/project/snshn/monolith/branch/master)

```
 ___     ___________    __________      ___________________    ___
|   \   /           \  |          |    |                   |  |   |
|    \_/     __      \_|    __    |    |    ___     ___    |__|   |
|           |  |           |  |   |    |   |   |   |   |          |
|           |__|    _      |__|   |____|   |   |   |   |    __    |
|   |\_/|          | \                     |   |   |   |   |  |   |
|___|   |__________|  \____________________|   |___|   |___|  |___|
```

A data hoarder's dream come true: bundle any web page into a single HTML file. You can finally replace that gazillion of open tabs with a gazillion of .html files stored somewhere on your precious little drive.

Unlike the conventional "Save page as", `monolith` not only saves the target document, it embeds CSS, image, and JavaScript assets **all at once**, producing a single HTML5 document that is a joy to store and share.

If compared to saving websites with `wget -mpk`, this tool embeds all assets as data URLs and therefore lets browsers render the saved page exactly the way it was on the Internet, even when no network connection is available.

## Installation

### From source
    $ git clone https://github.com/Y2Z/monolith.git
    $ cd monolith
    $ cargo install --path .

### With Homebrew (on macOS and GNU/Linux)
    $ brew install monolith

### Using Snapcraft (on GNU/Linux)
    $ snap install monolith

## Usage
    $ monolith https://lyrics.github.io/db/P/Portishead/Dummy/Roads/ -o portishead-roads-lyrics.html

## Options
 - `-c`: Ignore styles
 - `-f`: Exclude iframes
 - `-i`: Remove images
 - `-I`: Isolate document
 - `-j`: Exclude JavaScript
 - `-k`: Accept invalid X.509 (TLS) certificates
 - `-o`: Write output to file
 - `-s`: Silent mode
 - `-u`: Specify custom User-Agent

## HTTPS and HTTP proxies
Please set `https_proxy`, `http_proxy` and `no_proxy` environment variables.

## Contributing
Please open an issue if something is wrong, that helps make this project better.

## Related projects
 - `Monolith Chrome Extension`: https://github.com/rhysd/monolith-of-web
 - `Pagesaver`: https://github.com/distributed-mind/pagesaver
 - `Personal WayBack Machine`: https://github.com/popey/pwbm
 - `SingleFile`: https://github.com/gildas-lormeau/SingleFile

## License
The Unlicense

<!-- Microtext -->
<sub>Keep in mind that `monolith` is not aware of your browser's session</sub>
