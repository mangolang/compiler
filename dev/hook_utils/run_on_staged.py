"""
Run a shell command on the staged changes only.
"""

from sys import stderr, argv
from time import time
from util_cmd import git, run


def run_on_staged(cmd):
	"""
	Temporarily remove all unstaged changed, so that any checks are run on what is actually to be committed.
	
	Note that `diff` cannot include untracked files, and `stash` has trouble retrieving by name, but was chosen anyway.
	"""
	name = 'before_hook_check_{0:d}s'.format(int(time()))
	git("-c commit.gpgsign=false stash save --include-untracked --keep-index '{0:s}'".format(name))
	try:
		try:
			run(cmd, print=True)
		except Exception as err:
			stderr.write(err)
			return 1
	finally:
		# git("stash apply 'stash^{{/{0:s}}}'".format(name))
		git("stash pop")
	return 0  # ok


if __name__ == '__main__':
	assert len(argv) >= 2
	exit(run_on_staged(argv[1]))
