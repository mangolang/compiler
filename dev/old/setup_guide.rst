How to use
-------------------------------

The compiler is not complete yet and cannot be used for real code.

But if you want to play with it, or even help to complete it, here is how!

These instructions were tested on Ubuntu 18.4 (using Bash). It should also work on other platforms, if you 'translate' the commands.

* You will need `pkg-config`, `libssl-dev`, `git` and some way to run commands. Python is optional.

* Get the code with (something like).

.. code:: bash

	git clone https://github.com/mangolang/compiler.git mango
	cd mango  # choose the directory that you downloaded the code to
	# After this, I'll assume $PWD is the location of `mango`.

* The compiler is written in Rust. If you don't have `rustup` yet, install it with instructions `on this page`_. Then

.. code:: bash

	rustup toolchain install nightly
	rustup override set nightly  # make sure you are in the mango directory
	rustup target add wasm32-unknown-unknown --toolchain nightly

* We need a few packages:

.. code:: bash

	rustup component add rustfmt-preview
    cargo install wasm-bindgen-cli

* There are git commit hooks that you can use. They test that code is formatted well and compiles and commit messages are correctly formatted. You don't have to use them if you ensure these things yourself. If you want to use them:

.. code:: bash

	git config core.hooksPath $PWD/dev/hooks/
	# on git versions older than 2.9, use (from mango directory):
	rm -rf .git/hooks
	ln -s $PWD/dev/hooks/ .git/hooks
	chmod u+x .git/hooks

* The first build will be slow due to downloading and compiling dependencies. To format the code, test it, and launch the command line interface:

.. code:: bash

	cargo +nightly fmt
	cargo test --all
	cargo run --bin mango-cli

  or to build a fast, release-mode native binary:

.. code:: bash

    RUSTFLAGS="-C target-cpu=native" cargo build --release

* To deploy the web version in release mode, run the script `dev/build_web.sh` (or view it for the steps needed). It uses Python's SimpleHTTPServer, if you don't have that, you can still find the deployable code in `target/deploy`.

* You're now ready to make changes! If you want to help, you're very welcome! Have a glance at CONTRIBUTING.rst_ if you have a minute.

* For IDEs, Rust support isn't on the level of e.g. Java yet, but JetBrain's CLion_ with `Rust plugin`_ is not bad if you have a license.

Contributing
-------------------------------

The Mango compiler is `Apache 2.0 licensed`_. For notes on how to contribute, see `contributing`_. Please `behave`_.

.. _`Apache 2.0 licensed`: https://github.com/mangolang/mango/blob/master/LICENSE.rst
.. _`contributing`: https://github.com/mangolang/mango/blob/master/CONTRIBUTING.rst
.. _`behave`: https://github.com/mangolang/mango/blob/master/CODE_OF_CONDUCT.rst

.. _CLion: https://www.jetbrains.com/clion/
.. _`Rust plugin`: https://intellij-rust.github.io/
.. _`on this page`: https://www.rust-lang.org/en-US/install.html
.. _CONTRIBUTING.rst: https://github.com/mangolang/compiler/blob/dev/CONTRIBUTING.rst
