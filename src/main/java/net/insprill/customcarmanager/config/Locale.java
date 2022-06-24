package net.insprill.customcarmanager.config;

import java.io.IOException;
import java.io.InputStream;
import java.util.Properties;

public class Locale {

    private static final String DEFAULT_LOCALE = "en-us";

    private static Properties properties;

    /**
     * Loads the selected Locale, or the default if the selected couldn't be found.
     *
     * @throws IOException If an IO errors occurs.
     */
    public static void init() throws IOException {
        properties = new Properties();
        if (!tryLoad(Config.getString("locale"))) {
            System.err.println("Locale '" + Config.getString("locale") + "' not found, using default.");
            tryLoad(DEFAULT_LOCALE);
        }
    }

    /**
     * Attempts to load a internal locale file.
     *
     * @param locale The path to the internal file/
     * @return True if the locale was found and loaded, false otherwise.
     * @throws IOException If an IO error occurs.
     */
    private static boolean tryLoad(String locale) throws IOException {
        try (InputStream is = Locale.class.getClassLoader().getResourceAsStream("locale/" + locale + ".properties")) {
            if (is == null)
                return false;
            properties.load(is);
            return true;
        }
    }

    /**
     * Gets a line from the selected locale file.
     *
     * @param key The line to get.
     * @return The line.
     */
    public static String getLine(String key) {
        return properties.getProperty(key);
    }

    private Locale() {
        throw new IllegalStateException("Utility class");
    }

}
