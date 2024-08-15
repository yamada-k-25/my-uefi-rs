FROM ubuntu:latest
RUN apt-get update
RUN apt-get install -y build-essential
RUN apt-get install -y qemu-system ovmf rustup
RUN rustup default stable
RUN mkdir /src && cd /src
CMD [ "sleep", "infinity" ]