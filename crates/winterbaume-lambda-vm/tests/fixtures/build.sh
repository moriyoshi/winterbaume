#!/bin/sh
# Rebuilds the compiled handler fixtures of vm_test.rs, in containers.
#   java/Hello.class        from java/Hello.java   ( class file version of Java 8 )
#   dotnet/hello-net8.zip   from dotnet/*          ( for dotnet8 )
#   dotnet/hello-net10.zip  from dotnet/*          ( for dotnet10: the runtime
#                                                     starts with the function's
#                                                     runtimeconfig.json )
set -eu
cd "$(dirname "$0")"
user="$(id -u):$(id -g)"
docker run --rm --user "$user" -v "$PWD/java:/src" -w /src amazoncorretto:21 javac --release 8 Hello.java
for sdk in 8 10; do
  work="$(mktemp -d)"
  cp dotnet/Function.cs dotnet/Hello.csproj "$work/"
  sed -i "s|<TargetFramework>[^<]*</TargetFramework>|<TargetFramework>net$sdk.0</TargetFramework>|" "$work/Hello.csproj"
  docker run --rm -v "$work:/src" -w /src "mcr.microsoft.com/dotnet/sdk:$sdk.0" \
    sh -c "dotnet publish -c Release -o out --nologo; status=\$?; chown -R $user /src; exit \$status"
  rm -f "dotnet/hello-net$sdk.zip"
  (cd "$work/out" && zip -q -X "$OLDPWD/dotnet/hello-net$sdk.zip" Hello.dll Hello.deps.json Hello.runtimeconfig.json Amazon.Lambda.Core.dll)
  rm -rf "$work"
done
