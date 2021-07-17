# docker build script based on https://docs.docker.com/samples/dotnetcore/

# dotnet 5.0 sdk found on https://github.com/dotnet/dotnet-docker/issues/2375
FROM mcr.microsoft.com/dotnet/sdk:5.0 AS build-env
WORKDIR /app

# Copy csproj and restore as distinct layers
COPY AppointmentScheduling/*.csproj ./
RUN dotnet restore

# Copy everything else and build
COPY AppointmentScheduling/ ./
RUN dotnet publish -c Release -o out

# Build runtime image
FROM mcr.microsoft.com/dotnet/aspnet:5.0 AS run-env
WORKDIR /app
COPY --from=build-env /app/out .
ENTRYPOINT ["dotnet", "aspnetapp.dll"]
