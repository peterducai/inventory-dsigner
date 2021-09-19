# building container 
FROM registry.fedoraproject.org/fedora-minimal AS build 
RUN mkdir /invdsigner && microdnf install git rust cargo -y && microdnf clean all -y
WORKDIR /invdsigner 
COPY . /invdsigner
RUN cd /invdsigner && cargo build

FROM registry.fedoraproject.org/fedora-minimal 
WORKDIR / 
COPY static /static
RUN ls -la / && ls -la /usr/local/bin 
COPY --from=build /invdsigner/target/debug/invdsigner /usr/local/bin 
# COPY --from=build /go/server.* /
CMD ["invdsigner"] 
EXPOSE 7878