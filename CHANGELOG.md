# Change Log

## [0.19.2] - 2016-10-31

* Update handlebars to 0.22 for better error reporting

## [0.17.0] - 2016-07-27

* Update iron to 0.4

## [0.16.0] - 2016-07-27

* Update handlebars to 0.19

## [0.15.3] - 2016-06-25

### Changed

* Update handlebars to 0.18

## [0.15.2] - 2016-05-21

### Changed

* Improved performance for directory watcher. [#45]

## [0.15.1] - 2016-04-13

### Changed

* Fixed template loading on Windows. [#42]

## [0.15.0] - 2016-04-01

### Added

* Template from `catch` branch will be rendered too. [#40]

### Changed

* Handlebars data will be removed from Iron request extension map when
  we finished rendering.

## [0.14.0] - 2016-03-25

### Changed

* `HandlebarsEngine::new2` and `HandlebarsEngine::from2` are now `new`
  and `from`.
* Updated to iron 0.3.x

### Removed

* Previous `HandlebarsEngine::new` and `HandlebarsEngine::from` were
  removed.

## [0.13.1] - 2016-03-21

### Added

* `Template::with` to render some template string without having it
  from any source
