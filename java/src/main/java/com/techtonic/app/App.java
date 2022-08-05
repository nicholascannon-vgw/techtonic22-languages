package com.techtonic.app;

import static spark.Spark.*;
import com.google.gson.Gson;
import com.google.gson.GsonBuilder;

public class App {
    private static final Gson GSON = new GsonBuilder().create();

    public static void main( String[] args ) {
        port(8000);

        get("/healthcheck", (req, res) -> {
            res.type("application/json");
            return GSON.toJson(new MyMessage("healthy"));
        });

        System.out.println("Server running on port 8000...");
    }
}

class MyMessage {
    String message;

    public MyMessage(String message) {
        this.message = message;
    }
}