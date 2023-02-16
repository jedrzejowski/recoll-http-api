FROM ubuntu:22.04 as base

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get install -y \
        file wget curl \
        libxapian30 \
        libxslt1.1 \
        zlib1g \
        firejail \
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

FROM rust:buster as rust-builder

WORKDIR /app

COPY Cargo.* /app/
RUN cargo fetch

COPY . /app
RUN cargo build --release

FROM base

ENV PYTHONPATH="${PYTHONPATH}:/usr/local/lib/python3/dist-packages"

ENV FINDEX_BIN_RECOLLINDEX=/usr/local/bin/recollindex
ENV FINDEX_BIN_RECOLLQ=/usr/local/bin/recollq
ENV FINDEX_BIN_FIREJAIL=/usr/bin/firejail

COPY --from=recoll-builder /usr/local /usr/local
COPY --from=rust-builder /app/target/release/findex-serve  /usr/local/bin/findex-serve
COPY --from=rust-builder /app/target/release/findex-update /usr/local/bin/findex-update

CMD [ "/usr/local/bin/findex-serve" ]
