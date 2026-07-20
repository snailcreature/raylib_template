FROM raylib_rs_env:base_winapp AS bundle
WORKDIR /home/wineuser
USER wineuser

ARG PACKAGE
ARG FULL_VERSION

RUN mkdir -p ./output
COPY output/ ./output
WORKDIR ./output

RUN wine cmd <<EOT
/winappcli/winapp init . --verbose --no-prompt
EOT

RUN wine cmd <<EOT
/winappcli/winapp pack ./dist --executable $PACKAGE.exe --verbose --no-prompt --skip-pri --output ./${PACKAGE}_${FULL_VERSION}.msix
EOT

RUN ls -a
RUN ls dist -a
