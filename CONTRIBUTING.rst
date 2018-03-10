
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

* Try to follow the `Kotlin coding convention`_.
* Some of these and some other conventions are enforced by Detekt_ (config_) and KtLint_ as part of gradle build.
* Try to include tests for any non-trivial code.
* Assertion-style checks are encouraged, but should use ``require``.

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

Security
-------------------------------

If you feel there is a security issue, please `contact me`_ privately.


.. _`contact me`: https://markv.nl/about
.. _issues: https://github.com/mangolang/compiler/issues
.. _`pull request`: https://github.com/mangolang/compiler/pulls
.. _`Apache License 2.0`: https://github.com/mangolang/compiler/blob/master/LICENSE.txt
.. _`Be nice`: https://github.com/mangolang/compiler/blob/master/CODE_OF_CONDUCT.rst
.. _`Kotlin coding convention`: https://kotlinlang.org/docs/reference/coding-conventions.html
.. _`Detekt`: https://github.com/arturbosch/detekt
.. _`KtLint`: https://github.com/shyiko/ktlint
.. _`config`: https://github.com/mangolang/compiler/blob/master/detekt.yml
