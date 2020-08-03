
# This image builds the Mango CLI in a slim image.
# This is the image to interact with as a user of Mango.
# https://hub.docker.com/r/mangocode/mango

FROM mango_ci:stable AS build

# Probably still up-to-date, just just in case.
RUN cargo build --bin mango --release

# A find is needed here for it to work with multiple platforms (musl uses different path)
RUN find . -wholename '*/release/*' -name 'mango' -type f -executable -print -exec cp {} /mango/mango_exe \;

RUN ls -als /mango/mango_exe

# Second stage image to decrease size
# Note: this version should match `base.Dockerfile`
FROM scratch

ENV RUST_BACKTRACE=1

WORKDIR /

# It's really just the executable; other files are part of the Github release, but not Docker image.
#COPY README.rst LICENSE.txt ./
COPY --from=build /mango/mango_exe /mango

#TODO @mark: maybe printf does not work in 'scratch'
#CMD printf "Welcome to the Mango docker image!\nTo use, add 'mango' after your docker run command\n"
ENTRYPOINT ["/mango"]
