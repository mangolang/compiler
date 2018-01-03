
Mango
===============================

Mango is a programming language that is still in currently development.

https://mangolang.org/

How to use
-------------------------------

The project requires Gradle and Kotlin (multi-platform). To compile, test and run on JVM:

    gradle -p mango-jvm test run

To run the Javascript version with Node (including tests), first do `npm install`. Then:

    gradle -p mango-js run

To show it in a web-browser (after tests), run

    gradle -p mango-js web

then open `mango-js/build/web/home.html` in a browser.


