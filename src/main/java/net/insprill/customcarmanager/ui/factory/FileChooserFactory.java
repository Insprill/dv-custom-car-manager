package net.insprill.customcarmanager.ui.factory;

import javafx.stage.FileChooser;
import net.insprill.customcarmanager.ui.Window;

import java.io.File;

public class FileChooserFactory {

    public static File newDialog(String title, FileChooser.ExtensionFilter filter) {
        FileChooser fileChooser = new FileChooser();

        fileChooser.setTitle(title);
        fileChooser.getExtensionFilters().add(filter);
        fileChooser.setSelectedExtensionFilter(filter);

        return fileChooser.showOpenDialog(Window.getInstance().getPrimaryStage());
    }

    private FileChooserFactory() {
        throw new IllegalStateException("Utility class");
    }

}
