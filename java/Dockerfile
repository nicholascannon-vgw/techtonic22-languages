FROM maven:3.8.6-openjdk-11-slim AS build
WORKDIR /usr/src/app

# install deps + plugins and have them cached
COPY pom.xml .
RUN mvn clean package -Dmaven.main.skip -Dmaven.test.skip && rm -r target

# build our application (this step wont install deps again)
COPY src/ src/
RUN mvn clean package -Dmaven.test.skip

# amazon corretto is a prod dist of OpenJDK
FROM amazoncorretto:11-alpine AS runtime
WORKDIR /opt/app

COPY --from=build /usr/src/app/target/java-1.0-SNAPSHOT-jar-with-dependencies.jar .

CMD [ "java", "-jar", "./java-1.0-SNAPSHOT-jar-with-dependencies.jar" ]
