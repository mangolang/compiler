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
				print(run(cmd, log=True)) # TODO
				run(cmd, log=True)
			except Exception as err:
				stderr.write(err)
				return 1
		return 0
	
	tagname = 'hiding_latest_local_before_hook'
	# See if there are any staged changes
	staged = git('diff --name-only', errmsg='Could not get the staged files').strip()
	if not staged:
		print('skipping (no changes)')  # TODO: TMP
		# Skip checks and let git deal with empty commit attempt
		return 0
	# See if there are any UNstaged changes
	# if there are, we need to exclude those from tests
	unstaged = git('diff --cached --name-only', errmsg='Could not get the staged files').strip()
	if not unstaged:
		print('running tests directly')  # TODO: TMP
		# Skip all the setup and just run the tests
		return do_cmds(cmds)
	# If we get here, there were unstaged changes to hide
	# Clean up temporary tag if it already exists
	if git('tag -l "{}"'.format(tagname)).strip():
		stderr.write('Temporary tag still exists, cleaning up "{}"\n'.format(tagname))
		git('tag -d "{}"'.format(tagname), errmsg='Could not clean up temporary tag before starting')
	git('-c commit.gpgsign=false tag "{}"'.format(tagname),
		errmsg='Failed to create temporary tag')
	try:
		# We have a tag to go back to; we can create commits: create one for staged changes
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
				return_code = do_cmds(cmds)
				git('checkout master', allow_stderr=True, errmsg='Cannot go back to master after checkout of "staged files" temporary commit')
			finally:
				# Go back to before the 'unstaged' commit
				git('reset --soft "{}"~1'.format(unstage_hash), errmsg='Could not undo temporary commit (ustaged)')
				git('reset', errmsg='Could not unstage changed from reverted commit')  # unstage
		finally:
			# Go back to before the 'staged' commit
			git('reset --soft "{}"~1'.format(stage_hash), errmsg='Could not undo temporary commit (staged)')
	finally:
		# Delete the temporary tag
		# TODO: is this really useful? I never rever to this tag (yet)
		git('tag -d "{}"'.format(tagname), errmsg='Can not clean up temporary tag')
	print('reached end with {}'.format(return_code))  # TODO: TMP
	return 1
	return return_code
	
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
	exit(run_on_staged(argv[1:]))
