package net.insprill.customcarmanager.ui.factory;

import javafx.stage.FileChooser;
import javafx.stage.Stage;
import net.insprill.customcarmanager.config.Locale;

import java.io.File;

public class FileChooserFactory {

    public static File newOpenDialog(Stage stage, FileChooser.ExtensionFilter filter) {
        return newOpenDialog(stage, Locale.getLine("window.file-chooser.title"), filter);
    }

    public static File newOpenDialog(Stage stage, String title, FileChooser.ExtensionFilter filter) {
        FileChooser fileChooser = new FileChooser();

        fileChooser.setTitle(title);
        fileChooser.setSelectedExtensionFilter(filter);

        return fileChooser.showOpenDialog(stage);
    }

}
