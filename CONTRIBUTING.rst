
Contributing
===============================

Contributions are very welcome!

There are many ways to contribute:

* Reporting bugs (issues_)
* Suggesting features (issues_)
* Fixing bugs
* Implementing features
* Refactoring code
* Adding tests
* Writing documentation / examples
* Reviewing a `pull request`_
* Telling a friend :)


Licensing
-------------------------------

* Be aware that you are making your contributions available under `Apache License 2.0`_.
* A short copyright notice can be added automatically using ``gradle spotlessApply``.

Code conventions
-------------------------------

* Follow the `Rust style guide`_. A quick summary of non-automatic elements:

  - Use ``///``` line comments for documentation, ``//`` for technical notes. Place them on their own line(s) above what they document.
  - ``snake_case`` for local variables, fields, enum values, arguments/parameters and macros. ``PascalCase`` for traits, structs and enum types.
  - Do not use return statements in the last expression of a function.

* To apply some of the formatting conventions automatically, use ``cargo fmt``. This is enforced as automated test.
* Try to include tests for any non-trivial logic.
* Assertion-style checks are encouraged, with `assert!`_, not `debug_assert!`_.
* Follow these_ and other practises to avoid bugs.
* Identifiers can be long if that makes them clearer (they may be aliased locally).
* Put unit tests in a ``test`` submodule in the same file.

Pull requests
-------------------------------

* You can apply style fixes with ``gradle spotlessApply`` to save time.
* Make sure ``gradle test`` completes successfully (this is also done automatically by Travis).
* Feel free to add yourself so the [contributors.rst](contributors.rst) file.
* If you want to add new functionality, you may want to discuss in an issue before implementing it.

Git
-------------------------------

All changes must be done through pull requests. Automated tests and style checks are automatically ran for pull requests.

* Try not to make commits too large.
* The title of the commit message should summarize why the change is made.
* The title of the commit message should be imperative (be able to follow 'this change will...').
* If the commit message contains body, it should be separated from the title by a newline.
* Titles should start with a capital, omit the period, and be at most 72 characters (preferably 50).

Versioning
-------------------------------

* Semantic versioning is used:

  - Major version increases for backward-incompatible changes.
  - Minor version increases for new but (mostly) compatible functionality.
  - Patch versions increase for bug fixes.

* The compiler version is not the language version.

Support for older versions
-------------------------------

* Development happens relative to the ``dev`` branch (through pull requests). All features and most bugfixes should be pull requests against the ``dev`` branch.
* A branch ``vx.y`` is created for each major and minor version for back-porting fixes for a limited time.
* If a version ``x.y.6`` is released, that means ``x.y.0`` to ``x.y.5`` are deprecated and updating is recommended. Use ``x.y.*`` (or ``x.*``) as requirement.
* A timeline for how long old major and minor versions are supported will be added later.

Security
-------------------------------

If you feel there is a security issue, please `contact me`_ privately.


.. _`contact me`: https://markv.nl/about
.. _issues: https://github.com/mangolang/compiler/issues
.. _`pull request`: https://github.com/mangolang/compiler/pulls
.. _`Apache License 2.0`: https://github.com/mangolang/compiler/blob/master/LICENSE.txt
.. _`Be nice`: https://github.com/mangolang/compiler/blob/master/CODE_OF_CONDUCT.rst
.. _`Rust style guide`: https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/guide/guide.md
.. _`assert!: https://doc.rust-lang.org/1.22.1/std/macro.assert.html
.. _`debug_assert!: https://doc.rust-lang.org/1.22.1/std/macro.debug_assert.html
.. _these: http://hshno.de/HJhvEnVDz
