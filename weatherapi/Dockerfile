# docker build script based on https://docs.docker.com/samples/dotnetcore/

# dotnet 5.0 sdk found on https://github.com/dotnet/dotnet-docker/issues/2375
FROM mcr.microsoft.com/dotnet/sdk:5.0 AS build-env
WORKDIR /app

# Copy csproj and restore as distinct layers
COPY *.csproj ./
# RUN dotnet dev-certs https
# RUN dotnet dev-certs https --trust
RUN dotnet restore

# Copy everything else and build
COPY ./ ./
RUN dotnet publish -c Release -o out
# RUN dotnet dev-certs https -ep /usr/local/share/ca-certificates/aspnet/https.crt --format PEM
# RUN update-ca-certificates
# RUN mkdir -p /usr/local/firefox/distribution
# RUN echo '{"policies": {"Certificates": {"Install": ["/usr/local/share/ca-certificates/aspnet/https.crt"]}}}' > /usr/local/firefox/distribution/policies.json
# RUN ls -la /usr/local/share/ca-certificates/aspnet
# RUN ls -la /usr/local/firefox/distribution
# RUN cat /usr/local/share/ca-certificates/aspnet/https.crt
# RUN ls -la /etc/ca-certificates

# Build runtime image
FROM mcr.microsoft.com/dotnet/aspnet:5.0 AS run-env
WORKDIR /app
COPY --from=build-env /app/out .
# COPY --from=build-env /usr/local/share/ca-certificates/aspnet /root/.aspnet/https/
ENTRYPOINT ["dotnet", "weatherapi.dll"]
