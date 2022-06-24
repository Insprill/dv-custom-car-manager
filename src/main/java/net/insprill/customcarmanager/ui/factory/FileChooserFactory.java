package net.insprill.customcarmanager.ui.factory;

import javafx.stage.FileChooser;
import net.insprill.customcarmanager.ui.Window;

import java.io.File;

public class FileChooserFactory {

    /**
     * Shows a new {@link FileChooser} with the specified title and {@link FileChooser.ExtensionFilter}.
     *
     * @param title  The title of the file chooser.
     * @param filter The {@link FileChooser.ExtensionFilter} used.
     * @return The selected file, or null if the window was closed/ the cancel button was pressed.
     */
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
