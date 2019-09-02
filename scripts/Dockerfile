FROM ubuntu:18.04
RUN apt-get update
RUN apt-get install mysql-client
RUN apt-get install libmysqlclient-dev
COPY litentry-juniper-api /root/
ENTRYPOINT ["/root/litentry-juniper-api"]




