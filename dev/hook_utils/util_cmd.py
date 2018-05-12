from subprocess import Popen, PIPE
# noinspection PyUnresolvedReferences
from sys import stdout, stderr

from os.path import isdir


def run(cmd, *, allow_err=False, print=False):
	"""
	Run a shell command and return the output.
	"""
	if print:
		stdout.write('{0:s}\n'.format(cmd))
	out, err = Popen('{0:s}'.format(cmd), shell=True, stdout=PIPE, stderr=PIPE).communicate()
	if allow_err:
		return (out + '\n' + err).decode('utf-8')
	if err.strip():
		msg = 'an error occurred while running:\n{0:s}\n{1:s}'.format(cmd, err.decode('utf-8'))
		raise AssertionError(msg)
	return out.decode('utf-8')


def git(cmd, *, allow_err=False):
	"""
	Run a git command and return the output.
	"""
	return run('git {0:s}'.format(cmd), allow_err=allow_err)


def get_root():
	"""
	Get the root path of the project (cached).
	"""
	if not getattr(get_root, 'root_cache', None):
		get_root.root_cache = git("rev-parse --show-toplevel").strip()
		assert isdir(get_root.root_cache), 'not found: {:s}'.format(get_root.root_cache)
	return get_root.root_cache
