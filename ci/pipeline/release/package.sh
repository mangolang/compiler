#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

# Attempt to fix file permissions.
# https://stackoverflow.com/a/29584184
CHECK_NIGHTLY bash -c 'chown -R `stat -c "%u:%g" /release` /release'

# Delete old releases.
CHECK_NIGHTLY rm -f "/release/"*".zip"

# Stop if no files.
# shellcheck disable=SC2010
if ! ls -1qA "${RELEASE_PATH}" | grep -q . ; then
    printf 'did not find any files to release in "%s"\n' "${RELEASE_PATH}"  1>&2
    exit 1
fi

# Zip the files.
CHECK_NIGHTLY bash -c '\
chown -R `stat -c "%u:%g" /release` /release;\
cd /release;\
ls -als;\
zip -r "'${RELEASE_NAME}'.zip" .'

# Move the release zip archive.
cp -r "${RELEASE_PATH}/${RELEASE_NAME}.zip" "${RELEASE_PATH}/../${RELEASE_NAME}.zip" && \
    rm "${RELEASE_PATH}/${RELEASE_NAME}.zip"
