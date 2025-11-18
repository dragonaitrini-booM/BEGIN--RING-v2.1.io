// build.gradle.kts
val ktorVersion = "2.3.11"
val koinVersion = "3.6.0"
val kotlinVersion = "1.9.24"
val logbackVersion = "1.5.6"

plugins {
    kotlin("jvm") version kotlinVersion
    id("io.ktor.plugin") version ktorVersion
    kotlin("plugin.serialization") version kotlinVersion
    application
}

group = "com.phiring.dashboard"
version = "1.0-SNAPSHOT"

application {
    // Defines the main class where Ktor will start
    mainClass.set("com.phiring.dashboard.ApplicationKt")
}

repositories {
    mavenCentral()
}

dependencies {
    // === Ktor Server ===
    implementation("io.ktor:ktor-server-core-jvm:$ktorVersion")
    implementation("io.ktor:ktor-server-netty-jvm:$ktorVersion")
    implementation("io.ktor:ktor-server-content-negotiation-jvm:$ktorVersion")
    implementation("io.ktor:ktor-server-call-logging-jvm:$ktorVersion")
    implementation("io.ktor:ktor-server-status-pages-jvm:$ktorVersion")
    implementation("io.ktor:ktor-serialization-kotlinx-json-jvm:$ktorVersion")

    // === Ktor Client (for API calls) ===
    implementation("io.ktor:ktor-client-core:$ktorVersion")
    implementation("io.ktor:ktor-client-cio:$ktorVersion")
    implementation("io.ktor:ktor-client-content-negotiation:$ktorVersion")
    // Note: ktor-serialization-kotlinx-json covers both server and client

    // === Koin DI ===
    implementation("io.insert-koin:koin-ktor:$koinVersion")
    implementation("io.insert-koin:koin-logger-slf4j:$koinVersion")

    // === Logging ===
    implementation("io.github.microutils:kotlin-logging-jvm:3.0.5")
    implementation("ch.qos.logback:logback-classic:$logbackVersion")

    // === Testing ===
    testImplementation("org.jetbrains.kotlin:kotlin-test")
    testImplementation("io.ktor:ktor-server-tests-jvm:$ktorVersion")
}

tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
    kotlinOptions.jvmTarget = "17" // Use a modern LTS JVM target
}
