from subprocess import Popen, PIPE
# noinspection PyUnresolvedReferences
from sys import stdout, stderr
from os.path import isdir


class CmdError(Exception): pass


def run(cmd, *, allow_stderr=False, log=False, errmsg=None):
	"""
	Run a shell command and return the output.
	"""
	if log:
		stdout.write('{0:s}\n'.format(cmd))
	proc = Popen('{0:s}'.format(cmd), shell=True, stdout=PIPE, stderr=PIPE)
	try:
		out, err = proc.communicate()
	except Exception as ex:
		raise ex.__class__("{}\ncommand: {}\nexception message: {}".format(
			errmsg or "command failed",
			cmd,
			str(ex),
		))
	stdres = (out + b'\n' + err).decode('utf-8')
	if proc.returncode:
		stdout.write(stdres)
		raise CmdError('stopped because of return code {0:d}\ncommand: {1:s}'.format(
			proc.returncode,
			cmd,
		))
	if allow_stderr:
		return stdres
	if err.strip():
		stdout.write(stdres)
		err = err.decode('utf-8')
		raise CmdError('{0:s}\ncommand: {1:s}\nstderr message: {2:s}'.format(
			errmsg or "command failed",
			cmd,
			err,
		))
	return out.decode('utf-8')


def git(cmd, *, allow_stderr=False, errmsg=None):
	"""
	Run a git command and return the output.
	"""
	return run('git {0:s}'.format(cmd), allow_stderr=allow_stderr, errmsg=errmsg)


def get_root():
	"""
	Get the root path of the project (cached).
	"""
	if not getattr(get_root, 'root_cache', None):
		get_root.root_cache = git("rev-parse --show-toplevel").strip()
		assert isdir(get_root.root_cache), 'not found: {:s}'.format(get_root.root_cache)
	return get_root.root_cache
