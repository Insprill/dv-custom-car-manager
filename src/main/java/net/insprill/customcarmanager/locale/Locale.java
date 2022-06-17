package net.insprill.customcarmanager.locale;

import java.io.IOException;
import java.util.Properties;

public class Locale {

    private final Properties properties;

    public Locale(String locale) throws IOException {
        this.properties = new Properties();
        this.properties.load(getClass().getClassLoader().getResourceAsStream("locale/" + locale + ".properties"));
    }

    public String getLine(String key) {
        return this.properties.getProperty(key);
    }

}
