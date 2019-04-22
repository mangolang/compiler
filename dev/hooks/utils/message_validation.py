#!/usr/bin/env python3
# -*- coding: UTF-8 -*-

"""
This git commit message hook validates that the commit message conforms to the conventional style:

* Title should start with a cap and not have a period.
* Title up to 70 characters excluding issue reference.
* Title should end with a reference to an issue, or #so for standalone.
* If there is a body, it should follow after a newline.
* Body should use normal interpunction (caps, periods).

https://chris.beams.io/posts/git-commit/
"""

import string
from re import compile
from sys import argv, stderr


class ValidationError(Exception):
	def __init__(self, msg):
		self.message = msg


def validate_title(title):
	if not title:
		raise ValidationError('Commit message should have at least one line (the title), with any hash(#) appearing at the end')
	if title[0] in string.ascii_lowercase:
		raise ValidationError('Commit message title should start with a capital letter')
	if title[-1] == '.':
		raise ValidationError('Commit message title should not end with a period')
	if len(title) < 8:
		raise ValidationError('Commit message title too short ({}<8), please describe what this solves'.format(len(title)))
	if len(title) > 70:
		raise ValidationError('Commit message title too long ({}>70), use description body to provide details'.format(len(title)))


def validate_body(body):
	if not body:
		return 0
	if body[0] in string.ascii_lowercase:
		raise ValidationError('Commit message body should start with a capital letter')
	if body[-1] not in '!?.':
		raise ValidationError('Commit message body should end with a period or other punctuation symbol')
	for line in body.splitlines():
		if len(line) > 70:
			# This is not a fatal error
			stderr.write(('Commit message description line longer than 70 characters found ({}); this is okay if it is code, '
				'but should not happen otherwise:\n{}').format(len(line), line))


def validate_tags(tags):
	if '#so' in tags:
		if len(tags) == 1:
			return
		raise ValidationError('For standalone issues (marked "#so"), there should not be any other references (found {})'
			.format(' '.join(tag for tag in tags if tag != '#so')))
	regexp = compile(r'^#\d+$')
	for tag in tags:
		if not regexp.match(tag):
			raise ValidationError(('Commit message title reference "{}" is invalid; it should either be a number like "#42", '
				'or "#so" for standalone issues').format(tag))


def validate_commit_message(msg):
	if msg[:5].lower() == 'merge':
		return
	title, tags, body = split_msg(msg)
	validate_title(title)
	validate_tags(tags)
	validate_body(body)


def read_msg(pth):
	with open(pth, 'r') as fh:
		return fh.read()


def split_msg(txt):
	if not txt.strip():
		raise ValidationError('Empty commit message')
	lines = txt.splitlines()
	if len(lines) > 1:
		if lines[1]:
			raise ValidationError('Second line of the commit message should be empty, to separate the title from the body')
	body = '\n'.join(lines[2:])
	parts = lines[0].split('#', maxsplit=1)
	if len(parts) < 2:
		raise ValidationError('Did not find an issue link starting with # at the end of the commit message title (use #so for standalone issues)\n')
	title = parts[0].strip()
	tags = ('#' + parts[1]).split()
	return title, tags, body


assert len(argv) >= 2, 'Did not get enough argument for commit message validation hook'
msg = read_msg(argv[1])
try:
	validate_commit_message(msg)
	exit(0)
except ValidationError as ex:
	stderr.write(ex.message)
	stderr.write('\nCommit message validation failed; commit aborted\n')
	exit(1)
