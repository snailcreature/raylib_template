FROM raylib_rs_env:base_winapp AS bundle
WORKDIR /home/wineuser
USER wineuser

ENV WINEDEBUG=-all

ARG PACKAGE
ARG FULL_VERSION
ARG AUTHOR
ARG DESCRIPTION
ARG VERSION

RUN mkdir -p ./output
COPY output/ ./output
WORKDIR ./output

RUN wineboot --restart

# Initialise
RUN wine cmd <<EOT
winapp init . --verbose --no-prompt --use-defaults
EOT

# Generate the app manifest
RUN wine cmd <<EOT
winapp manifest generate . \
--package-name ${PACKAGE} \
--publisher-name "CN=${AUTHOR}" \
--entrypoint ./dist/${PACKAGE}.exe \
--description "${DESCRIPTION}" \
--version "${VERSION}" \
--template packaged \
--if-exists overwrite
EOT

# Package
RUN wine cmd <<EOT
winapp pack ./dist \
--executable $PACKAGE.exe --verbose --skip-pri \
--publisher "CN=${AUTHOR}" \
--self-contained --generate-cert --install-cert \
--output ./${PACKAGE}_${FULL_VERSION}.msix
EOT

RUN ls -a
RUN ls dist -a

RUN wineboot --shutdown --end-session
