#!/usr/bin/env python3
# -*- coding: UTF-8 -*-

"""
This git hooks makes sure a tag is created each time the version is increased.

This version works with version for `Cargo.toml`, `setup.py` and `settings.gradle`, but can be adapted.

https://gist.github.com/mverleg/9988b44d57b63b0eef40dd9ce7b48e45
"""
import traceback
from re import findall
from sys import stdout
from os.path import join

from util_cmd import git, get_root


def get_existing_versions():
	"""
	Get all versions for which there is a tag.
	"""
	return set(tag.strip().lstrip('v') for tag in
			git('tag --list --format "%(refname:short)"').splitlines())


def get_setup_py_version():
	try:
		with open(join(get_root(), 'setup.py'), 'r') as fh:
			res = findall(r'\n[^#\n]*version\s*=\s*[\'"]([\w.-_]+)[\'"]', fh.read())
		assert len(res) != 0, 'Did not find version inside setup.py'
		assert len(res) < 2, 'Found multiple versions inside setup.py'
		return res[0].strip().lstrip('v')
	except IOError:
		return None


def get_settings_gradle_version():
	try:
		with open(join(get_root(), 'settings.gradle'), 'r') as fh:
			""" Only single-quotes supported, because double quotes may contain variables. """
			res = findall(r'\n\s*version\s*=\s*\'([\w.-_]+)\'', fh.read())
		assert len(res) != 0, 'Did not find version inside settings.gradle'
		assert len(res) < 2, 'Found multiple versions inside settings.gradle'
		return res[0].strip().lstrip('v')
	except IOError:
		return None


def get_cargo_toml_version():
	pth = join(get_root(), 'Cargo.toml')
	try:
		with open(pth, 'r') as fh:
			res = findall(r'\n[^#\n]*version\s*=\s*[\'"]([\w.-_]+)[\'"]', fh.read())
		assert len(res) != 0, 'Did not find version inside {:s}'.format(pth)
		assert len(res) < 2, 'Found multiple versions inside {:s}'.format(pth)
		return res[0].strip().lstrip('v')
	except IOError:
		return None


def get_current_version():
	"""
	Get the current version of the project, according to package file.
	"""
	return get_cargo_toml_version() or get_setup_py_version() or get_settings_gradle_version() or None


def version_tag():
	"""
	Create a tag for the current version if there isn't one yet.
	"""
	version = get_current_version()
	if version is None:
		""" No version found. """
		return
	versions = get_existing_versions()
	if version in versions:
		""" Version already tagged. """
		return
	stdout.write('\nCREATING TAG v{0:} (use `git tag --delete v{0:}` if not desired)\n\n'.format(version))
	git('tag v{0:}'.format(version))


try:
	exit(version_tag())
except Exception as err:
	traceback.print_exc()
exit(1)
