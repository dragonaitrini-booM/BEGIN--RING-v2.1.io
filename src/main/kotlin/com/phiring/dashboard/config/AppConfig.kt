// src/main/kotlin/com/phiring/dashboard/di/Modules.kt
package com.phiring.dashboard.di

import com.phiring.dashboard.config.AppConfig
import io.ktor.client.*
import io.ktor.client.engine.cio.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.config.ApplicationConfig
import io.ktor.server.application.*
import org.koin.dsl.module
import org.koin.core.qualifier.named

val appModule = module {
    // HttpClient: Created as a singleton, correctly installed with ContentNegotiation,
    // and its resources will be cleaned up automatically by Koin/Ktor on shutdown.
    single {
        HttpClient(CIO) {
            install(ContentNegotiation) { json() }
            // Ensure requests only succeed on 2xx responses (Ktor best practice)
            expectSuccess = true 
        }
    }

    // AppConfig: Safely constructed by pulling values from Ktor's ApplicationConfig,
    // ensuring no mock secrets or System.getenv() calls are in the main logic.
    single<AppConfig> {
        // We get the Application object's config from Koin's environment
        val cfg = get<ApplicationConfig>()
        AppConfig(
            cloudflareApiToken = cfg.property("secrets.cloudflare_token").getString(),
            githubPat = cfg.property("secrets.github_pat").getString(),
            gasServiceAccountKey = cfg.property("secrets.gas_key").getString(),
            supabaseAnonKey = cfg.property("secrets.supabase_anon_key").getString(),
            supabaseServiceRoleKey = cfg.property("secrets.supabase_service_role_key").getString(),
            supabaseUrl = cfg.property("secrets.supabase_url").getString(),
            // Property access can be simplified for Integers/other types too, but getString().toInt() is explicit
            sssShareCount = cfg.property("secrets.sss_share_count").getString().toInt()
        )
    }
}
