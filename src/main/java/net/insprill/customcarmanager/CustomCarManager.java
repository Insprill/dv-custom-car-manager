package net.insprill.customcarmanager;

import net.insprill.customcarmanager.locale.Locale;
import net.insprill.customcarmanager.ui.Window;

import java.io.IOException;

public class CustomCarManager {

    private static Locale locale;

    public static void main(String[] args) {
        try {
            locale = new Locale("en-us");
        } catch (IOException e) {
            e.printStackTrace(); //todo
        }
        Window.launch(Window.class, args);
    }

    public static Locale getLocale() {
        return CustomCarManager.locale;
    }

}
