package net.insprill.customcarmanager;

import net.insprill.customcarmanager.config.Config;
import net.insprill.customcarmanager.config.Locale;
import net.insprill.customcarmanager.ui.Window;

import java.io.IOException;

public class CustomCarManager {

    public static void main(String[] args) throws IOException {
        Config.init();
        Locale.init();

        Window.launch(Window.class, args);

        Runtime.getRuntime().addShutdownHook(new Thread(() -> {
            try {
                Config.save();
            } catch (IOException e) {
                e.printStackTrace();
            }
        }));
    }

}
