FROM postgres:latest

ENV POSTGRES_USER=root
ENV POSTGRES_PASSWORD=password
ENV POSTGRES_DB=state

EXPOSE 5432

CMD ["postgres"]
