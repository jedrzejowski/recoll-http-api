FROM ubuntu:22.04 as base

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get install -y \
        file \
        wget \
        libxapian30 \
        libxslt1.1 \
        zlib1g \
        python3-waitress \
        python3-mutagen \
        python3-lxml \
        untex \
        aspell aspell-en aspell-pl \
        libimage-exiftool-perl \
        poppler-utils antiword unrtf xsltproc \
    && \
    apt-get clean


FROM base as recoll-builder

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get install -y \
        wget pkg-config build-essential \
        libxapian-dev xapian-tools \
        libxslt1-dev \
        zlib1g-dev \
        libpython3-dev python3-setuptools

WORKDIR /app

RUN wget https://www.lesbonscomptes.com/recoll/recoll-1.32.5.tar.gz
RUN tar -xvzf recoll-1.32.5.tar.gz

WORKDIR /app/recoll-1.32.5

RUN ./configure \
        --disable-qtgui \
        --disable-userdoc \
        --disable-x11mon \
        --disable-python-chm \
        --enable-recollq

RUN make
RUN make install

FROM base as rust-builder

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

FROM base

ENV PYTHONPATH="${PYTHONPATH}:/usr/local/lib/python3/dist-packages"
COPY --from=recoll-builder /usr/local /usr/local

