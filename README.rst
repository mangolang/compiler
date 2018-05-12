
Mango
===============================

Mango is a programming language that is currently in development. It focuses on maintainability and bug prevention.

https://mangolang.org/

Status
-------------------------------

This project is still in early development stage. It is not ready to use, not even experimentally.

There are dozens of pages of design notes, but the plan still lacks coherence, so is unpublished.

How to use
-------------------------------

The compiler is not complete yet and cannot be used for real code.

But if you want to play with it, or even help to complete it, here is how!

These instructions were tested on Ubuntu 18.4 (using Bash). It should also work on other platforms, if you 'translate' the commands.

* Get the code with (something like). 

.. code:: bash

	git clone https://github.com/mangolang/compiler.git mango
	cd mango  # choose the directory that you downloaded the code to
	# After this, I'll assume $PWD is the location of `mango`.

* The compiler is written in Rust. If you don't have `rustup` yet, install it with instructions `on this page`_. Then

.. code:: bash

	rustup toolchain install nightly
	rustup override set nightly  # make sure you are in the mango directory
	
* We need a few packages:

.. code:: bash

	rustup component add rustfmt-preview
	

* There are git commit hooks that you can use. They test that code is formatted well and compiles and commit messages are correctly formatted. You don't have to use them if you ensure these things yourself. If you want to use them:

.. code:: bash

	git config core.hooksPath $PWD/dev/hooks/
	# on git versions older than 2.9, use (from mango directory):
	rm -rf .git/hooks
	ln -s $PWD/dev/hooks/ .git/hooks
	chmod u+x .git/hooks

* To format and test the code, run

.. code:: bash

	cargo +nightly fmt
	cargo test --all

* You're now ready to make changes! If you want to help, you're very welcome! Have a glance at CONTRIBUTING.rst_ if you have a minute.

* For IDEs, Rust support isn't on the level of e.g. Java yet, but JetBrain's CLion_ with `Rust plugin`_ is not bad if you have a license.

.. _CLion: https://www.jetbrains.com/clion/
.. _`Rust plugin`: https://intellij-rust.github.io/
.. _`on this page`: https://www.rust-lang.org/en-US/install.html
.. _CONTRIBUTING.rst: 
