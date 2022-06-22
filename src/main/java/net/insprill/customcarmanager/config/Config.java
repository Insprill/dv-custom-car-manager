package net.insprill.customcarmanager.config;

import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import net.insprill.customcarmanager.ui.dialog.ErrorDialog;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.StandardOpenOption;

public class Config {

    private static File configFolder;
    private static File configFile;

    private static JsonObject json;

    public static void init() throws IOException {
        configFolder = new File(System.getProperty("user.home") + File.separator + ".customcarmanager");
        json = new JsonObject();

        configFile = new File(configFolder, "config.json");

        try (InputStream is = Config.class.getClassLoader().getResourceAsStream("config.json")) {
            if (!configFile.exists()) {
                configFolder.mkdirs();
                json = JsonParser.parseString(new String(is.readAllBytes())).getAsJsonObject();
                save();
            } else {
                loadFromDisk();
            }
        }
    }

    private static void loadFromDisk() throws IOException {
        try (InputStream is = Files.newInputStream(configFile.toPath(), StandardOpenOption.READ)) {
            json = JsonParser.parseString(new String(is.readAllBytes())).getAsJsonObject();
        }
    }

    public static String getString(String key) {
        if (json.has(key)) {
            return json.get(key).getAsString();
        }
        return null;
    }

    public static void setString(String key, String value) {
        json.addProperty(key, value);
        try {
            save();
        } catch (IOException e) {
            new ErrorDialog(Locale.getLine("dialog.error.config-save-failed").formatted(configFile.getAbsolutePath()), e);
        }
    }

    public static void save() throws IOException {
        Files.writeString(configFile.toPath(), json.toString(), StandardOpenOption.CREATE, StandardOpenOption.WRITE, StandardOpenOption.TRUNCATE_EXISTING);
    }

    private Config() {
        throw new IllegalStateException("Utility class");
    }

}
