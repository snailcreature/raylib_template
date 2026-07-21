FROM raylib_rs_env:base_winapp AS bundle
WORKDIR /home/wineuser
USER wineuser

ENV WINEDEBUG=-all

ARG PACKAGE
ARG FULL_VERSION
ARG AUTHOR
ARG DESCRIPTION
ARG VERSION

RUN wine cmd <<EOT
/powershell/pwsh -c dotnet dev-certs https --trust
EOT

RUN mkdir -p ./output
COPY output/ ./output
WORKDIR ./output

RUN wine cmd <<EOT
/winappcli/winapp init . --verbose --no-prompt
EOT

RUN wine cmd <<EOT
/winappcli/winapp manifest generate . \
--package-name ${PACKAGE} \
--publisher-name "CN=${AUTHOR}" \
--entrypoint ./dist/${PACKAGE}.exe \
--description "${DESCRIPTION}" \
--version "${VERSION}" \
--template packaged \
--if-exists overwrite
EOT

RUN wine cmd <<EOT
/winappcli/winapp pack ./dist \
--executable $PACKAGE.exe --verbose --skip-pri \
--name "${PACKAGE}" --publisher "${AUTHOR}" \
--self-contained \
--output ./${PACKAGE}_${FULL_VERSION}.msix
EOT

RUN ls -a
RUN ls dist -a
