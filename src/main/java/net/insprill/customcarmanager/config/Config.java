package net.insprill.customcarmanager.config;

import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.util.Properties;

public class Config {

    private static File configFolder;
    private static File configFile;

    private static Properties configProps;

    public static void init() throws IOException {
        configFolder = new File(System.getProperty("user.home") + File.separator + ".customcarmanager");
        configProps = new Properties();

        configFile = new File(configFolder, "config.properties");

        try (InputStream is = Config.class.getClassLoader().getResourceAsStream("config.properties")) {
            if (!configFile.exists()) {
                configFolder.mkdirs();
                configFile.createNewFile();
                configProps.load(is);
            } else {
                loadFromDisk();
            }
            save();
        }
    }

    private static void loadFromDisk() throws IOException {
        try (BufferedReader reader = Files.newBufferedReader(configFile.toPath())) {
            String line;
            while ((line = reader.readLine()) != null) {
                if (line.startsWith("#"))
                    continue;
                String[] split = line.split("=");
                configProps.setProperty(split[0], split[1]);
            }
        }
    }

    public static String getString(String key) {
        return configProps.getProperty(key);
    }

    public static void setString(String key, String value) {
        configProps.setProperty(key, value);
    }

    public static void save() throws IOException {
        configProps.store(Files.newOutputStream(configFile.toPath()), null);
    }

    private Config() {
        throw new IllegalStateException("Utility class");
    }

}
