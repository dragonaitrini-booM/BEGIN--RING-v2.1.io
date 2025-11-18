get("/api/drive") {
    // 1. Check for the API Key from the terminal environment variable
    val apiKey = System.getenv("DRIVE_API_KEY") 
        ?: return@get call.respondText("NO_KEY", status = HttpStatusCode.Unauthorized)

    try {
        // 2. Make the request to the Google Drive 'about' endpoint
        val resp = client.get("https://www.googleapis.com/drive/v3/about") {
            parameter("fields", "storageQuota") // Request only the quota field
            parameter("key", apiKey)            // Pass the API key
        }

        // 3. Handle API errors
        if (resp.status.value !in 200..299) {
            call.respondText("API_ERROR")
            return@get
        }

        // 4. Parse the JSON response
        val quota = resp.body<JsonObject>()["storageQuota"]!!.jsonObject
        val usedBytes  = quota["usage"]!!.jsonPrimitive.long
        val limitBytes = quota["limit"]!!.jsonPrimitive.long

        // 5. Convert bytes to Gigabytes for the frontend
        val usedGB  = usedBytes / 1_000_000_000.0
        val limitGB = limitBytes / 1_000_000_000.0   

        // 6. Send the formatted string back to the frontend
        call.respondText("%.1f GB / %.0f GB".format(usedGB, limitGB))
    } catch (e: Exception) {
        // 7. Handle network or parsing exceptions
        call.respondText("OFFLINE")
    }
}get("/api/drive") {
    // 1. Check for the API Key from the terminal environment variable
    val apiKey = System.getenv("DRIVE_API_KEY") 
        ?: return@get call.respondText("NO_KEY", status = HttpStatusCode.Unauthorized)

    try {
        // 2. Make the request to the Google Drive 'about' endpoint
        val resp = client.get("https://www.googleapis.com/drive/v3/about") {
            parameter("fields", "storageQuota") // Request only the quota field
            parameter("key", apiKey)            // Pass the API key
        }

        // 3. Handle API errors
        if (resp.status.value !in 200..299) {
            call.respondText("API_ERROR")
            return@get
        }

        // 4. Parse the JSON response
        val quota = resp.body<JsonObject>()["storageQuota"]!!.jsonObject
        val usedBytes  = quota["usage"]!!.jsonPrimitive.long
        val limitBytes = quota["limit"]!!.jsonPrimitive.long

        // 5. Convert bytes to Gigabytes for the frontend
        val usedGB  = usedBytes / 1_000_000_000.0
        val limitGB = limitBytes / 1_000_000_000.0   

        // 6. Send the formatted string back to the frontend
        call.respondText("%.1f GB / %.0f GB".format(usedGB, limitGB))
    } catch (e: Exception) {
        // 7. Handle network or parsing exceptions
        call.respondText("OFFLINE")
    }
}
