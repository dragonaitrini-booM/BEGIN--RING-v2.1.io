# ----------------------------------------------------------------------
# 1. BUILD STAGE: Compile the Kotlin/Ktor application using Gradle
# ----------------------------------------------------------------------
FROM gradle:8.6.0-jdk17 AS build
LABEL stage=build

# Set the working directory inside the container
WORKDIR /app

# Copy the build configuration files first (for efficient caching)
COPY build.gradle.kts settings.gradle.kts /app/
# Copy the source code
COPY src /app/src

# Generate the distribution package using the installDist task
# This downloads dependencies and compiles the application into a runnable archive.
RUN gradle installDist --no-daemon

# ----------------------------------------------------------------------
# 2. RUNTIME STAGE: Create the final, lightweight image
# ----------------------------------------------------------------------
# Use a minimal, secure JDK runtime image (Alpine variant is very small)
FROM eclipse-temurin:17-jre-alpine AS runtime
LABEL stage=runtime
LABEL maintainer="Phiring Dashboard Developer"

# Set the working directory for the application
WORKDIR /app

# Copy the compiled application distribution from the build stage
# The 'install' directory contains the runnable bin/app script and all JARs
COPY --from=build /app/build/install/phiring-dashboard /app

# Expose the port Ktor is configured to listen on (8080)
EXPOSE 8080

# The default command to run the Ktor application
# 'bin/app' is the executable script created by the installDist task
CMD ["bin/phiring-dashboard/bin/app"]
