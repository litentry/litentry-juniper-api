FROM ubuntu:16.04
RUN apt-get update
RUN apt-get install -y mysql-client
RUN apt-get install -y libmysqlclient-dev
COPY litentry-juniper-api /root/
COPY config.json /root/
COPY config.json /
ENTRYPOINT ["/root/litentry-juniper-api"]




