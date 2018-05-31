#!/usr/bin/env python3
# -*- coding: UTF-8 -*-

"""
Run a shell command on the staged changes only.
"""

from sys import stderr, argv
from util_cmd import git, run


def run_on_staged(cmds):
	"""
	Temporarily remove all unstaged changed, so that any checks are run on what is actually to be committed.
	
	Note that `diff` cannot include untracked files, and `stash` has trouble retrieving by name, but was chosen anyway.
	"""
	def do_cmds(cmds):
		for cmd in cmds:
			try:
				run(cmd, allow_stderr=True, log=True)
			except Exception as err:
				stderr.write(str(err))
				stderr.write('\nFAILED, cancelling commit\n')
				return 1
		return 0
	
	# See if there are any staged changes
	staged = git('diff --staged --name-only', errmsg='Could not get the staged files').strip()
	if not staged:
		# Skip checks and let git deal with empty commit attempt
		return 0
	# See if there are any UNstaged changes
	# if there are, we need to exclude those from tests
	unstaged = git('diff --name-only', errmsg='Could not get the staged files').strip()
	if not unstaged:
		# Skip all the setup and just run the tests
		return do_cmds(cmds)
	# If we get here, there were unstaged changes to hide
	branchname = git('rev-parse --abbrev-ref HEAD', errmsg='Could not get the current branch name').strip()
	assert branchname != '' and branchname != 'HEAD', 'could not find branch (maybe detached head?)'
	git('-c commit.gpgsign=false commit --no-verify -m "*** This is a temporary commit to run hooks; '
		'it contains STAGED changed; you should not see it, revert it if you do ***"',
		errmsg='Could not create temporary commit for planned changes')
	stage_hash = git('rev-parse --verify HEAD', errmsg='Could not get the temporary commit hash (staged)').strip()
	try:
		# Now commit all the unstaged changes in a separate commit
		git('add :/ --all', errmsg='Could not stage unstaged changes for temporary commit')
		git('-c commit.gpgsign=false commit --no-verify -m "*** This is a temporary commit to run hooks; '
			'it contains UNSTAGED changed; you should not see it, revert it if you do ***"',
			errmsg='Could not create temporary commit for unstaged changes')
		unstage_hash = git('rev-parse --verify HEAD', errmsg='Could not get the temporary commit hash (staged)').strip()
		try:
			# There is one commit with staged changes and one with unstaged ones
			git('checkout "{}"'.format(stage_hash), allow_stderr=True, errmsg='Cannot check out "staged files" temporary commit')
			try:
				return_code = do_cmds(cmds)
			finally:
				git('checkout "{}"'.format(branchname), allow_stderr=True, errmsg='Cannot go back to HEAD after checkout of "staged files" temporary commit')
		finally:
			# Go back to before the 'unstaged' commit
			git('reset --soft "{}"~1'.format(unstage_hash), errmsg='Could not undo temporary commit (ustaged)')
			git('reset', errmsg='Could not unstage changed from reverted commit')  # unstage
	finally:
		# Go back to before the 'staged' commit
		git('reset --soft "{}"~1'.format(stage_hash), errmsg='Could not undo temporary commit (staged)')
	return return_code


assert len(argv) >= 2, 'Did not get enough argument for pre-commit test hook'
exit(run_on_staged(argv[1:]))
