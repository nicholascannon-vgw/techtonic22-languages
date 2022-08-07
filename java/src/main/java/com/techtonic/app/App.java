package com.techtonic.app;

import static spark.Spark.*;
import com.google.gson.Gson;
import com.google.gson.GsonBuilder;

public class App {
    private static final Gson GSON = new GsonBuilder().create();

    public static void main( String[] args ) {
        get("/healthcheck", (req, res) -> {
            res.type("application/json");
            return GSON.toJson(new MessageResponse("healthy"));
        });

        System.out.println("Server running on port 8000...");
        port(8000);
    }
}

class MessageResponse {
    String message;

    public MessageResponse(String message) {
        this.message = message;
    }
}
