package net.insprill.customcarmanager.config;

import java.io.IOException;
import java.io.InputStream;
import java.util.Properties;

public class Locale {

    private static final String DEFAULT_LOCALE = "en-us";

    private static Properties properties;

    public static void init() throws IOException {
        properties = new Properties();
        if (!tryLoad(Config.getString("locale"))) {
            tryLoad(DEFAULT_LOCALE);
        }
    }

    private static boolean tryLoad(String locale) throws IOException {
        try (InputStream is = Locale.class.getClassLoader().getResourceAsStream("locale/" + locale + ".properties")) {
            if (is == null)
                return false;
            properties.load(is);
            return true;
        }
    }

    public static String getLine(String key) {
        return properties.getProperty(key);
    }

    private Locale() {
        throw new IllegalStateException("Utility class");
    }

}
